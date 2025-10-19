#!/bin/bash
set -e

# --- CONFIGURATION ---
# IMPORTANT: This secret key is for a testnet account. DO NOT use it on the mainnet.
SECRET_KEY="SASLV6J573UU3WRBZFTVF3O37DZPVFYSR26QWZVAWU5EKVUN5HWITEJ5"
IDENTITY_NAME="deployer" # A name for your key, can be anything
# --- END CONFIGURATION ---


# Build the contract
echo "Building contract..."
soroban contract build

# Add the secret key as a named identity using a pipe for maximum compatibility.
echo "Adding deployment key as identity '$IDENTITY_NAME'..."
echo "$SECRET_KEY" | soroban keys add "$IDENTITY_NAME"

# Define wasm file path
WASM_FILE="target/wasm32v1-none/release/fan_engagement_contract.wasm"

# Deploy the contract using the named identity
echo "Deploying contract..."
soroban contract deploy \
  --source "$IDENTITY_NAME" \
  --network testnet \
  --wasm "$WASM_FILE"

echo "Contract deployed successfully!"
echo "Note: To initialize the contract, you will need the Contract ID from the output above."

# To initialize the contract, uncomment the lines below and replace
# YOUR_ADMIN_ADDRESS with the public key of the admin and the new CONTRACT_ID.
# Then, run the script again.

# echo "Initializing contract..."
# CONTRACT_ID="<PASTE_YOUR_CONTRACT_ID_HERE>"
# YOUR_ADMIN_ADDRESS="<PASTE_YOUR_ADMIN_PUBLIC_KEY_HERE>"

# soroban contract invoke \
#   --id "$CONTRACT_ID" \
#   --source "$IDENTITY_NAME" \
#   --network testnet \
#   -- \
#   initialize \
#   --admin "$YOUR_ADMIN_ADDRESS"

