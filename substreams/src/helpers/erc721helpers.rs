extern crate regex;
use crate::abi::erc721::functions;
use crate::pb::debbie::{Change, Erc721Deployment};
use evm_core::{ExitReason, Opcode};
use primitive_types::H256;
use substreams::scalar::BigInt;
use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Call, CallType, StorageChange};
use tiny_keccak::{Hasher, Keccak};

const SAFE_TRANSFER_FROM_FN_SIG: &str = "b88d4fde";
const NAME_FN_SIG: &str = "06fdde03";
const SYMBOL_FN_SIG: &str = "95d89b41";
const TOKENURI_FN_SIG: &str = "c87b56dd";
// to check for gas and delegatecall opcodes in sequence
//const GASDELEGATECALL: &str = "5af4";

fn contains_erc721_fns(code_string: &str) -> bool {
    code_string.contains(SAFE_TRANSFER_FROM_FN_SIG)
        && code_string.contains(NAME_FN_SIG)
        && code_string.contains(SYMBOL_FN_SIG)
        && code_string.contains(TOKENURI_FN_SIG)
}

// fn contains_delegate_call(code_str: &str) -> bool {
//     code_str.contains(GASDELEGATECALL)
// }

trait UsizeConversion {
    fn to_usize(&self) -> usize;
}

impl UsizeConversion for H256 {
    fn to_usize(&self) -> usize {
        self.to_low_u64_be().try_into().unwrap()
    }
}
enum ParentCallType<'a> {
    Normal(&'a Call),
    Delegate(&'a Call),
    None,
}

impl ParentCallType<'_> {
    pub fn new<'a>(calls: &'a Vec<Call>, current_call: &'a Call) -> ParentCallType<'a> {
        let parent_call_index = current_call.parent_index as usize;
        let parent_call = calls.get(parent_call_index);
        if let Some(parent_call) = parent_call {
            if parent_call.call_type() == CallType::Delegate {
                ParentCallType::Delegate(parent_call)
            } else {
                ParentCallType::Normal(parent_call)
            }
        } else {
            ParentCallType::None
        }
    }
}

struct StorageChanges(HashMap<H256, Vec<u8>>);
impl StorageChanges {
    pub fn new<'a>(changes: &'a Vec<&'a StorageChange>) -> Self {
        let storage_changes = changes
            .iter()
            .map(|s| (H256::from_slice(s.key.as_ref()), s.new_value.to_vec()))
            .collect();
        Self(storage_changes)
    }
}

pub struct ERC721Creation<'a> {
    pub address: Vec<u8>,
    pub code: Vec<u8>,
    pub storage_changes: Vec<&'a StorageChange>,
}

impl<'a> ERC721Creation<'a> {
    pub fn from_call(calls: &'a Vec<Call>) -> Option<Self> {
        for call in calls.iter() {
            if let Some(last_code_change) = call.code_changes.iter().last() {
                let code = &last_code_change.new_code;
                let address = &call.address;
                let code_string = Hex::encode(&code);

                if contains_erc721_fns(&code_string) {
                    substreams::log::info!("found functions");

                    match ParentCallType::new(&calls, call) {
                        ParentCallType::Normal(_parent_call) => {
                            substreams::log::info!("Normal parent calltype");
                            let storage_ref: Vec<&StorageChange> =
                                call.storage_changes.iter().collect();
                            return Some(Self {
                                address: address.to_vec(),
                                code: code.to_vec(),
                                storage_changes: storage_ref,
                            });
                        }

                        ParentCallType::Delegate(parent_call) => {
                            let storage_ref: Vec<&StorageChange> =
                                parent_call.storage_changes.iter().collect();
                            return Some(Self {
                                address: address.to_vec(),
                                code: code.to_vec(),
                                storage_changes: storage_ref,
                            });
                        }

                        ParentCallType::None => {
                            log::info!("no proxy found{:?}", address);
                            let storage_ref: Vec<&StorageChange> =
                                call.storage_changes.iter().collect();
                            return Some(Self {
                                address: address.to_vec(),
                                code: code.to_vec(),
                                storage_changes: storage_ref,
                            });
                        }
                    };
                }
            }
        }
        None
    }
}

