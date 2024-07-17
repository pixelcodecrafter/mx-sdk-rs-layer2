use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract("file:output/erc721.wasm", erc721::ContractBuilder);
    blockchain
}

#[test]
fn nft_approve_non_existent_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-approve-non-existent-token.scen.json", world());
}

#[test]
fn nft_approve_non_owned_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-approve-non-owned-token.scen.json", world());
}

#[test]
fn nft_approve_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-approve-ok.scen.json", world());
}

#[test]
fn nft_init_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-init.scen.json", world());
}

#[test]
fn nft_mint_more_tokens_caller_not_owner_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/nft-mint-more-tokens-caller-not-owner.scen.json",
        world(),
    );
}

#[test]
fn nft_mint_more_tokens_receiver_acc1_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/nft-mint-more-tokens-receiver-acc1.scen.json",
        world(),
    );
}

#[test]
fn nft_mint_more_tokens_receiver_owner_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/nft-mint-more-tokens-receiver-owner.scen.json",
        world(),
    );
}

#[test]
fn nft_revoke_non_approved_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-revoke-non-approved.scen.json", world());
}

#[test]
fn nft_revoke_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-revoke-ok.scen.json", world());
}

#[test]
fn nft_transfer_approved_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-transfer-approved-token.scen.json", world());
}

#[test]
fn nft_transfer_non_existent_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-transfer-non-existent-token.scen.json", world());
}

#[test]
fn nft_transfer_not_owned_not_approved_token_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/nft-transfer-not-owned-not-approved-token.scen.json",
        world(),
    );
}

#[test]
fn nft_transfer_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-transfer-ok.scen.json", world());
}

#[test]
fn nft_transfer_token_after_revoked_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-transfer-token-after-revoked.scen.json", world());
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_caller_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/nft-transfer-token-not-owner-no-approval-to-caller.scen.json",
        world(),
    );
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_other_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/nft-transfer-token-not-owner-no-approval-to-other.scen.json",
        world(),
    );
}

#[test]
fn nft_transfer_token_ok_rs() {
    dharitri_wasm_debug::denali_rs("denali/nft-transfer-token-ok.scen.json", world());
}
