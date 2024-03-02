# Data Detective

### A Token indexing tool that embeds and EVM instance in the substreams to allow us to get token data while making zero rpc calls.


## Challenges with Substreams

Substreams significantly enhance subgraph development by offering improved performance, greater flexibility in data processing, and a more streamlined development experience but they have their own set of challenges, particularly concerning the reliance on RPC (Remote Procedure Call) calls for fetching token data. This reliance can introduce several issues:

- **Performance Bottlenecks**: Frequent RPC calls can significantly slow down data processing, especially during periods of high demand.
- **Data Availability and Consistency**: Relying on external sources for data can lead to inconsistencies and potential delays in data fetching.
- **Scalability Concerns**: As the need for data grows, the sheer volume of RPC calls can overwhelm infrastructure, leading to scalability issues.

## Our Solution: Embedded EVM Instance

To address the challenges associated with reliance on RPC calls in Substreams, our project introduces an innovative solution: an embedded EVM  instance using the evm_core crate. This approach offers several key benefits:

### Direct Access to Token Data:

- By embedding an EVM instance within the Substream, our solution can directly access token data stored on the blockchain, eliminating the need for external RPC calls. 

### Enhanced Performance:

- This direct access approach significantly reduces the data processing time, offering a substantial performance improvement over traditional methods that rely on RPC calls.

### Improved Data Consistency and Reliability:

- Accessing data directly from the blockchain ensures consistency and reliability, as there is no intermediary that could introduce delays or inconsistencies.

### Scalability:

- Our solution scales efficiently with the blockchain, as the embedded EVM instance processes data locally, reducing the dependency on external infrastructure and avoiding common bottlenecks associated with RPC calls.

## How we build the EVM instance

Our solution enhances blockchain data processing by leveraging information available on the blockchain itself. Here's a brief overview of our approach:

#### Using Code Changes as Contract Source Code

- **Contract Deployments**: When a smart contract is deployed on the blockchain, it includes the deployment of contract bytecode. This bytecode is the low-level, executable code that defines the contract's behavior and capabilities. This bytecode deployment appears as code changes from the block.
- **Code Changes Extraction**: We capture the contracts bytecode from the code changes during the deployment allowing us to have the definitive version of the contracts code as it exists on the blockchain. 
- **Usage**: With the definitive version of the contracts source code, paired with our EVM instance we can execute or simulate contract interactions. 
``` rust
 // We grab the last code_change to get the most recent version of contract logic
 if let Some(last_code_change) = call.code_changes.iter().last() {
                let code = &last_code_change.new_code;
```

### Using Storage Changes as the Contract Storage Layout

- **Storage Changes**: Smart contracts on the blockchain have associated storage, a key-value store that holds the contract's state. When transactions modify the contract's state, these changes are reflected as storage changes.
- **Extraction and Representation**: Our substream monitors and extracts these storage changes during the contract deployment. This provides us with a view of the contract's storage layout, allowing us access to state variables such as token Name, Symbol, Decimals and TotalSupply.
- **Application**: We build a hashmap with the hashed storage key and new value of the storage change so we can match the storage key pushed to the stack during the evm execution. 
``` rust
// We Build a Hashmap with the hashed storage key to match the storage key pushed to the stack during function execution
let storage_changes: HashMap<H256, Vec<u8>> = call
                    .storage_changes
                    .iter()
                    .map(|s| (H256::from_slice(s.key.as_ref()), s.new_value.clone()))
                    .collect();
```

#### Using Function Abi to execute functions locally

A critical aspect of our substreams functionality involves interacting with smart contracts in a way that accurately reflects their intended behavior. To achieve this, we utilize the ABI to get our function data. With knowledge of the contract's functions, our EVM instance can dynamically construct and execute function calls. This allows for precise interactions based on the contract's actual code and design. Integrating the contract bytecode with the dynamically captured storage layout, our embedded EVM instance can execute or simulate contract functions locally. 


## Getting the data to build our EVM instance

We integrated our evm instance into our intitial map module. We map the data from the block and pass it into our EVM instance in the form of Contract Creation Structs. We implemented methods for our structs that take in data from the block and identify if it is an erc20 or erc721 and then build a ERC20Creation or ERC721Creation struct accordingly. 

We did the following to get the data necessary to build our EVM instance.