pub fn process_erc721_contract(
    contract_creation: ERC721Creation,
    clock: Clock,
) -> Option<Erc721Deployment> {
    let changes: &Vec<Change> = &contract_creation
        .storage_changes
        .iter()
        .map(|storage_change| Change {
            key: storage_change.key.clone(),
            new_value: storage_change.new_value.clone(),
        })
        .collect();

    let mut contract = Erc721Deployment {
        address: Hex::encode(&contract_creation.address),
        name: String::new(),
        symbol: String::new(),
        blocknumber: clock.number.to_string(),
        token_uri: String::new(),
        timestamp_seconds: clock.timestamp.unwrap().seconds,
        code: contract_creation.code.clone(),
        storage_changes: changes.to_vec(),
    };

    let code = Rc::new(contract_creation.code);

    let log_name = "name";
    let log_symbol = "symbol";

    // Name
    match execute_on(
        Hex::encode(&contract_creation.address),
        code.clone(),
        functions::Name {}.encode(),
        &contract_creation.storage_changes,
        log_name,
    ) {
        Ok(return_value) => match functions::Name::output(return_value.as_ref()) {
            Ok(x) => {
                contract.name = x;
            }
            Err(e) => {
                log::info!("Unable to decode output for name: {}", e);
            }
        },
        Err(e) => {
            log::info!("Error: {}", e);
        }
    }

    // Symbol
    match execute_on(
        Hex::encode(&contract_creation.address),
        code.clone(),
        functions::Symbol {}.encode(),
        &contract_creation.storage_changes,
        log_symbol,
    ) {
        Ok(return_value) => match functions::Symbol::output(return_value.as_ref()) {
            Ok(x) => {
                contract.symbol = x;
            }
            Err(e) => {
                log::info!("Unable to decode output for symbol: {}", e);
            }
        },
        Err(e) => {
            log::info!("Error: {}", e);
        }
    }

    Some(contract)
}

