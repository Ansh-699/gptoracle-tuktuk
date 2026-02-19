use anchor_lang::prelude::*;

#[error_code]
pub enum OracleWorkflowFault {
    #[msg("Invalid callback signer")]
    InvalidCallbackSigner,
}