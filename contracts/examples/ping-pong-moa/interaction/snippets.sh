PEM_FILE="./ping-pong.pem"
PING_PONG_CONTRACT="./dharitri-wasm-rs/contracts/examples/ping-pong-moa"

PROXY_ARGUMENT="--proxy=https://devnet-api.dharitri.com"
CHAIN_ARGUMENT="--chain=D"

build_ping_pong() {
    (set -x; moapy --verbose contract build "$PING_PONG_CONTRACT")
}

deploy_ping_pong() {
    local FIXED_SUM=1500000000000000000 # 1.5 MOA
    local DURATION=$1
    local BEGINNING=$(option_u64_arg $2)
    local MAX_FUNDS=$3

    if [[ $# -ne 3 ]]; then
        echo "Incorrect number of arguments. Need to provide 3 arguments: DURATION(seconds) BEGINNING(unix timestamp) MAX_FUNDS(int)"
        return 1
    fi
    
    local OUTFILE="out.json"
    (set -x; moapy contract deploy --bytecode="$PING_PONG_CONTRACT/output/ping-pong-moa.wasm" \
        --pem="$PEM_FILE" \
        $PROXY_ARGUMENT $CHAIN_ARGUMENT \
        --outfile="$OUTFILE" --recall-nonce --gas-limit=60000000 \
        --arguments $FIXED_SUM $DURATION $BEGINNING $MAX_FUNDS --send \
        || return)

    local RESULT_ADDRESS=$(moapy data parse --file="$OUTFILE" --expression="data['contractAddress']")
    local RESULT_TRANSACTION=$(moapy data parse --file="$OUTFILE" --expression="data['emittedTransactionHash']")

    echo ""
    echo "Deployed contract with:"
    echo "  \$RESULT_ADDRESS == ${RESULT_ADDRESS}"
    echo "  \$RESULT_TRANSACTION == ${RESULT_TRANSACTION}"
    echo ""
}

number_to_u64() {
    local NUMBER=$1
    printf "%016x" $NUMBER
}

option_u64_arg() {
    local NUMBER=$1
    echo "0x01$(number_to_u64 $NUMBER)"
}
