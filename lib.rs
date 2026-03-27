#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct StudentClubContract;

#[contractimpl]
impl StudentClubContract {
    // 1. Hàm để sinh viên tham gia Câu lạc bộ (Mint thẻ thành viên)
    pub fn join_club(env: Env, student: Address, student_name: String) {
        // Yêu cầu sinh viên phải ký xác nhận giao dịch bằng ví của mình
        student.require_auth();

        // Lưu thông tin: Địa chỉ ví sinh viên -> Tên sinh viên vào blockchain
        env.storage().persistent().set(&student, &student_name);
    }

    // 2. Hàm kiểm tra xem một ví đã là thành viên của CLB chưa
    pub fn check_membership(env: Env, student: Address) -> bool {
        // Trả về 'true' nếu ví đã đăng ký, 'false' nếu chưa
        env.storage().persistent().has(&student)
    }
}
