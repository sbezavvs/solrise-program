use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

#[program]
pub mod solrise {
    use super::*;

    // Creates a new crowdfunding project
    pub fn create_project(
        ctx: Context<CreateProject>,
        project_id: u64,
        name: String,
        funding_goal: u64,
        funding_deadline: i64,
        address: Pubkey,
    ) -> Result<()> {
        instructions::create_project(ctx, id, name, funding_goal, funding_deadline, address)
    }

    // Send a donation to Solrise's escrow account with the project id
    pub fn donate(
        ctx: Context<Donate>, 
        project_id: u64, 
        origin_address: Pubkey,
        amount: u64
    ) -> Result<()> {
        instructions::donate(ctx, project_id, amount)
    }

    
}
