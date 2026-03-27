# clubstudent
# Title
Student Club Membership (Thẻ Thành Viên Câu Lạc Bộ Sinh Viên)

# Description
Dự án này giúp số hóa và tự động hóa quá trình đăng ký tham gia câu lạc bộ của sinh viên thông qua công nghệ blockchain Stellar (Soroban). Thay vì dùng giấy tờ hay file Excel dễ bị thất lạc/chỉnh sửa, mỗi sinh viên khi tham gia sẽ dùng ví của mình để xác nhận và lưu trữ thông tin trực tiếp trên chuỗi (on-chain). Điều này giúp minh bạch hóa danh sách thành viên, chống giả mạo và tạo nền tảng vững chắc để phát triển các tính năng quản lý câu lạc bộ sau này.

# Tính năng 
Dự án hiện tại bao gồm các tính năng cốt lõi (Smart Contract):
- **Tham gia CLB (`join_club`):** Cho phép sinh viên đăng ký tham gia bằng cách liên kết địa chỉ ví Freighter của họ với họ tên thật, sau đó lưu trữ vĩnh viễn dữ liệu này lên blockchain. Yêu cầu chính chủ ký xác nhận.
- **Kiểm tra trạng thái (`check_membership`):** Tra cứu nhanh xem một địa chỉ ví bất kỳ đã chính thức là thành viên của câu lạc bộ hay chưa (trả về kết quả True/False).

# Contract
- **Contract ID:** `[DÁN MÃ CONTRACT ID CỦA BẠN VÀO ĐÂY]`
- **Stellar Expert Link:** [https://stellar.expert/explorer/testnet/contract/[DÁN MÃ CONTRACT ID CỦA BẠN VÀO ĐÂY]](https://stellar.expert/explorer/testnet/contract/[DÁN MÃ CONTRACT ID CỦA BẠN VÀO ĐÂY])

### Ảnh chụp màn hình thực tế:
*(Xóa dòng chữ in nghiêng này đi và dán ảnh của bạn vào. Nếu dùng Github, bạn bấm vào hình cây bút chì để sửa bài, sau đó bấm Ctrl + V (hoặc Command + V trên Mac) để dán ảnh trực tiếp vào đây nhé)*

1. **Ảnh chụp màn hình Deploy Contract thành công:**
[DÁN ẢNH 1 VÀO ĐÂY]

2. **Ảnh chụp màn hình Invoke Contract (tương tác) thành công:**
[DÁN ẢNH 2 VÀO ĐÂY]

# Future scopes
Trong tương lai (Milestone 2 và xa hơn), dự án có thể được mở rộng với các tính năng:
- **Điểm danh On-chain:** Dùng thẻ thành viên (ví) để điểm danh check-in tại các buổi họp mặt sự kiện của CLB.
- **Hệ thống điểm thưởng (Loyalty Token):** Tự động phát token thưởng cho các sinh viên hoạt động tích cực, token này có thể đổi lấy quà lưu niệm của trường.
- **Quản trị DAO:** Cho phép các thành viên đã xác thực được quyền bỏ phiếu cho các hoạt động hoặc nhân sự ban chủ nhiệm CLB.
- **Xây dựng Giao diện Frontend:** Tạo một website trực quan với nút "Connect Wallet" để sinh viên dễ dàng thao tác mà không cần dùng dòng lệnh Terminal.

# Profile
- **Tên / Nickname:** [ĐIỀN TÊN CỦA BẠN VÀO ĐÂY]
- **Kỹ năng:** Sinh viên IT / Đang tìm hiểu về Blockchain / Stellar Soroban / ... [GHI THÊM NẾU CÓ]
- **Github:** [LINK GITHUB CỦA BẠN]