1.  filter all the transactions in the block by transaction status. 
2.  filter the calls for that transaction and check that the calls were not reverted. 
3. collect these calls into a vector of successful calls. 
4.   filter the vector of sucessful calls by the Creation CallType to ensure they are contract creation calls. 
5.  push those calls into a vector named `all_calls` to be later used identifying  proxy contracts. 
6. grab the code from the code changes, the call address, and storage changes from the call.  
7. use pattern matching on the storage changes to grab the base token uri if possible. 
8. For erc20 we create a ERC20Creation struct from the address, code, changes, and storage changes.  
9. For erc721 we create the ERC721Creation struct by passing in all the calls and the base token uri. We need to pass all the calls into the ERC721 creation struct to analyze the call tree for potential proxy contracts. 
10. On the sucessful creation of these structs we pass them into our `process_erc20_contract` or `process_erc721_contract` functions.
``` rust
let filtered_calls: Vec<_> = blk
        .transaction_traces
        .into_iter()
        .filter(|tx| tx.status == 1)
        .flat_map(|tx| {
            tx.calls.into_iter().filter(|call| !call.state_reverted)
        })
        .collect();
    for call in filtered_calls {
        if call.call_type == eth::CallType::Create as i32 {
            let mut all_calls = Vec::new();
            all_calls.push(&call);
            if let Some(last_code_change) = call.code_changes.iter().last() {
                let code = &last_code_change.new_code;
                let address = &call.address.to_vec();
                let token_uri = get_token_uri(&call);
                let storage_changes: HashMap<H256, Vec<u8>> = call
                    .storage_changes
                    .iter()
                    .map(|s| (H256::from_slice(s.key.as_ref()), 
                    s.new_value.clone()))
                    .collect();
                if let Some(token) =
                    ERC20Creation::from_call(&address, code.to_vec(), 
                    storage_changes.clone())
                {
                    if let Some(deployment) = process_erc20_contract(token, 
                    clk.clone()) {
                        erc20_contracts.push(deployment);
                    }
                } else if let Some(token) = ERC721Creation::from_call(all_calls, 
                token_uri) {
                    if let Some(deployment) = process_erc721_contract(token, 
                    clk.clone()) {
                        erc721_contracts.push(deployment);
                    }
                }
            }
```


## Using the EVM instance to get our token data

Our process begins by instantiating the Deployment Protobuf, incorporating the contract's creation address and block number as specified in our struct. Initially, other fields are left as empty strings, awaiting population with data from EVM execution outcomes. Into the `execute_on` function, we input the contract's address, bytecode, and observed storage changes, accompanied by encoded function data for the targeted execution. This function then operates within our EVM instance, executing the specified function and comparing its return value against the expected outcome.

``` rust
pub fn process_erc20_contract(contract_creation: ERC20Creation, clock: Clock) -> Option<Erc20Deployment> {
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
        Ok(return_value) => match 
        functions::Symbol::output(return_value.as_ref()) {
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
        Ok(return_value) => match 
        functions::Decimals::output(return_value.as_ref()) {
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
        Ok(return_value) => match 
        functions::TotalSupply::output(return_value.as_ref()) {
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

```

``` rust
pub fn process_erc721_contract(
    contract_creation: ERC721Creation,
    clock: Clock,
) -> Option<Erc721Deployment> {
    let mut contract = Erc721Deployment {
        address: Hex::encode(&contract_creation.address),
        name: String::new(),
        symbol: String::new(),
        blocknumber: clock.number.to_string(),
        token_uri: contract_creation.token_uri,
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
        Ok(return_value) => match 
        functions::Symbol::output(return_value.as_ref()) {
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

```

#### Execute On Function

The `execute_on` function constructs the Machine Instance using our contract creation struct, initializing it with the contract's bytecode and the encoded function data, while setting both the stack and memory limits to 1024.

``` rust
let mut machine = evm_core::Machine::new(
        code,
        Rc::new(data), // name()
        // Rc::new(vec![0x5c, 0x97, 0x5a, 0xbb]), // paused()
        1024,
        1024,
    );

```

Upon creating the machine instance, we enter a loop where an `active_opcode` variable is initialized to `None`. We then proceed to inspect and execute the next opcode, which results in either an `Ok(())` indicating normal execution, or an `Err` containing a `Capture` enum—signifying either an `ExitReason` (for scenarios like errors, reverts, self-destructs, or returns) or a `Trap` (indicating the need for further input or processing).
#### Handling Traps

We address the Trap condition for CALLVALUE and SLOAD opcodes during execution as follows:

- **For CALLVALUE Opcode**: When encountering the CALLVALUE opcode, we insert a zero-initialized hash onto the Stack. This zero hash is used purely to fulfill EVM requirements, simulating the behavior expected within a typical EVM execution.

- **For SLOAD Operation**: The handling of an SLOAD operation is slightly different. This opcode typically succeeds a PUSH, where a storage key is pushed onto the Stack. In regular EVM execution:
    - The SLOAD opcode reads this key, retrieves the corresponding storage value, and then pushes this value back onto the Stack.
    - Without direct access to the contract's storage layout, our approach simulates this process. Upon an SLOAD encounter, we pop the storage key from the Stack. Using our HashMap of Storage Changes, we look up the value associated with this key.
    - After finding the value, we manually push it onto the Stack, where it is then returned to our process. This value is subsequently used to fill in the relevant field in the protobuff with the data obtained.




