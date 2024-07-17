use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/erc-style-contracts/crowdfunding-erc20",
    );

    blockchain.register_contract(
        "file:output/crowdfunding-erc20.wasm",
        crowdfunding_erc20::ContractBuilder,
    );

    blockchain.register_contract("file:../erc20/output/erc20.wasm", erc20::ContractBuilder);

    blockchain
}

#[test]
fn deploy_erc20_and_crowdfunding_rs() {
    dharitri_wasm_debug::denali_rs("denali/deploy_erc20_and_crowdfunding.scen.json", world());
}

#[test]
fn fund_with_insufficient_allowance_rs() {
    dharitri_wasm_debug::denali_rs("denali/fund_with_insufficient_allowance.scen.json", world());
}

#[test]
fn fund_with_sufficient_allowance_rs() {
    dharitri_wasm_debug::denali_rs("denali/fund_with_sufficient_allowance.scen.json", world());
}

#[test]
fn fund_without_allowance_rs() {
    dharitri_wasm_debug::denali_rs("denali/fund_without_allowance.scen.json", world());
}
