#!/bin/bash
set -e

# --- CONFIGURATION ---
# ID của hợp đồng đã triển khai thành công
CONTRACT_ID="CCPM77BQ3PI6GRRVCNESVLD4IE5WVRWICRYITIB4SVXFKGXCABRE3UXU"

# ID của NFT mà bạn muốn xem chi tiết
TOKEN_ID=1

# Tên định danh chúng ta đã tạo trong script deploy
# Mặc dù đây là lệnh đọc (read), việc cung cấp source sẽ đảm bảo tương thích
SOURCE_IDENTITY="deployer"
# --- END CONFIGURATION ---

echo "Đang truy vấn thông tin chi tiết cho NFT có ID: $TOKEN_ID..."

# Sử dụng 'invoke' thay vì 'read' để tương thích với các phiên bản soroban-cli cũ hơn
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE_IDENTITY" \
  --network testnet \
  -- \
  get_nft \
  --token_id "$TOKEN_ID"

echo ""
echo "✅ Truy vấn thành công!"