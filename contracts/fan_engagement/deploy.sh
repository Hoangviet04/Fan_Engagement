#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# --- ATTENTION ---
# The secret key is hardcoded below for convenience as requested.
# For production environments, it is strongly recommended to use environment variables
# to avoid exposing sensitive keys in your source code.
export SOROBAN_SECRET_KEY="SATIGQGR5FFESF2VKN63TF246362MNPCCIV53XCJ65XSZONYQCAG6E4F"

# Build the contract
soroban contract build

# Set the network
NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org:443"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Make sure to set your secret key
# For local sandbox, you can use:
# export SOROBAN_SECRET_KEY=$(soroban keys generate --network local)
# For testnet, generate a key and fund it through the Stellar Laboratory or a faucet
if [ -z "$SOROBAN_SECRET_KEY" ]; then
    echo "SOROBAN_SECRET_KEY is not set. Please set it to your Stellar secret key."
    exit 1
fi

# Deploy the contract
CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fan_engagement_contract.wasm \
  --source $(soroban keys address $SOROBAN_SECRET_KEY) \
  --network $NETWORK \
  --rpc-url $RPC_URL \
  --network-passphrase "$NETWORK_PASSPHRASE")

if [ $? -eq 0 ]; then
  echo "Contract deployed successfully!"
  echo "Contract ID: $CONTRACT_ID"
else
  echo "Contract deployment failed."
fi

# Example of how to initialize the contract after deployment
# You would need to replace `YOUR_ADMIN_ADDRESS` with an actual address
# ADMIN_ADDRESS="YOUR_ADMIN_ADDRESS"
# soroban contract invoke \
#   --id $CONTRACT_ID \
#   --source $(soroban keys address $SOROBAN_SECRET_KEY) \
#   --network $NETWORK \
#   --rpc-url $RPC_URL \
#   --network-passphrase "$NETWORK_PASSPHRASE" \
#   -- \
#   initialize \
#   --admin $ADMIN_ADDRESS