pub fn get_token_uri(contract: &Erc721Deployment, token_id: &str) -> Result<String, anyhow::Error> {
    let function_log = String::from("tokenuri");
    
    let valids = evm_core::Valids::new(&contract.code);
    let mut jump_dest = 0;
    for i in 0..valids.len() {
        if valids.is_valid(i) {
            jump_dest += 1;
        }
    }

    let contract_storage: HashMap<H256, Vec<u8>> = contract.storage_changes
    .iter()
    .map(|s| (H256::from_slice(s.key.as_ref()), s.new_value.to_vec()))
    .collect();

    let contract_code = contract.code.clone();

    let code = Rc::new(contract_code);

    log::info!(
        "\n\n\n\n\nERC721: Trying function {:?} for token: {:?}, of collection: {:?}\n{} valid jump destinations)\ncode len {}",
        function_log,
        token_id,
        contract.name,
        jump_dest,
        code.len(),
    );
    
    let token_id_bigint = BigInt::from_str(token_id).unwrap();
    log::info!("converted tokenid {}", &token_id_bigint);
    let data = functions::TokenUri {token_id: token_id_bigint}.encode();


    let mut machine = evm_core::Machine::new(
        code,
        Rc::new(data), // name()
        // Rc::new(vec![0x5c, 0x97, 0x5a, 0xbb]), // paused()
        1024,
        1024,
    );

    loop {
        let mut active_opcode: Option<Opcode> = None;
        if let Some((opcode, stack)) = machine.inspect() {
            // log::info!(
            //     "Machine active opcode is {}",
            //     // display_opcode_input(opcode, stack),
            // );

            active_opcode = Some(opcode)
        }

        match machine.step() {
            Ok(()) => {
                if let Some(opcode) = active_opcode {
                    if let Some(_output) = display_opcode_output(opcode, machine.stack()) {
                        // log::info!("Machine executed opcode {}", output);
                    }
                }
            }
            Err(res) => {
                match res {
                    evm_core::Capture::Exit(ExitReason::Succeed(reason)) => {
                        match reason {
                            evm_core::ExitSucceed::Stopped => {
                                log::info!("EVM stopped gracefully");
                            }
                            evm_core::ExitSucceed::Returned => {
                                let return_value = machine.return_value();
                                log::info!("EVM returned gracefully {}", Hex(&return_value));
                                                               
                                match functions::TokenUri::output(&return_value) {
                                    Ok(return_value) => {
                                         return Ok(return_value);
                                    }
                                    Err(e) => {
                                        log::info!("failed to decode output for tokenuri");
                                    }
                                }
                                
                            }
                            evm_core::ExitSucceed::Suicided => {
                                log::info!("EVM suicided gracefully");
                            }
                        }

                        return Ok(String::new());
                    }
                    evm_core::Capture::Exit(out) => {
                        return Err(anyhow::anyhow!("Capture exit: {:?}", out));
                    }
                    evm_core::Capture::Trap(opcode) => {
                        match opcode.0 {
                            // CALLVALUE
                            0x34 => {
                                machine.stack_mut().push(H256::zero()).unwrap();
                            }

                            // SHA3
                            0x20 => {
                                let offset = machine.stack_mut().pop().unwrap();
                                let data_length = machine.stack_mut().pop().unwrap();
                                let data_memory = machine
                                    .memory_mut()
                                    .get(offset.to_usize(), data_length.to_usize());
                                let mut hash = Keccak::v256();
                                let mut output = [0u8; 32];
                                hash.update(&data_memory);
                                hash.finalize(&mut output);
                                machine.stack_mut().push(H256::from_slice(&output)).unwrap();

                                log::info!("SHA3 HANDLED \n");
                            }

                            // SLOAD
                            0x54 => {
                                let key = machine.stack_mut().pop().unwrap();
                                log::info!("storage key {:?}", key);

                                if let Some(value) = contract_storage.get(&key) {
                                    machine.stack_mut().push(H256::from_slice(value)).unwrap();
                                    log::info!("SLOAD HANDLED \n")
                                } else {
                                    return Err(anyhow::anyhow!(
                                        "SLOAD unknown storage key {:x}",
                                        key
                                    ));
                                }
                            }

                            _ => {
                                return Err(anyhow::anyhow!(
                                    "ERC721: Capture trap unhandled: {:?}",
                                    opcode_to_string(opcode)
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn execute_on<'a>(
    address: String,
    code: Rc<Vec<u8>>,
    data: Vec<u8>,
    storage_changes: &'a Vec<&'a StorageChange>,
    function_log: &str,
) -> Result<Vec<u8>, anyhow::Error> {
    let valids = evm_core::Valids::new(&code);
    let mut jump_dest = 0;
    for i in 0..valids.len() {
        if valids.is_valid(i) {
            jump_dest += 1;
        }
    }

    let contract_storage = StorageChanges::new(storage_changes);

    log::info!(
        "\n\n\n\n\nERC721: Trying function {:?} for contract: {:?}\n{} valid jump destinations\ncode len {})",
        function_log,
        address,
        jump_dest,
        code.len(),
    );

    let mut machine = evm_core::Machine::new(
        code,
        Rc::new(data), // name()
        // Rc::new(vec![0x5c, 0x97, 0x5a, 0xbb]), // paused()
        1024,
        1024,
    );

    loop {
        let mut active_opcode: Option<Opcode> = None;
        if let Some((opcode, stack)) = machine.inspect() {
            log::info!(
                "Machine active opcode is {}",
                display_opcode_input(opcode, stack),
            );

            active_opcode = Some(opcode)
        }

        match machine.step() {
            Ok(()) => {
                if let Some(opcode) = active_opcode {
                    if let Some(_output) = display_opcode_output(opcode, machine.stack()) {
                        // log::info!("Machine executed opcode {}", output);
                    }
                }
            }
            Err(res) => {
                match res {
                    evm_core::Capture::Exit(ExitReason::Succeed(reason)) => {
                        match reason {
                            evm_core::ExitSucceed::Stopped => {
                                log::info!("EVM stopped gracefully");
                            }
                            evm_core::ExitSucceed::Returned => {
                                let return_value = machine.return_value();
                                log::info!("EVM returned gracefully {}", Hex(&return_value));

                                return Ok(return_value);
                            }
                            evm_core::ExitSucceed::Suicided => {
                                log::info!("EVM suicided gracefully");
                            }
                        }

                        return Ok(vec![]);
                    }
                    evm_core::Capture::Exit(out) => {
                        return Err(anyhow::anyhow!("Capture exit: {:?}", out));
                    }
                    evm_core::Capture::Trap(opcode) => {
                        match opcode.0 {
                            // CALLVALUE
                            0x34 => {
                                machine.stack_mut().push(H256::zero()).unwrap();
                            }

                            // SHA3
                            0x20 => {
                                let offset = machine.stack_mut().pop().unwrap();
                                let data_length = machine.stack_mut().pop().unwrap();
                                let data_memory = machine
                                    .memory_mut()
                                    .get(offset.to_usize(), data_length.to_usize());
                                let mut hash = Keccak::v256();
                                let mut output = [0u8; 32];
                                hash.update(&data_memory);
                                hash.finalize(&mut output);
                                machine.stack_mut().push(H256::from_slice(&output)).unwrap();

                                log::info!("SHA3 HANDLED \n");
                            }

                            // SLOAD
                            0x54 => {
                                let key = machine.stack_mut().pop().unwrap();
                                log::info!("storage key {:?}", key);

                                if let Some(value) = contract_storage.0.get(&key) {
                                    machine.stack_mut().push(H256::from_slice(value)).unwrap();
                                    log::info!("SLOAD HANDLED \n")
                                } else {
                                    return Err(anyhow::anyhow!(
                                        "SLOAD unknown storage key {:x}",
                                        key
                                    ));
                                }
                            }

                            _ => {
                                return Err(anyhow::anyhow!(
                                    "ERC721: Capture trap unhandled: {:?}",
                                    opcode_to_string(opcode)
                                ));
                            }
                        }
                    }
                };
            }
        }
    }
}

fn display_opcode_input(opcode: evm_core::Opcode, stack: &evm_core::Stack) -> String {
    match opcode.0 {
        0x10 => display_opcode_with_stack("LT", stack, 2),
        0x11 => display_opcode_with_stack("GT", stack, 2),
        0x12 => display_opcode_with_stack("SLT", stack, 2),
        0x13 => display_opcode_with_stack("SGT", stack, 2),
        0x14 => display_opcode_with_stack("EQ", stack, 2),

        0x35 => display_opcode_with_stack("CALLDATALOAD", stack, 1),
        0x56 => display_opcode_with_stack("JUMP", stack, 1),
        0x57 => display_opcode_with_stack("JUMPI", stack, 2),

        _ => opcode_to_string(opcode),
    }
}

fn display_opcode_output(opcode: evm_core::Opcode, stack: &evm_core::Stack) -> Option<String> {
    match opcode.0 {
        0x35 => Some(display_opcode_with_stack("CALLDATALOAD", stack, 1)),
        0x36 => Some(display_opcode_with_stack("CALLDATASIZE", stack, 1)),
        _ => None,
    }
}

fn display_opcode_with_stack(name: &str, stack: &evm_core::Stack, count: usize) -> String {
    let mut stack_items: Vec<String> = Vec::new();
    for i in 0..count {
        let value = match stack.peek(i) {
            Ok(value) => format!("{:x}", value).trim_start_matches('0').to_string(),
            Err(_) => {
                return format!(
                    "{} {} <INVALID STACK AT {}>",
                    name,
                    stack_items.join(" "),
                    i
                )
            }
        };

        stack_items.push(value)
    }

    format!("{} {}", name, stack_items.join(" "))
}

fn opcode_to_string(opcode: evm_core::Opcode) -> String {
    match opcode.0 {
        0x00 => "STOP: Halts execution".to_string(),
        0x01 => "ADD: Adds top two stack items".to_string(),
        0x02 => "MUL: Multiplies top two stack items".to_string(),
        0x03 => "SUB: Subtracts top two stack items".to_string(),
        0x04 => "DIV: Divides top two stack items, integer division".to_string(),
        0x05 => "SDIV: Signed division of top two stack items".to_string(),
        0x06 => "MOD: Modulus of top two stack items".to_string(),
        0x07 => "SMOD: Signed modulus of top two stack items".to_string(),
        0x08 => "ADDMOD: Modular addition of top three stack items".to_string(),
        0x09 => "MULMOD: Modular multiplication of top three stack items".to_string(),
        0x0a => "EXP: Exponential operation of top two stack items".to_string(),
        0x0b => "SIGNEXTEND: Extend length of two's complement signed integer".to_string(),
        0x10 => "LT: Less than comparison of top two stack items".to_string(),
        0x11 => "GT: Greater than comparison of top two stack items".to_string(),
        0x12 => "SLT: Signed less than comparison of top two stack items".to_string(),
        0x13 => "SGT: Signed greater than comparison of top two stack items".to_string(),
        0x14 => "EQ: Equality comparison of top two stack items".to_string(),
        0x15 => "ISZERO: Checks if top stack item is zero".to_string(),
        0x16 => "AND: Bitwise AND of top two stack items".to_string(),
        0x17 => "OR: Bitwise OR of top two stack items".to_string(),
        0x18 => "XOR: Bitwise XOR of top two stack items".to_string(),
        0x19 => "NOT: Bitwise NOT of the top stack item".to_string(),
        0x1a => "BYTE: Extracts byte at position from the top stack item".to_string(),
        0x35 => "CALLDATALOAD: Loads input data in current environment to stack".to_string(),
        0x36 => "CALLDATASIZE: Gets size of input data in current environment".to_string(),
        0x37 => "CALLDATACOPY: Copies input data in current environment to memory".to_string(),
        0x38 => "CODESIZE: Gets the size of the code running in the current environment".to_string(),
        0x39 => "CODECOPY: Copies the code running in the current environment to memory".to_string(),
        0x1b => "SHL: Shifts left the second top stack item by the top stack item".to_string(),
        0x1c => "SHR: Shifts right the second top stack item by the top stack item, without preserving sign".to_string(),
        0x1d => "SAR: Shifts right the second top stack item by the top stack item, preserving sign".to_string(),
        0x50 => "POP: Removes the top item from the stack".to_string(),
        0x51 => "MLOAD: Loads word from memory to the stack".to_string(),
        0x52 => "MSTORE: Saves a word to memory from the stack".to_string(),
        0x53 => "MSTORE8: Saves a single byte to memory from the stack".to_string(),
        0x56 => "JUMP: Alters program counter to the address specified in the top stack item".to_string(),
        0x57 => "JUMPI: Conditionally alters program counter if the second top stack item is not zero".to_string(),
        0x58 => "PC: Pushes the program counter to the stack".to_string(),
        0x59 => "MSIZE: Pushes the size of active memory in bytes to the stack".to_string(),
        0x5b => "JUMPDEST: Marks a valid destination for jumps".to_string(),
        0x5f => "PUSH0: Pushes 0 bytes onto the stack (not a standard opcode)".to_string(),
        0x60 => "PUSH1: Pushes 1 byte onto the stack".to_string(),
        0x61 => "PUSH2: Pushes 2 bytes onto the stack".to_string(),
        0x62 => "PUSH3: Pushes 3 bytes onto the stack".to_string(),
        0x63 => "PUSH4: Pushes 4 bytes onto the stack".to_string(),
        0x64 => "PUSH5: Pushes 5 bytes onto the stack".to_string(),
        0x65 => "PUSH6: Pushes 6 bytes onto the stack".to_string(),
        0x66 => "PUSH7: Pushes 7 bytes onto the stack".to_string(),
        0x67 => "PUSH8: Pushes 8 bytes onto the stack".to_string(),
        0x68 => "PUSH9: Pushes 9 bytes onto the stack".to_string(),
        0x69 => "PUSH10: Pushes 10 bytes onto the stack".to_string(),
        0x6a => "PUSH11: Pushes 11 bytes onto the stack".to_string(),
        0x6b => "PUSH12: Pushes 12 bytes onto the stack".to_string(),
        0x6c => "PUSH13: Pushes 13 bytes onto the stack".to_string(),
        0x6d => "PUSH14: Pushes 14 bytes onto the stack".to_string(),
        0x6e => "PUSH15: Pushes 15 bytes onto the stack".to_string(),
        0x6f => "PUSH16: Pushes 16 bytes onto the stack".to_string(),
        0x70 => "PUSH17: Pushes 17 bytes onto the stack".to_string(),
        0x71 => "PUSH18: Pushes 18 bytes onto the stack".to_string(),
        0x72 => "PUSH19: Pushes 19 bytes onto the stack".to_string(),
        0x73 => "PUSH20: Pushes 20 bytes onto the stack".to_string(),
        0x74 => "PUSH21: Pushes 21 bytes onto the stack".to_string(),
        0x75 => "PUSH22: Pushes 22 bytes onto the stack".to_string(),
        0x76 => "PUSH23: Pushes 23 bytes onto the stack".to_string(),
        0x77 => "PUSH24: Pushes 24 bytes onto the stack".to_string(),
        0x78 => "PUSH25: Pushes 25 bytes onto the stack".to_string(),
        0x79 => "PUSH26: Pushes 26 bytes onto the stack".to_string(),
        0x7a => "PUSH27: Pushes 27 bytes onto the stack".to_string(),
        0x7b => "PUSH28: Pushes 28 bytes onto the stack".to_string(),
        0x7c => "PUSH29: Pushes 29 bytes onto the stack".to_string(),
        0x7d => "PUSH30: Pushes 30 bytes onto the stack".to_string(),
        0x7e => "PUSH31: Pushes 31 bytes onto the stack".to_string(),
        0x7f => "PUSH32: Pushes 32 bytes onto the stack".to_string(),
        0x80 => "DUP1: Duplicates the top stack item".to_string(),
        0x81 => "DUP2: Duplicates the second item from the top of the stack".to_string(),
        0x82 => "DUP3: Duplicates the third item from the top of the stack".to_string(),
        0x83 => "DUP4: Duplicates the fourth item from the top of the stack".to_string(),
        0x84 => "DUP5: Duplicates the fifth item from the top of the stack".to_string(),
        0x85 => "DUP6: Duplicates the sixth item from the top of the stack".to_string(),
        0x86 => "DUP7: Duplicates the seventh item from the top of the stack".to_string(),
        0x87 => "DUP8: Duplicates the eighth item from the top of the stack".to_string(),
        0x88 => "DUP9: Duplicates the ninth item from the top of the stack".to_string(),
        0x89 => "DUP10: Duplicates the tenth item from the top of the stack".to_string(),
        0x8a => "DUP11: Duplicates the eleventh item from the top of the stack".to_string(),
        0x8b => "DUP12: Duplicates the twelfth item from the top of the stack".to_string(),
        0x8c => "DUP13: Duplicates the thirteenth item from the top of the stack".to_string(),
        0x8d => "DUP14: Duplicates the fourteenth item from the top of the stack".to_string(),
        0x8e => "DUP15: Duplicates the fifteenth item from the top of the stack".to_string(),
        0x8f => "DUP16: Duplicates the sixteenth item from the top of the stack".to_string(),
        0x90 => "SWAP1: Swaps the top stack item with the second item".to_string(),
        0x91 => "SWAP2: Swaps the top stack item with the third item".to_string(),
        0x92 => "SWAP3: Swaps the top stack item with the fourth item".to_string(),
        0x93 => "SWAP4: Swaps the top stack item with the fifth item".to_string(),
        0x94 => "SWAP5: Swaps the top stack item with the sixth item".to_string(),
        0x95 => "SWAP6: Swaps the top stack item with the seventh item".to_string(),
        0x96 => "SWAP7: Swaps the top stack item with the eighth item".to_string(),
        0x97 => "SWAP8: Swaps the top stack item with the ninth item".to_string(),
        0x98 => "SWAP9: Swaps the top stack item with the tenth item".to_string(),
        0x99 => "SWAP10: Swaps the top stack item with the eleventh item".to_string(),
        0x9a => "SWAP11: Swaps the top stack item with the twelfth item".to_string(),
        0x9b => "SWAP12: Swaps the top stack item with the thirteenth item".to_string(),
        0x9c => "SWAP13: Swaps the top stack item with the fourteenth item".to_string(),
        0x9d => "SWAP14: Swaps the top stack item with the fifteenth item".to_string(),
        0x9e => "SWAP15: Swaps the top stack item with the sixteenth item".to_string(),
        0x9f => "SWAP16: Swaps the top stack item with the seventeenth item".to_string(),
        0xf3 => "RETURN: Stops execution and returns data".to_string(),
        0xfd => "REVERT: Reverts state changes but returns data and consumes all gas".to_string(),
        0xfe => "INVALID: Designates an invalid instruction".to_string(),
        0xef => "EOFMAGIC: Marks the end of code in the EOF format".to_string(),
        0x20 => "SHA3: Computes Keccak-256 hash".to_string(),
        0x30 => "ADDRESS: Gets the address of the currently executing account".to_string(),
        0x31 => "BALANCE: Gets the balance of the given account".to_string(),
        0x47 => "SELFBALANCE: Gets the balance of the currently executing account".to_string(),
        0x48 => "BASEFEE: Gets the base fee of the current block".to_string(),
        0x32 => "ORIGIN: Gets the address that originated the transaction".to_string(),
        0x33 => "CALLER: Gets the address that called the current function".to_string(),
        0x34 => "CALLVALUE: Gets the value (in wei) sent with the call".to_string(),
        0x3a => "GASPRICE: Gets the gas price of the transaction".to_string(),
        0x3b => "EXTCODESIZE: Gets the size of an account's code".to_string(),
        0x3c => "EXTCODECOPY: Copies an account's code to memory".to_string(),
        0x3f => "EXTCODEHASH: Gets the hash of an account's code".to_string(),
        0x3d => "RETURNDATASIZE: Gets the size of the output data from the previous call".to_string(),
        0x3e => "RETURNDATACOPY: Copies the output data from the previous call to memory".to_string(),
        0x40 => "BLOCKHASH: Gets the hash of one of the 256 most recent complete blocks".to_string(),
        0x41 => "COINBASE: Gets the block's beneficiary address".to_string(),
        0x42 => "TIMESTAMP: Gets the block's timestamp".to_string(),
        0x43 => "NUMBER: Gets the current block number".to_string(),
        0x44 => "DIFFICULTY: Gets the difficulty of the current block".to_string(),
        0x45 => "GASLIMIT: Gets the gas limit of the current block".to_string(),
        0x54 => "SLOAD: Loads a word from storage".to_string(),
        0x55 => "SSTORE: Saves a word to storage".to_string(),
        0x5a => "GAS: Gets the remaining gas".to_string(),
        0xa0 => "LOG0: Appends log record with no topics".to_string(),
        0xa1 => "LOG1: Appends log record with one topic".to_string(),
        0xa2 => "LOG2: Appends log record with two topics".to_string(),
        0xa3 => "LOG3: Appends log record with three topics".to_string(),
        0xa4 => "LOG4: Appends log record with four topics".to_string(),
        0xf0 => "CREATE: Creates a new account with some Wei and initializes code".to_string(),
        0xf5 => "CREATE2: Creates a new account with some Wei and initializes code, with a salt".to_string(),
        0xf1 => "CALL: Calls another contract or sends Ether".to_string(),
        0xf2 => "CALLCODE: Calls another contract with this contract's storage".to_string(),
        0xf4 => "DELEGATECALL: Calls another contract with this contract's storage and code".to_string(),
        0xfa => "STATICCALL: Makes a call without allowing state modifications".to_string(),
        0xff => "SUICIDE (SELFDESTRUCT): Terminates the contract and sends remaining funds to a designated address".to_string(),
        0x46 => "CHAINID: Gets the chain ID of the current chain".to_string(),
        x => format!("UNKNOWN OPCODE: 0x{:x}", x),
    
        

    }
}
