use crate::{
    configuration::{AGENT_PROFILE_SEED, AGENT_PROMPT_TEXT},
    domain::AgentProfile,
};
use anchor_lang::prelude::*;
use solana_gpt_oracle::{cpi::create_llm_context, Counter};

#[derive(Accounts)]
pub struct BootstrapAgentWorkflow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        seeds = [AGENT_PROFILE_SEED, payer.key().as_ref()],
        space = 8 + 32 + 1,
        bump
    )]
    pub agent: Account<'info, AgentProfile>,

    #[account(mut)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    /// CHECK: This account is initialized and validated by the `solana-gpt-oracle`
    /// program in the `create_llm_context` CPI call.
    pub llm_context: AccountInfo<'info>,
    /// CHECK: Address is constrained to `solana_gpt_oracle::ID`, so this points
    /// to the expected oracle program used for CPI.
    #[account(address = solana_gpt_oracle::ID)]
    pub oracle_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> BootstrapAgentWorkflow<'info> {
    pub fn bootstrap(&mut self, bumps: &BootstrapAgentWorkflowBumps) -> Result<()> {
        self.agent.set_inner(AgentProfile {
            context: self.llm_context.key(),
            bump: bumps.agent,
        });

        create_llm_context(
            CpiContext::new(
                self.oracle_program.to_account_info(),
                solana_gpt_oracle::cpi::accounts::CreateLlmContext {
                    payer: self.payer.to_account_info(),
                    counter: self.counter.to_account_info(),
                    context_account: self.llm_context.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
            ),
            AGENT_PROMPT_TEXT.to_string(),
        )?;

        Ok(())
    }
}