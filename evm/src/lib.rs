use gstd::{
    codec::{Decode, Encode},
    msg,
};
use revm::{
    primitives::{
        AccountInfo, Bytecode, Bytes, ExecutionResult, Output, ResultAndState, TransactTo, U256,
    },
    InMemoryDB, EVM,
};

/// Transaction gas limit.
const GAS_LIMIT: u64 = 1_000_000_000;

/// Alice account address.
pub const ALICE: [u8; 20] = [0; 20];

/// Contract address if any.
pub const CONTRACT: [u8; 20] = [1; 20];

/// Inputs to the EVM.
#[derive(Clone, Debug, Encode, Decode)]
#[codec(crate = gstd::codec)]
pub struct Input {
    /// The runtime bytecode of the contract.
    pub runtime_bytecode: Vec<u8>,
    /// The call data for the contract.
    pub input: Vec<u8>,
}

#[no_mangle]
extern "C" fn handle() {
    let runtime_bytecode = msg::load_bytes().expect("Failed to load payload");

    let mut db = InMemoryDB::default();
    db.insert_account_info(ALICE.into(), AccountInfo::from_balance(U256::MAX));
    db.insert_account_info(
        CONTRACT.into(),
        AccountInfo::new(
            Default::default(),
            0,
            Default::default(),
            Bytecode::new_raw(Bytes::copy_from_slice(&runtime_bytecode)),
        ),
    );

    let mut evm = EVM::<InMemoryDB>::new();
    evm.database(db);
    evm.env.tx.gas_limit = GAS_LIMIT;
    evm.env.tx.caller = ALICE.into();
    evm.env.tx.transact_to = TransactTo::Call(CONTRACT.into());

    let Ok(ResultAndState {
        result:
            ExecutionResult::Success {
                output: Output::Call(data),
                ..
            },
        ..
    }) = evm.transact()
    else {
        panic!("Failed to execute contract");
    };

    msg::reply_bytes(&data, 0).expect("Failed to reply");
    // msg::send_bytes(msg::source(), &data, 0).expect("Failed to send output");
}

#[test]
fn add_two() {
    use gtest::{Program, System};
    use revm::primitives::alloy_primitives::hex::FromHex;

    let runtime_bytecode = Bytes::from_hex("6000356020350160005260206000f3")
        .expect("Failed to decode bytecode")
        .to_vec();
    let mut one: [u8; 32] = [0; 32];
    one[31] = 1;

    let mut two: [u8; 32] = [0; 32];
    two[31] = 2;

    //
    // testing with gtest.
    //

    let system = System::new();
    system.init_logger();
    let program = Program::current(&system);

    // Init program.
    let init = program.send_bytes(0, b"init");
    assert!(!init.main_failed());

    let result = program.send(
        0,
        Input {
            runtime_bytecode,
            input: [one, one].concat().to_vec(),
        },
    );

    println!("{result:?}");
    // Currently failed with:
    //
    // ...
    //
    // [runtime::sandbox add_two] invocation error: wasm `unreachable` instruction executed
    // [gear_core_backend::state add_two] Execution result = Err(Error::Execution)
    //
    // ...
    assert!(result.main_failed());
}
