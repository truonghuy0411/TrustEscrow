#![no_std]
#![allow(non_snake_case)]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env,
};

const DAY: u64 = 17280;

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectState {
    Created,
    InProgress,
    Disputed,
    Completed,
    Canceled,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Project {
    pub id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub totalAmount: i128,
    pub releasedAmount: i128,
    pub deadline: u64,
    pub state: ProjectState,
}

#[contracttype]
pub enum DataKey {
    Admin,
    TokenAddr,
    FeeBps,
    ProjectCount,
    Project(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAdmin = 1,
    InvalidState = 2,
    InvalidAmount = 3,
    NotAuthorized = 4,
    DeadlinePassed = 5,
    ProjectNotExpired = 6,
}

#[contract]
pub struct TrustEscrow;

#[contractimpl]
impl TrustEscrow {
    pub fn initialize(env: Env, admin: Address, tokenAddr: Address, feeBps: u32) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::TokenAddr, &tokenAddr);
        env.storage().instance().set(&DataKey::FeeBps, &feeBps);
        env.storage().instance().set(&DataKey::ProjectCount, &0_u64);
        env.storage()
            .instance()
            .extend_ttl(6 * DAY as u32, 7 * DAY as u32);
    }

    pub fn createProject(
        env: Env,
        client: Address,
        freelancer: Address,
        totalAmount: i128,
        durationInSeconds: u64,
    ) -> u64 {
        client.require_auth();
        if totalAmount <= 0 {
            panic!("Amount must be > 0");
        }

        let tokenAddr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
        let tokenClient = token::Client::new(&env, &tokenAddr);
        tokenClient.transfer(&client, &env.current_contract_address(), &totalAmount);

        let id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::ProjectCount)
            .unwrap_or(0)
            + 1;

        let current_time = env.ledger().timestamp();
        let deadline = current_time + durationInSeconds;

        let project = Project {
            id,
            client,
            freelancer,
            totalAmount,
            releasedAmount: 0,
            deadline,
            state: ProjectState::InProgress,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Project(id), &project);
        env.storage().instance().set(&DataKey::ProjectCount, &id);

        env.events()
            .publish((symbol_short!("create"), id), totalAmount);
        id
    }

    pub fn releaseMilestone(env: Env, projectId: u64, amount: i128) -> Result<(), Error> {
        let mut project: Project = env
            .storage()
            .persistent()
            .get(&DataKey::Project(projectId))
            .unwrap();
        project.client.require_auth();

        if project.state != ProjectState::InProgress {
            return Err(Error::InvalidState);
        }
        if amount <= 0 || project.releasedAmount + amount > project.totalAmount {
            return Err(Error::InvalidAmount);
        }

        if env.ledger().timestamp() > project.deadline {
            return Err(Error::DeadlinePassed);
        }

        project.releasedAmount += amount;
        if project.releasedAmount == project.totalAmount {
            project.state = ProjectState::Completed;
        }
        env.storage()
            .persistent()
            .set(&DataKey::Project(projectId), &project);

        let fee_bps: u32 = env.storage().instance().get(&DataKey::FeeBps).unwrap_or(0);
        let fee_amount = (amount * fee_bps as i128) / 10000;
        let freelancer_amount = amount - fee_amount;

        let tokenAddr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
        let tokenClient = token::Client::new(&env, &tokenAddr);
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();

        if fee_amount > 0 {
            tokenClient.transfer(&env.current_contract_address(), &admin, &fee_amount);
        }
        tokenClient.transfer(
            &env.current_contract_address(),
            &project.freelancer,
            &freelancer_amount,
        );

        env.events()
            .publish((symbol_short!("release"), projectId), amount);
        Ok(())
    }

    pub fn cancelAndRefund(env: Env, projectId: u64) -> Result<(), Error> {
        let mut project: Project = env
            .storage()
            .persistent()
            .get(&DataKey::Project(projectId))
            .unwrap();
        project.client.require_auth();

        if project.state != ProjectState::InProgress {
            return Err(Error::InvalidState);
        }

        if env.ledger().timestamp() < project.deadline {
            return Err(Error::ProjectNotExpired);
        }

        let remainingFunds = project.totalAmount - project.releasedAmount;
        if remainingFunds > 0 {
            let tokenAddr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
            let tokenClient = token::Client::new(&env, &tokenAddr);
            tokenClient.transfer(
                &env.current_contract_address(),
                &project.client,
                &remainingFunds,
            );
        }

        project.state = ProjectState::Canceled;
        env.storage()
            .persistent()
            .set(&DataKey::Project(projectId), &project);
        Ok(())
    }
}
