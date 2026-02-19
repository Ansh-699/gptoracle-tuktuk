use crate::{
    configuration::{AGENT_PROFILE_SEED, AGENT_PROMPT_TEXT},
    domain::AgentProfile,
    ID,
};
use anchor_lang::prelude::*;
use solana_gpt_oracle::ContextAccount;

#[derive(Accounts)]
pub struct RequestOracleInteractionWorkflow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub interaction: AccountInfo<'info>,

    #[account(
        seeds = [AGENT_PROFILE_SEED, payer.key().as_ref()],
        bump
    )]
    pub agent: Account<'info, AgentProfile>,

    #[account(address = agent.context)]
    pub context_acount: Account<'info, ContextAccount>,

    #[account(address = solana_gpt_oracle::ID)]
    pub oracle_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RequestOracleInteractionWorkflow<'info> {
    pub fn dispatch_interaction(&mut self) -> Result<()> {
        let callback_discriminator: [u8; 8] = crate::instruction::CallbackFromLlm::DISCRIMINATOR
            .try_into()
            .expect("Must be 8 bytes");

        solana_gpt_oracle::cpi::interact_with_llm(
            CpiContext::new(
                self.oracle_program.to_account_info(),
                solana_gpt_oracle::cpi::accounts::InteractWithLlm {
                    payer: self.payer.to_account_info(),
                    context_account: self.context_acount.to_account_info(),
                    interaction: self.interaction.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
            ),
            AGENT_PROMPT_TEXT.to_string(),
            ID,
            callback_discriminator,
            None,
        )?;

        Ok(())
    }
}