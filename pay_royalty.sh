#!/bin/bash
set -e

# --- CONFIGURATION ---
# ID của hợp đồng đã triển khai thành công
CONTRACT_ID="CCPM77BQ3PI6GRRVCNESVLD4IE5WVRWICRYITIB4SVXFKGXCABRE3UXU"

# ID của NFT mà chúng ta đang thanh toán bản quyền
TOKEN_ID=1

# Tên định danh của người đang thực hiện thanh toán (đóng vai người mua)
SOURCE_IDENTITY="deployer"

# Địa chỉ công khai của người mua (phải khớp với SOURCE_IDENTITY)
BUYER_ADDRESS="GCJIZYOR6VHGSUOXKHDZB62PNPAFTKSONNPLGVD57NXLQYTYF4TEGAKA"

# Số tiền bản quyền để thanh toán (đơn vị: stroops, 1 XLM = 10,000,000 stroops)
ROYALTY_AMOUNT="10000000"
# --- END CONFIGURATION ---

echo "Đang mô phỏng thanh toán tiền bản quyền cho NFT có ID: $TOKEN_ID..."

# Gọi hàm pay_royalty của hợp đồng
# Trong một hợp đồng thực tế, hàm này sẽ xử lý việc chuyển token.
# Trong ví dụ này, nó sẽ ghi lại một sự kiện (event) trên blockchain.
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE_IDENTITY" \
  --network testnet \
  -- \
  pay_royalty \
  --buyer "$BUYER_ADDRESS" \
  --token_address "$TOKEN_ID" \
  --amount "$ROYALTY_AMOUNT"
  

echo ""
echo "✅ Mô phỏng thanh toán bản quyền thành công!"

