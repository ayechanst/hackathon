use primitive_types::H256;
use std::collections::HashMap;
use substreams::Hex;
use std::rc::Rc;
use substreams::pb::substreams::Clock;
use evm_core::{ExitReason, Opcode};
use substreams::log;
use crate::pb::debbie::{Erc20Deployment, Erc20Transfer, MasterProto};
use crate::abi::erc20::functions;
use substreams_ethereum::pb::eth::v2::Call;

pub struct ERC20Creation {
    pub address: Vec<u8>,
    pub code: Vec<u8>,
    pub storage_changes: HashMap<H256, Vec<u8>>,
}

impl ERC20Creation {
    pub fn from_call(
        address: &Vec<u8>,
        code: Vec<u8>,
        storage_changes: HashMap<H256, Vec<u8>>,
    ) -> Option<Self> {
        let code_string = Hex::encode(&code);
        if code_string.contains("06fdde03")
            && code_string.contains("95d89b41")
            && code_string.contains("313ce567")
            && code_string.contains("18160ddd")
        {
            Some(Self {
                address: address.to_vec(),
                code,
                storage_changes,
            })
        } else {
            None
        }
    }
}

pub fn process_contract(contract_creation: ERC20Creation, clock: Clock) -> Option<Erc20Deployment> {
    let mut contract = Erc20Deployment {
        address: Hex::encode(&contract_creation.address),
        name: String::new(),
        symbol: String::new(),
        total_supply: String::new(),
        decimals: String::new(),
        blocknumber: clock.number.to_string()
    };
    let code = Rc::new(contract_creation.code);

    // Name
    match execute_on(
        Hex::encode(&contract_creation.address),
        code.clone(),
        functions::Name {}.encode(),
        &contract_creation.storage_changes,
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

    // Decimals
    match execute_on(
        Hex::encode(&contract_creation.address),
        code.clone(),
        functions::Decimals {}.encode(),
        &contract_creation.storage_changes,
    ) {
        Ok(return_value) => match functions::Decimals::output(return_value.as_ref()) {
            Ok(x) => {
                contract.decimals = x.to_string();
            }
            Err(e) => {
                log::info!("Unable to decode output for decimals: {}", e);
            }
        },
        Err(e) => {
            log::info!("Error: {}", e);
        }
    }

    //total supply
    match execute_on(
        Hex::encode(&contract_creation.address),
        code.clone(),
        functions::Decimals {}.encode(),
        &contract_creation.storage_changes,
    ) {
        Ok(return_value) => match functions::TotalSupply::output(return_value.as_ref()) {
            Ok(x) => {
                contract.total_supply = x.to_string();
            }
            Err(e) => {
                log::info!("Unable to decode output for total supply: {}", e);
            }
        },
        Err(e) => {
            log::info!("Error: {}", e);
        }
    }

    Some(contract)
}

fn execute_on(
    address: String,
    code: Rc<Vec<u8>>,
    data: Vec<u8>,
    storage_changes: &HashMap<H256, Vec<u8>>,
) -> Result<Vec<u8>, anyhow::Error> {
    let valids = evm_core::Valids::new(&code);
    let mut jump_dest = 0;
    for i in 0..valids.len() {
        if valids.is_valid(i) {
            jump_dest += 1;
        }
    }

    log::info!(
        "Trying contract: {:?} with {} valid jump destinations (code len {}))",
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
            // log::info!(
            //     "Machine active opcode is {}",
            //     display_opcode_input(opcode, stack),
            // );

            active_opcode = Some(opcode)
        }

        match machine.step() {
            Ok(()) => {
                if let Some(opcode) = active_opcode {
                    if let Some(output) = display_opcode_output(opcode, machine.stack()) {
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

                            // SLOAD
                            0x54 => {
                                let key = machine.stack_mut().pop().unwrap();

                                if let Some(value) = storage_changes.get(&key) {
                                    machine.stack_mut().push(H256::from_slice(value)).unwrap();
                                } else {
                                    return Err(anyhow::anyhow!(
                                        "SLOAD unknown storage key {:x}",
                                        key
                                    ));
                                }
                            }
                            _ => {
                                return Err(anyhow::anyhow!(
                                    "Capture trap unhandled: {:?}",
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
        0x00 => "STOP".to_string(),
        0x01 => "ADD".to_string(),
        0x02 => "MUL".to_string(),
        0x03 => "SUB".to_string(),
        0x04 => "DIV".to_string(),
        0x05 => "SDIV".to_string(),
        0x06 => "MOD".to_string(),
        0x07 => "SMOD".to_string(),
        0x08 => "ADDMOD".to_string(),
        0x09 => "MULMOD".to_string(),
        0x0a => "EXP".to_string(),
        0x0b => "SIGNEXTEND".to_string(),
        0x10 => "LT".to_string(),
        0x11 => "GT".to_string(),
        0x12 => "SLT".to_string(),
        0x13 => "SGT".to_string(),
        0x14 => "EQ".to_string(),
        0x15 => "ISZERO".to_string(),
        0x16 => "AND".to_string(),
        0x17 => "OR".to_string(),
        0x18 => "XOR".to_string(),
        0x19 => "NOT".to_string(),
        0x1a => "BYTE".to_string(),
        0x35 => "CALLDATALOAD".to_string(),
        0x36 => "CALLDATASIZE".to_string(),
        0x37 => "CALLDATACOPY".to_string(),
        0x38 => "CODESIZE".to_string(),
        0x39 => "CODECOPY".to_string(),
        0x1b => "SHL".to_string(),
        0x1c => "SHR".to_string(),
        0x1d => "SAR".to_string(),
        0x50 => "POP".to_string(),
        0x51 => "MLOAD".to_string(),
        0x52 => "MSTORE".to_string(),
        0x53 => "MSTORE8".to_string(),
        0x56 => "JUMP".to_string(),
        0x57 => "JUMPI".to_string(),
        0x58 => "PC".to_string(),
        0x59 => "MSIZE".to_string(),
        0x5b => "JUMPDEST".to_string(),
        0x5f => "PUSH0".to_string(),
        0x60 => "PUSH1".to_string(),
        0x61 => "PUSH2".to_string(),
        0x62 => "PUSH3".to_string(),
        0x63 => "PUSH4".to_string(),
        0x64 => "PUSH5".to_string(),
        0x65 => "PUSH6".to_string(),
        0x66 => "PUSH7".to_string(),
        0x67 => "PUSH8".to_string(),
        0x68 => "PUSH9".to_string(),
        0x69 => "PUSH10".to_string(),
        0x6a => "PUSH11".to_string(),
        0x6b => "PUSH12".to_string(),
        0x6c => "PUSH13".to_string(),
        0x6d => "PUSH14".to_string(),
        0x6e => "PUSH15".to_string(),
        0x6f => "PUSH16".to_string(),
        0x70 => "PUSH17".to_string(),
        0x71 => "PUSH18".to_string(),
        0x72 => "PUSH19".to_string(),
        0x73 => "PUSH20".to_string(),
        0x74 => "PUSH21".to_string(),
        0x75 => "PUSH22".to_string(),
        0x76 => "PUSH23".to_string(),
        0x77 => "PUSH24".to_string(),
        0x78 => "PUSH25".to_string(),
        0x79 => "PUSH26".to_string(),
        0x7a => "PUSH27".to_string(),
        0x7b => "PUSH28".to_string(),
        0x7c => "PUSH29".to_string(),
        0x7d => "PUSH30".to_string(),
        0x7e => "PUSH31".to_string(),
        0x7f => "PUSH32".to_string(),
        0x80 => "DUP1".to_string(),
        0x81 => "DUP2".to_string(),
        0x82 => "DUP3".to_string(),
        0x83 => "DUP4".to_string(),
        0x84 => "DUP5".to_string(),
        0x85 => "DUP6".to_string(),
        0x86 => "DUP7".to_string(),
        0x87 => "DUP8".to_string(),
        0x88 => "DUP9".to_string(),
        0x89 => "DUP10".to_string(),
        0x8a => "DUP11".to_string(),
        0x8b => "DUP12".to_string(),
        0x8c => "DUP13".to_string(),
        0x8d => "DUP14".to_string(),
        0x8e => "DUP15".to_string(),
        0x8f => "DUP16".to_string(),
        0x90 => "SWAP1".to_string(),
        0x91 => "SWAP2".to_string(),
        0x92 => "SWAP3".to_string(),
        0x93 => "SWAP4".to_string(),
        0x94 => "SWAP5".to_string(),
        0x95 => "SWAP6".to_string(),
        0x96 => "SWAP7".to_string(),
        0x97 => "SWAP8".to_string(),
        0x98 => "SWAP9".to_string(),
        0x99 => "SWAP10".to_string(),
        0x9a => "SWAP11".to_string(),
        0x9b => "SWAP12".to_string(),
        0x9c => "SWAP13".to_string(),
        0x9d => "SWAP14".to_string(),
        0x9e => "SWAP15".to_string(),
        0x9f => "SWAP16".to_string(),
        0xf3 => "RETURN".to_string(),
        0xfd => "REVERT".to_string(),
        0xfe => "INVALID".to_string(),
        0xef => "EOFMAGIC".to_string(),
        0x20 => "SHA3".to_string(),
        0x30 => "ADDRESS".to_string(),
        0x31 => "BALANCE".to_string(),
        0x47 => "SELFBALANCE".to_string(),
        0x48 => "BASEFEE".to_string(),
        0x32 => "ORIGIN".to_string(),
        0x33 => "CALLER".to_string(),
        0x34 => "CALLVALUE".to_string(),
        0x3a => "GASPRICE".to_string(),
        0x3b => "EXTCODESIZE".to_string(),
        0x3c => "EXTCODECOPY".to_string(),
        0x3f => "EXTCODEHASH".to_string(),
        0x3d => "RETURNDATASIZE".to_string(),
        0x3e => "RETURNDATACOPY".to_string(),
        0x40 => "BLOCKHASH".to_string(),
        0x41 => "COINBASE".to_string(),
        0x42 => "TIMESTAMP".to_string(),
        0x43 => "NUMBER".to_string(),
        0x44 => "DIFFICULTY".to_string(),
        0x45 => "GASLIMIT".to_string(),
        0x54 => "SLOAD".to_string(),
        0x55 => "SSTORE".to_string(),
        0x5a => "GAS".to_string(),
        0xa0 => "LOG0".to_string(),
        0xa1 => "LOG1".to_string(),
        0xa2 => "LOG2".to_string(),
        0xa3 => "LOG3".to_string(),
        0xa4 => "LOG4".to_string(),
        0xf0 => "CREATE".to_string(),
        0xf5 => "CREATE2".to_string(),
        0xf1 => "CALL".to_string(),
        0xf2 => "CALLCODE".to_string(),
        0xf4 => "DELEGATECALL".to_string(),
        0xfa => "STATICCALL".to_string(),
        0xff => "SUICIDE".to_string(),
        0x46 => "CHAINID".to_string(),
        x => format!("<UNKNOWN 0x{:x}>", x),
    }
}