#!/bin/bash
set -e

# --- CONFIGURATION ---
# ID của hợp đồng đã triển khai thành công
# LƯU Ý: Nếu bạn đã triển khai lại, hãy đảm bảo ID này là mới nhất
CONTRACT_ID="CCPM77BQ3PI6GRRVCNESVLD4IE5WVRWICRYITIB4SVXFKGXCABRE3UXU"

# ID của NFT mà chúng ta đang thanh toán bản quyền
TOKEN_ID=1

# Tên định danh của người đang thực hiện thanh toán (đóng vai người mua)
SOURCE_IDENTITY="deployer"

# Số tiền bản quyền để thanh toán (đơn vị: stroops, 1 XLM = 10,000,000 stroops)
ROYALTY_AMOUNT="10000000"
# --- END CONFIGURATION ---

echo "Đang mô phỏng thanh toán tiền bản quyền cho NFT có ID: $TOKEN_ID..."

# Gọi hàm pay_royalty của hợp đồng
# LƯU Ý: Chúng ta dùng chính $SOURCE_IDENTITY cho cả cờ --source và --buyer
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE_IDENTITY" \
  --network testnet \
  -- \
  pay_royalty \
  --buyer "$SOURCE_IDENTITY" \
  --token_id "$TOKEN_ID" \
  --amount "$ROYALTY_AMOUNT"

echo ""
echo "✅ Mô phỏng thanh toán bản quyền thành công!"
