use anchor_lang::prelude::*;

#[account]
pub struct AgentProfile {
    pub context: Pubkey,
    pub bump: u8,
}