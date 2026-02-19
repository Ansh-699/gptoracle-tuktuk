use crate::faults::OracleWorkflowFault;
use anchor_lang::prelude::*;
use solana_gpt_oracle::Identity;

#[derive(Accounts)]
pub struct ReceiveOracleCallbackWorkflow<'info> {
    pub identity: Account<'info, Identity>,
}

impl<'info> ReceiveOracleCallbackWorkflow<'info> {
    pub fn capture_response(&mut self, _response: String) -> Result<()> {
        require!(
            self.identity.to_account_info().is_signer,
            OracleWorkflowFault::InvalidCallbackSigner
        );
        Ok(())
    }
}