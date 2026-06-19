#![no_std]

//! # Council Dues Smart Contract
//!
//! A Soroban smart contract that manages annual professional/industry
//! council dues for members of a real estate industry council. Members
//! pay an annual fee, the contract records the payment year, and members
//! whose dues have lapsed lose voting rights. Admins can verify the
//! current standing of any member and revoke/restore membership status
//! in case of disciplinary actions or reinstatement.

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, Symbol, Vec};

/// Storage key for the contract administrator address.
const ADMIN: Symbol = symbol_short!("ADMIN");
/// Storage key for the annual fee schedule: `Map<u32, i128>` year -> amount.
const FEE_SCHEDULE: Symbol = symbol_short!("FEES");
/// Storage key prefix for a member's paid-years set: `Map<Address, Vec<u32>>`.
const PAID_YEARS: Symbol = symbol_short!("PAID");
/// Storage key prefix for a member's revocation record: `Map<Address, Symbol>`.
const REVOKED: Symbol = symbol_short!("REVOKED");

#[contract]
pub struct CouncilDues;

#[contractimpl]
impl CouncilDues {
    /// Initialize the contract by storing the admin address.
    /// Must be called exactly once before any other function.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Contract already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
    }

    /// Set (or update) the annual council fee for a specific `year`.
    /// Only the admin can change the fee schedule.
    pub fn set_annual_fee(env: Env, admin: Address, year: u32, amount: i128) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);

        if year < 2024 || year > 2100 {
            panic!("Year out of supported range");
        }
        if amount <= 0 {
            panic!("Fee must be positive");
        }

        let mut fees: Map<u32, i128> = env
            .storage()
            .instance()
            .get(&FEE_SCHEDULE)
            .unwrap_or(Map::new(&env));
        fees.set(year, amount);
        env.storage().instance().set(&FEE_SCHEDULE, &fees);
    }

    /// Record payment of the annual council fee for `member` for `year`.
    /// The member must authorize the call. A member cannot pay the same
    /// year twice, and a revoked member cannot pay until reinstated.
    pub fn pay(env: Env, member: Address, year: u32) {
        member.require_auth();

        if env.storage().instance().has(&REVOKED) {
            let revoked: Map<Address, Symbol> = env
                .storage()
                .instance()
                .get(&REVOKED)
                .unwrap_or(Map::new(&env));
            if revoked.get(member.clone()).is_some() {
                panic!("Membership revoked; must be restored first");
            }
        }

        let fees: Map<u32, i128> = env
            .storage()
            .instance()
            .get(&FEE_SCHEDULE)
            .unwrap_or(Map::new(&env));
        if fees.get(year).unwrap_or(0) <= 0 {
            panic!("Fee not configured for given year");
        }

        let mut paid: Map<Address, Vec<u32>> = env
            .storage()
            .instance()
            .get(&PAID_YEARS)
            .unwrap_or(Map::new(&env));
        let mut years = paid.get(member.clone()).unwrap_or(Vec::new(&env));
        for y in years.iter() {
            if y == year {
                panic!("Member already paid for this year");
            }
        }
        years.push_back(year);
        paid.set(member, years);
        env.storage().instance().set(&PAID_YEARS, &paid);
    }

    /// Revoke a member's council membership for a given `reason`.
    /// Only the admin can revoke. A revoked member loses voting rights
    /// and is blocked from paying future dues until restored.
    pub fn revoke(env: Env, admin: Address, member: Address, reason: Symbol) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);

        let mut revoked: Map<Address, Symbol> = env
            .storage()
            .instance()
            .get(&REVOKED)
            .unwrap_or(Map::new(&env));
        if revoked.get(member.clone()).is_some() {
            panic!("Member already revoked");
        }
        revoked.set(member, reason);
        env.storage().instance().set(&REVOKED, &revoked);
    }

    /// Restore a previously revoked member's council membership.
    /// Only the admin can restore. Clears the revocation record.
    pub fn restore(env: Env, admin: Address, member: Address) {
        admin.require_auth();
        Self::assert_admin(&env, &admin);

        let mut revoked: Map<Address, Symbol> = env
            .storage()
            .instance()
            .get(&REVOKED)
            .unwrap_or(Map::new(&env));
        if revoked.get(member.clone()).is_none() {
            panic!("Member is not revoked");
        }
        revoked.remove(member);
        env.storage().instance().set(&REVOKED, &revoked);
    }

    /// Returns `true` if `member` is in good standing for the given
    /// `year` (i.e. has paid the annual fee and is not revoked).
    pub fn is_current(env: Env, member: Address, year: u32) -> bool {
        if env.storage().instance().has(&REVOKED) {
            let revoked: Map<Address, Symbol> = env
                .storage()
                .instance()
                .get(&REVOKED)
                .unwrap_or(Map::new(&env));
            if revoked.get(member.clone()).is_some() {
                return false;
            }
        }

        let paid: Map<Address, Vec<u32>> = env
            .storage()
            .instance()
            .get(&PAID_YEARS)
            .unwrap_or(Map::new(&env));
        match paid.get(member) {
            Some(years) => {
                for y in years.iter() {
                    if y == year {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }

    /// Returns the number of distinct years a `member` has paid dues for.
    pub fn paid_years(env: Env, member: Address) -> u32 {
        let paid: Map<Address, Vec<u32>> = env
            .storage()
            .instance()
            .get(&PAID_YEARS)
            .unwrap_or(Map::new(&env));
        match paid.get(member) {
            Some(years) => years.len(),
            None => 0,
        }
    }

    /// Read the annual fee configured for a given `year`.
    /// Returns 0 if no fee has been set.
    pub fn get_annual_fee(env: Env, year: u32) -> i128 {
        let fees: Map<u32, i128> = env
            .storage()
            .instance()
            .get(&FEE_SCHEDULE)
            .unwrap_or(Map::new(&env));
        fees.get(year).unwrap_or(0)
    }

    // --- internal helpers ---

    fn assert_admin(env: &Env, admin: &Address) {
        let stored: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .expect("Contract not initialized");
        if stored != *admin {
            panic!("Caller is not the admin");
        }
    }
}
