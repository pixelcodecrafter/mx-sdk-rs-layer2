use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/queue-repeat");

    blockchain.register_contract(
        "file:output/queue-repeat.wasm",
        queue_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn queue_repeat_struct_rs() {
    dharitri_wasm_debug::denali_rs("denali/queue_repeat_struct.scen.json", world());
}

#[test]
fn queue_repeat_rs() {
    dharitri_wasm_debug::denali_rs("denali/queue_repeat.scen.json", world());
}
