#!/bin/bash
set -e

# --- CONFIGURATION ---
# ID của hợp đồng đã triển khai thành công
# LƯU Ý: Nếu bạn đã triển khai lại, hãy đảm bảo ID này là mới nhất
CONTRACT_ID="CCPM77BQ3PI6GRRVCNESVLD4IE5WVRWICRYITIB4SVXFKGXCABRE3UXU"

# Tên định danh chúng ta đã tạo trong script deploy
SOURCE_IDENTITY="deployer"

# Địa chỉ public key của người sẽ sở hữu NFT mới.
OWNER_ADDRESS="GCJIZYOR6VHGSUOXKHDZB62PNPAFTKSONNPLGVD57NXLQYTYF4TEGAKA"

# Địa chỉ public key của người tạo ra NFT (dùng để trả tiền bản quyền).
CREATOR_ADDRESS="GCJIZYOR6VHGSUOXKHDZB62PNPAFTKSONNPLGVD57NXLQYTYF4TEGAKA"

# Một đường dẫn URL trỏ đến metadata của NFT (ví dụ: một tệp JSON trên IPFS)
METADATA_URL="https://example.com/nft/metadata/1.json"
# --- END CONFIGURATION ---

echo "Đang đúc (minting) một NFT mới..."

# Gọi hàm mint_nft
# Sửa lại tên tham số thành --metadata_url
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE_IDENTITY" \
  --network testnet \
  -- \
  mint_nft \
  --owner "$OWNER_ADDRESS" \
  --creator "$CREATOR_ADDRESS" \
  --metadata_url "$METADATA_URL"

echo "✅ Đã đúc NFT thành công!"
echo "Kết quả trả về ở trên sẽ hiển thị token_id mới (ví dụ: 1)."

