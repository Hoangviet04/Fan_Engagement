# StellarFan: Hợp đồng thông minh Tương tác với Người hâm mộ

Dự án này là một hợp đồng thông minh được xây dựng trên nền tảng Soroban của Stellar, được thiết kế để giúp các nhà sáng tạo (nghệ sĩ, nhạc sĩ, v.v.) kết nối với người hâm mộ của họ thông qua NFT (Non-Fungible Tokens) và đảm bảo họ nhận được tiền bản quyền một cách tự động và minh bạch.

## Mục tiêu

Trong nền kinh tế Web3, việc trao quyền cho các nhà sáng tạo là một mục tiêu quan trọng. Hợp đồng này giải quyết vấn đề đó bằng cách:

1. **Phát hành NFT:** Cho phép người sáng tạo phát hành các tác phẩm của họ dưới dạng NFT độc nhất trên blockchain.

2. **Quản lý quyền sở hữu:** Ghi lại và cho phép chuyển nhượng quyền sở hữu NFT một cách an toàn.

3. **Bản quyền tự động:** Cung cấp một cơ chế để mô phỏng việc thanh toán tiền bản quyền cho người sáng tạo mỗi khi NFT được giao dịch lại.

## Các tính năng chính

Hợp đồng có các chức năng cốt lõi sau:

* `initialize`: Khởi tạo hợp đồng với một địa chỉ quản trị viên (admin) duy nhất.

* `mint_nft`: Đúc (tạo) một NFT mới, gán chủ sở hữu, người tạo và một đường dẫn metadata.

* `transfer_nft`: Cho phép chủ sở hữu hiện tại chuyển NFT của họ cho một địa chỉ khác.

* `get_nft`: Truy vấn và xem thông tin chi tiết của một NFT cụ thể.

* `pay_royalty`: Mô phỏng việc thanh toán tiền bản quyền cho người tạo ra NFT.

* `set_royalty`: Cho phép admin thiết lập các thông số về tiền bản quyền.

## Yêu cầu

Để chạy và tương tác với dự án này, bạn cần cài đặt:

* [Ngôn ngữ lập trình Rust](https://www.rust-lang.org/tools/install)

* [Soroban CLI](https://www.google.com/search?q=https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup%23install-the-soroban-cli)

* Một tài khoản Stellar Testnet đã được nạp tiền bằng [Friendbot](https://www.google.com/search?q=https://laboratory.stellar.org/%23account-creator%3Fnetwork%3Dtest).

## Hướng dẫn sử dụng

Dự án này đi kèm với các tệp script `.sh` để giúp bạn dễ dàng tương tác với hợp đồng.

### 1. Triển khai Hợp đồng

Chạy script `deploy.sh` để biên dịch mã nguồn Rust thành Wasm và triển khai nó lên mạng Stellar Testnet. Script này sẽ tự động tạo một định danh (identity) có tên là `deployer` để ký các giao dịch.

./deploy.sh
Sau khi thành công, bạn sẽ nhận được một **Contract ID**. Hãy sao chép ID này để cập nhật vào các tệp script khác.

### 2. Khởi tạo Hợp đồng

Sau khi triển khai, bạn cần khởi tạo hợp đồng để thiết lập admin.

./initialize.sh
### 3. Đúc (Mint) một NFT

Sử dụng script `mint_nft.sh` để tạo ra một NFT mới. Bạn có thể thay đổi các thông số như `OWNER_ADDRESS` và `METADATA_URL` trong tệp.

./mint_nft.sh
### 4. Xem thông tin NFT

Để kiểm tra lại thông tin của NFT vừa được đúc.

./get_nft_details.sh
### 5. Chuyển nhượng NFT

Để chuyển NFT cho một người khác. Bạn cần tạo một ví Stellar mới và cập nhật địa chỉ người nhận trong tệp `transfer_nft.sh`.

./transfer_nft.sh
### 6. Thanh toán Tiền bản quyền

Mô phỏng việc người mua mới thanh toán tiền bản quyền cho người sáng tạo ban đầu.

./pay_royalty.sh
## Các tệp Script

* `deploy.sh`: Biên dịch và triển khai hợp đồng.

* `initialize.sh`: Khởi tạo hợp đồng với admin.

* `mint_nft.sh`: Đúc một NFT mới.

* `get_nft_details.sh`: Lấy thông tin chi tiết của một NFT.

* `transfer_nft.sh`: Chuyển NFT cho người khác.

* `pay_royalty.sh`: Mô phỏng thanh toán tiền bản quyền.
