#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env,
};

// ============================================================
// CONSTANTS
// ============================================================

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_TTL: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = 6 * DAY_IN_LEDGERS;
const PERSISTENT_TTL: u32 = 30 * DAY_IN_LEDGERS;
const PERSISTENT_THRESHOLD: u32 = 29 * DAY_IN_LEDGERS;

// ============================================================
// DATA TYPES
// ============================================================

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address), // Lưu trữ số dư ngày tập của từng người dùng
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotAuthorized = 3,
    InsufficientBalance = 4,
    InvalidAmount = 5,
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct GymMembershipToken;

#[contractimpl]
impl GymMembershipToken {
    /// Hàm khởi tạo — Gọi một lần khi deploy để thiết lập chủ phòng Gym
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
    }

    /// Lấy địa chỉ Admin
    fn get_admin(env: &Env) -> Result<Address, ContractError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ContractError::NotInitialized)
    }

    /// Đọc số dư ngày tập của một hội viên
    pub fn balance(env: Env, user: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    /// Admin cấp phát ngày tập (Ví dụ: mua gói 3 tháng = mint 90 ngày)
    pub fn mint(env: Env, admin: Address, to: Address, amount: u32) -> Result<(), ContractError> {
        if amount == 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Bắt buộc admin phải ký xác nhận
        admin.require_auth();

        let stored_admin = Self::get_admin(&env)?;
        if admin != stored_admin {
            return Err(ContractError::NotAuthorized);
        }

        let mut balance = Self::balance(env.clone(), to.clone());
        balance += amount;

        // Cập nhật số dư và gia hạn vòng đời dữ liệu
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &balance);
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(to.clone()),
            PERSISTENT_THRESHOLD,
            PERSISTENT_TTL,
        );

        // Phát sự kiện (Event) để hệ thống frontend/quản lý cập nhật
        env.events()
            .publish((symbol_short!("mint"), admin, to), amount);

        Ok(())
    }

    /// Hội viên chuyển nhượng (bán/tặng) ngày tập cho người khác
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: u32,
    ) -> Result<(), ContractError> {
        if amount == 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Bắt buộc người gửi phải ký xác nhận giao dịch
        from.require_auth();

        let mut from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(ContractError::InsufficientBalance);
        }

        let mut to_balance = Self::balance(env.clone(), to.clone());

        from_balance -= amount;
        to_balance += amount;

        // Cập nhật số dư cho người gửi
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &from_balance);
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(from.clone()),
            PERSISTENT_THRESHOLD,
            PERSISTENT_TTL,
        );

        // Cập nhật số dư cho người nhận
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &to_balance);
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(to.clone()),
            PERSISTENT_THRESHOLD,
            PERSISTENT_TTL,
        );

        env.events()
            .publish((symbol_short!("transfer"), from, to), amount);

        Ok(())
    }

    /// Hội viên đến phòng tập, quét mã và trừ đi 1 ngày
    pub fn check_in(env: Env, user: Address) -> Result<(), ContractError> {
        // Chủ thẻ phải ký bằng ví để check-in
        user.require_auth();

        let mut balance = Self::balance(env.clone(), user.clone());
        if balance < 1 {
            return Err(ContractError::InsufficientBalance);
        }

        balance -= 1;

        env.storage()
            .persistent()
            .set(&DataKey::Balance(user.clone()), &balance);
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(user.clone()),
            PERSISTENT_THRESHOLD,
            PERSISTENT_TTL,
        );

        env.events()
            .publish((symbol_short!("checkin"), user), 1_u32);

        Ok(())
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_gym_membership_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        let contract_id = env.register(
            GymMembershipToken,
            GymMembershipTokenArgs::__constructor(&admin),
        );
        let client = GymMembershipTokenClient::new(&env, &contract_id);

        // 1. Admin bán gói 3 tháng (90 ngày) cho user1
        client.mint(&admin, &user1, &90);
        assert_eq!(client.balance(&user1), 90);

        // 2. User1 bận, bán/chuyển nhượng 10 ngày cho user2
        client.transfer(&user1, &user2, &10);
        assert_eq!(client.balance(&user1), 80);
        assert_eq!(client.balance(&user2), 10);

        // 3. User1 đi tập 1 ngày (check-in)
        client.check_in(&user1);
        assert_eq!(client.balance(&user1), 79);
    }
}