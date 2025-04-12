use revm::bytecode::Bytecode;
use revm::context::ContextTr;
use revm::database::EmptyDB;
use revm::handler::EvmTr;
use revm::handler::instructions::{EthInstructions, InstructionProvider};
use revm::interpreter::host::DummyHost;
use revm::interpreter::interpreter::{EthInterpreter, ExtBytecode};
use revm::interpreter::interpreter_types::{Jumps, LoopControl, MemoryTr};
use revm::interpreter::{EMPTY_SHARED_MEMORY, InputsImpl, Interpreter};
use revm::primitives::hardfork::SpecId;
use revm::primitives::{Address, Bytes, b256};
use revm::state::{Account, AccountInfo};
use revm::{Context, Journal, MainBuilder};
use std::cell::RefCell;
use std::rc::Rc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO(toms): can we deconstruct `evm` to simplify things? (and pull the necessary types out of it)

    let spec = SpecId::default();
    let mut ctx: Context = Context::new(EmptyDB::new(), spec);

    let target_address = Address::from_word(b256!(
        "0x00000000000000000000000000000000000000000000000000000000000000F0"
    ));
    ctx.journal()
        .state()
        .insert(target_address, Account::default());

    let caller_address = Address::from_word(b256!(
        "0x00000000000000000000000000000000000000000000000000000000000000FF"
    ));
    ctx.journal()
        .state()
        .insert(caller_address, Account::default());

    // let bytecode_address = Address::from_word(b256!(
    //     "0x0000000000000000000000000000000000000000000000000000000000000002"
    // ));
    // ctx.journal().state().insert(
    //     bytecode_address,
    //     AccountInfo::from_bytecode(Bytecode::new_raw(Bytes::from(&[0x60, 0x00][..]))).into(),
    // );

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
        // create_init_frame
        // run_exec_loop
        // EthFrame::process_next_action

        let instructions = EthInstructions::default();
        let instructions = instructions.instruction_table();

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

                instructions[opcode as usize](&mut interpreter, &mut ctx)
            }

            interpreter.take_next_action()
        };

        println!("{action:#?}");
    }

    Ok(())
}

mod isolate {}
