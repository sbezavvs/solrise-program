use solana_program::{
    account_info::{self, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the struct for the crowdfunding project state
#[account]
#[derive(Default)]
pub struct CrowdfundingProject {
    pub project_id: u64,
    pub name: String,
    pub funding_goal: u64,
    pub funding_deadline: i64,
    pub project_address: Pubkey,
    pub stages: Vec<Stage>,
}

impl CrowdfundingProject {
    // Initialize a new crowdfunding project
    pub fn new(owner: Pubkey, target_amount: u64) -> Self {
        CrowdfundingProject {
            owner,
            target_amount,
            current_amount: 0,
        }
    }

    // Add funds to the crowdfunding project
    pub fn add_funds(&mut self, amount: u64) {
        self.current_amount += amount;
    }

    // Check if the crowdfunding project is fully funded
    pub fn is_fully_funded(&self) -> bool {
        self.current_amount >= self.target_amount
    }
}