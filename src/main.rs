use revm::bytecode::Bytecode;
use revm::context::ContextTr;
use revm::database::EmptyDB;
use revm::handler::instructions::InstructionProvider;
use revm::interpreter::interpreter::{EthInterpreter, ExtBytecode};
use revm::interpreter::interpreter_types::{Jumps, LoopControl, MemoryTr};
use revm::interpreter::{EMPTY_SHARED_MEMORY, InputsImpl, Interpreter};
use revm::primitives::hardfork::SpecId;
use revm::primitives::{Address, Bytes, b256};
use revm::state::{Account, AccountInfo};
use revm::{Context, MainBuilder};
use std::cell::RefCell;
use std::rc::Rc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // // Set up the HTTP transport which is consumed by the RPC client.
    // let rpc_url = format!("https://mainnet.infura.io/v3/{API_KEY}").parse()?;
    //
    // // Create a provider
    // let client = ProviderBuilder::new().on_http(rpc_url);
    //
    // // Params
    // let chain_id: u64 = 1;
    // let block_number = 10889447;
    //
    // // Fetch the transaction-rich block
    // let block = match client
    //     .get_block_by_number(BlockNumberOrTag::Number(block_number))
    //     .full()
    //     .await
    // {
    //     Ok(Some(block)) => block,
    //     Ok(None) => anyhow::bail!("Block not found"),
    //     Err(error) => anyhow::bail!("Error: {:?}", error),
    // };
    // println!("Fetched block number: {}", block.header.number);
    // let previous_block_number = block_number - 1;
    //
    // // Use the previous block state as the db with caching
    // let prev_id: BlockId = previous_block_number.into();
    // // SAFETY: This cannot fail since this is in the top-level tokio runtime
    //
    // let state_db = WrapDatabaseAsync::new(AlloyDB::new(client, prev_id)).unwrap();
    // let cache_db: CacheDB<_> = CacheDB::new(state_db);
    // let mut state = StateBuilder::new_with_database(cache_db).build();
    // let ctx = Context::mainnet()
    //     .with_db(&mut state)
    //     .modify_block_chained(|b| {
    //         b.number = block.header.number;
    //         b.beneficiary = block.header.beneficiary;
    //         b.timestamp = block.header.timestamp;
    //
    //         b.difficulty = block.header.difficulty;
    //         b.gas_limit = block.header.gas_limit;
    //         b.basefee = block.header.base_fee_per_gas.unwrap_or_default();
    //     })
    //     .modify_cfg_chained(|c| {
    //         c.chain_id = chain_id;
    //     });
    // std::fs::create_dir_all("target/traces")?;
    //
    // let write = OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .truncate(true)
    //     .open("target/traces/0.jsonl");
    // let inner = Arc::new(Mutex::new(BufWriter::new(
    //     write.expect("Failed to open file"),
    // )));
    // let writer = FlushWriter::new(Arc::clone(&inner));
    // let mut evm = ctx.build_mainnet_with_inspector(TracerEip3155::new(Box::new(writer)));
    //
    // let txs = block.transactions.len().min(5);
    // println!(
    //     "Found {} transactions. (Fetching the first {txs}.)",
    //     block.transactions.len()
    // );
    //
    // let console_bar = Arc::new(ProgressBar::new(txs as u64));
    // let start = Instant::now();
    //
    // // Fill in CfgEnv
    // let BlockTransactions::Full(transactions) = block.transactions else {
    //     panic!("Wrong transaction type")
    // };
    //
    // for tx in transactions.iter().take(txs) {
    //     evm.modify_tx(|etx| {
    //         etx.caller = tx.inner.signer();
    //         etx.gas_limit = tx.gas_limit();
    //         etx.gas_price = tx.gas_price().unwrap_or(tx.inner.max_fee_per_gas());
    //         etx.value = tx.value();
    //         etx.data = tx.input().to_owned();
    //         etx.gas_priority_fee = tx.max_priority_fee_per_gas();
    //         etx.chain_id = Some(chain_id);
    //         etx.nonce = tx.nonce();
    //         if let Some(access_list) = tx.access_list() {
    //             etx.access_list = access_list.clone()
    //         } else {
    //             etx.access_list = Default::default();
    //         }
    //
    //         etx.kind = match tx.to() {
    //             Some(to_address) => TxKind::Call(to_address),
    //             None => TxKind::Create,
    //         };
    //     });
    //
    //     // Construct the file writer to write the trace to
    //     let tx_number = tx.transaction_index.unwrap_or_default();
    //     let file_name = format!("target/traces/{}.jsonl", tx_number);
    //     let write = OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .truncate(true)
    //         .open(file_name)
    //         .expect("Failed to open file");
    //     let inner = Arc::new(Mutex::new(BufWriter::new(write)));
    //     let writer = FlushWriter::new(Arc::clone(&inner));
    //
    //     // Inspect and commit the transaction to the EVM
    //     let res = evm.inspect_replay_with_inspector(TracerEip3155::new(Box::new(writer)));
    //
    //     if let Err(error) = res {
    //         println!("Got error: {:?}", error);
    //     }
    //
    //     // Flush the file writer
    //     inner.lock().unwrap().flush().expect("Failed to flush file");
    //
    //     console_bar.inc(1);
    // }
    //
    // console_bar.finish_with_message("Finished all transactions.");
    //
    // let elapsed = start.elapsed();
    // println!(
    //     "Finished execution. Total CPU time: {:.6}s",
    //     elapsed.as_secs_f64()
    // );

    // TODO(toms): can we deconstruct `evm` to simplify things? (and pull the necessary types out of it)

    let spec = SpecId::default();
    let ctx: Context = Context::new(EmptyDB::new(), spec);
    let mut evm = ctx.build_mainnet_with_inspector(());

    let target_address = Address::from_word(b256!(
        "0x00000000000000000000000000000000000000000000000000000000000000F0"
    ));
    evm.journal()
        .state()
        .insert(target_address, Account::default());

    let caller_address = Address::from_word(b256!(
        "0x00000000000000000000000000000000000000000000000000000000000000FF"
    ));
    evm.journal()
        .state()
        .insert(caller_address, Account::default());

    let bytecode_address = Address::from_word(b256!(
        "0x0000000000000000000000000000000000000000000000000000000000000002"
    ));
    evm.journal().state().insert(
        bytecode_address,
        AccountInfo::from_bytecode(Bytecode::new_raw(Bytes::from(&[0x60, 0x00][..]))).into(),
    );

    let memory = Rc::new(RefCell::new(EMPTY_SHARED_MEMORY));
    let interpreter_input = InputsImpl {
        target_address,
        caller_address,
        input: Default::default(),
        call_value: Default::default(),
    };
    let ext_bytecode = {
        // let bytecode = Bytecode::new_raw(Bytes::from(&[0x60, 0x00][..]));

        // https://github.com/ethereum/go-ethereum/tree/master/cmd/evm
        /*
        // Map of address to account definition.
        type Alloc map[common.Address]Account
        // Genesis account. Each field is optional.
        type Account struct {
            Code       []byte                           `json:"code"`
            Storage    map[common.Hash]common.Hash      `json:"storage"`
            Balance    *big.Int                         `json:"balance"`
            Nonce      uint64                           `json:"nonce"`
            SecretKey  []byte                            `json:"secretKey"`
        }

        type Env struct {
            // required
            CurrentCoinbase  common.Address      `json:"currentCoinbase"`
            CurrentGasLimit  uint64              `json:"currentGasLimit"`
            CurrentNumber    uint64              `json:"currentNumber"`
            CurrentTimestamp uint64              `json:"currentTimestamp"`
            Withdrawals      []*Withdrawal       `json:"withdrawals"`
            // optional
            CurrentDifficulty *big.Int           `json:"currentDifficulty"`
            CurrentRandom     *big.Int           `json:"currentRandom"`
            CurrentBaseFee    *big.Int           `json:"currentBaseFee"`
            ParentDifficulty  *big.Int           `json:"parentDifficulty"`
            ParentGasUsed     uint64             `json:"parentGasUsed"`
            ParentGasLimit    uint64             `json:"parentGasLimit"`
            ParentTimestamp   uint64             `json:"parentTimestamp"`
            BlockHashes       map[uint64]common.Hash `json:"blockHashes"`
            ParentUncleHash   common.Hash        `json:"parentUncleHash"`
            Ommers            []Ommer            `json:"ommers"`
        }

        type LegacyTx struct {
            Nonce     uint64          `json:"nonce"`
            GasPrice  *big.Int        `json:"gasPrice"`
            Gas       uint64          `json:"gas"`
            To        *common.Address `json:"to"`
            Value     *big.Int        `json:"value"`
            Data      []byte          `json:"data"`
            V         *big.Int        `json:"v"`
            R         *big.Int        `json:"r"`
            S         *big.Int        `json:"s"`
            SecretKey *common.Hash    `json:"secretKey"`
        }
        */

        // https://eips.ethereum.org/EIPS/eip-3155#test-cases
        //
        // λ evm run --code '0x604080536040604055604060006040600060ff5afa6040f3'
        //     --json --debug --dump --nomemory=false --noreturndata=false
        //     --sender '0xF0' --receiver '0xF1' --gas 10000000000
        //
        // {"opName":"PUSH1","pc":0,"op":96,"gas":"0x2540be400","gasCost":"0x3","memSize":0,"stack":[],"depth":1,"refund":0}
        // {"opName":"DUP1","pc":2,"op":128,"gas":"0x2540be3fd","gasCost":"0x3","memSize":0,"stack":["0x40"],"depth":1,"refund":0}
        // {"opName":"MSTORE8","pc":3,"op":83,"gas":"0x2540be3fa","gasCost":"0xc","memSize":0,"stack":["0x40","0x40"],"depth":1,"refund":0}
        // {"opName":"PUSH1","pc":4,"op":96,"gas":"0x2540be3ee","gasCost":"0x3","memSize":96,"stack":[],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"PUSH1","pc":6,"op":96,"gas":"0x2540be3eb","gasCost":"0x3","memSize":96,"stack":["0x40"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"SSTORE","pc":8,"op":85,"gas":"0x2540be3e8","gasCost":"0x5654","memSize":96,"stack":["0x40","0x40"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"PUSH1","pc":9,"op":96,"gas":"0x2540b8d94","gasCost":"0x3","memSize":96,"stack":[],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"PUSH1","pc":11,"op":96,"gas":"0x2540b8d91","gasCost":"0x3","memSize":96,"stack":["0x40"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"PUSH1","pc":13,"op":96,"gas":"0x2540b8d8e","gasCost":"0x3","memSize":96,"stack":["0x40","0x0"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"PUSH1","pc":15,"op":96,"gas":"0x2540b8d8b","gasCost":"0x3","memSize":96,"stack":["0x40","0x0","0x40"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"PUSH1","pc":17,"op":96,"gas":"0x2540b8d88","gasCost":"0x3","memSize":96,"stack":["0x40","0x0","0x40","0x0"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"GAS","pc":19,"op":90,"gas":"0x2540b8d85","gasCost":"0x2","memSize":96,"stack":["0x40","0x0","0x40","0x0","0xff"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"STATICCALL","pc":20,"op":250,"gas":"0x2540b8d83","gasCost":"0x24abb5f76","memSize":96,"stack":["0x40","0x0","0x40","0x0","0xff","0x2540b8d83"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"PUSH1","pc":21,"op":96,"gas":"0x2540b835b","gasCost":"0x3","memSize":96,"stack":["0x1"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"opName":"RETURN","pc":23,"op":243,"gas":"0x2540b8358","gasCost":"0x0","memSize":96,"stack":["0x1","0x40"],"depth":1,"refund":0,"memory":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000"}
        // {"output":"40","gasUsed":"0x60a8"}
        // {
        //     "root": "3463104800c5985b196eb96437cdff04e0a669d85a898ff68924353b5f973597",
        //     "accounts": {
        //         "0x00000000000000000000000000000000000000f1": {
        //             "balance": "0",
        //             "nonce": 0,
        //             "root": "0x362d2b556fc3ace7e6b0a2d2ddd306a7bc0cc299f5264d9abd557cde6cd2dbf2",
        //             "codeHash": "0x58c35e4d81bf6b27e1725f0b3c3364b849bace3196710ee574714da41310b492",
        //             "code": "0x604080536040604055604060006040600060ff5afa6040f3",
        //             "storage": {
        //                 "0x0000000000000000000000000000000000000000000000000000000000000040": "40"
        //             },
        //             "address": "0x00000000000000000000000000000000000000f1",
        //             "key": "0xe8c07bab8822eeeb875236e148f781341157f9bfc56c1c53972489ff4009695b"
        //         }
        //     }
        // }

        let bytecode = Bytecode::new_raw(Bytes::from(
            &[
                0x60, 0x40, 0x80, 0x53, 0x60, 0x40, 0x60, 0x40, 0x55, 0x60, 0x40, 0x60, 0x00, 0x60,
                0x40, 0x60, 0x00, 0x60, 0x02, 0x5a, 0xfa, 0x60, 0x40, 0xf3,
            ][..],
        ));
        let hash = bytecode.hash_slow();
        let ext_bytecode = ExtBytecode::new_with_hash(bytecode.clone(), hash);
        // assert_eq!(ext_bytecode.bytecode_hash, Some(hash));
        ext_bytecode
    };
    let mut interpreter = Interpreter::<EthInterpreter>::new(
        memory.clone(),
        ext_bytecode, // ExtBytecode::new(Bytecode::Eof(Arc::new(initcode))),
        interpreter_input,
        false,
        true,
        spec,
        // NOTE(toms): matches default of `evmtool`
        10_000_000_000,
    );

    {
        // evm.run_interpreter(&mut interpreter);

        let context = &mut evm.data.ctx;
        // let action = interpreter.run_plain(instructions.instruction_table(), context);
        let action = {
            interpreter.reset_control();

            // Main loop
            while interpreter.control.instruction_result().is_continue() {
                // interpreter.step(instruction_table, host);

                // STATICCALL:
                // Stack input
                //     gas: amount of gas to send to the sub context to execute. The gas that is not used by the sub context is returned to this one.
                //     address: the account which context to execute.
                //     argsOffset: byte offset in the memory in bytes, the calldata of the sub context.
                //     argsSize: byte size to copy (size of the calldata).
                //     retOffset: byte offset in the memory in bytes, where to store the return data of the sub context.
                //     retSize: byte size to copy (size of the return data).

                let pc = interpreter.bytecode.pc();
                let opcode = interpreter.bytecode.opcode();
                let stack = interpreter.stack.data();
                let gas_remaining = interpreter.control.gas().remaining();
                println!(
                    "pc={pc:?} opcode={opcode:?} stack={stack:?} memSize={} gas_remaining=0x{gas_remaining:x}",
                    memory.size()
                );

                // SAFETY: In analysis we are doing padding of bytecode so that we are sure that last
                // byte instruction is STOP so we are safe to just increment program_counter bcs on last instruction
                // it will do noop and just stop execution of this contract
                interpreter.bytecode.relative_jump(1);

                // Execute instruction.
                evm.instruction.instruction_table()[opcode as usize](&mut interpreter, context)
            }

            interpreter.take_next_action()
        };
        println!("{action:#?}");
    }

    Ok(())
}
