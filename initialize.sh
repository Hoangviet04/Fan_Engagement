#!/bin/bash
set -e

# --- CONFIGURATION ---
# The Contract ID from the successful deployment
CONTRACT_ID="CCPM77BQ3PI6GRRVCNESVLD4IE5WVRWICRYITIB4SVXFKGXCABRE3UXU"

# The public key of the account that will be the admin of this contract
ADMIN_ADDRESS="GCJIZYOR6VHGSUOXKHDZB62PNPAFTKSONNPLGVD57NXLQYTYF4TEGAKA"

# The identity name we created in the deploy script
SOURCE_IDENTITY="deployer"
# --- END CONFIGURATION ---

echo "Initializing contract with admin address..."

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE_IDENTITY" \
  --network testnet \
  -- \
  initialize \
  --admin "$ADMIN_ADDRESS"

echo "✅ Contract initialized successfully!"
