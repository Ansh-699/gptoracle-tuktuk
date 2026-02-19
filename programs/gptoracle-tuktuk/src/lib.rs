#![allow(deprecated, unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod configuration;
pub mod domain;
pub mod faults;
pub mod workflows;

use workflows::*;

declare_id!("4ac78uTsz4YZt3ZKVntFbiQTequg7twtZjgAAv41KMxk");

#[program]
pub mod tuktuk_gpt_oracle {
    use super::*;

    pub fn initialize(ctx: Context<BootstrapAgentWorkflow>) -> Result<()> {
        ctx.accounts.bootstrap(&ctx.bumps)
    }

    pub fn callback_from_llm(ctx: Context<ReceiveOracleCallbackWorkflow>, response: String) -> Result<()> {
        ctx.accounts.capture_response(response)
    }

    pub fn interact_with_llm(ctx: Context<RequestOracleInteractionWorkflow>) -> Result<()> {
        ctx.accounts.dispatch_interaction()
    }

    pub fn schedule(ctx: Context<EnqueueInteractionWorkflow>, task_id: u16) -> Result<()> {
        ctx.accounts.enqueue(task_id, &ctx.bumps)
    }
}
