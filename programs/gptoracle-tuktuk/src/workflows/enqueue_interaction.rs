use crate::{
    configuration::{AGENT_PROFILE_SEED, QUEUE_AUTHORITY_PDA_SEED},
    domain::AgentProfile,
};
use anchor_lang::{
    prelude::{instruction::Instruction, *},
    InstructionData,
};
use solana_gpt_oracle::ContextAccount;
use tuktuk_program::{
    compile_transaction,
    tuktuk::{cpi::queue_task_v0, program::Tuktuk},
    TransactionSourceV0, TriggerV0,
};

#[derive(Accounts)]
pub struct EnqueueInteractionWorkflow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: This account is created and validated by the oracle CPI during
    /// interaction creation; this workflow only forwards it.
    pub interaction: AccountInfo<'info>,

    #[account(
        seeds = [AGENT_PROFILE_SEED, payer.key().as_ref()],
        bump
    )]
    pub agent: Account<'info, AgentProfile>,

    #[account(address = agent.context)]
    pub context_account: Account<'info, ContextAccount>,

    #[account(mut)]
    /// CHECK: Tuktuk queue account validated by the tuktuk program during CPI.
    pub task_queue: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Tuktuk queue authority account validated by the tuktuk CPI.
    pub task_queue_authority: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Task account is created/validated by the tuktuk CPI.
    pub task: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [QUEUE_AUTHORITY_PDA_SEED],
        bump
    )]
    /// CHECK: PDA is constrained by seeds+bump and only used as a CPI signer.
    pub queue_authority: AccountInfo<'info>,
    pub tuktuk_program: Program<'info, Tuktuk>,
    pub system_program: Program<'info, System>,
}

impl<'info> EnqueueInteractionWorkflow<'info> {
    pub fn enqueue(&self, task_id: u16, bumps: &EnqueueInteractionWorkflowBumps) -> Result<()> {
        let interact_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(self.payer.key(), false),
                AccountMeta::new(self.interaction.key(), false),
                AccountMeta::new_readonly(self.agent.key(), false),
                AccountMeta::new_readonly(self.context_account.key(), false),
                AccountMeta::new_readonly(solana_gpt_oracle::ID, false),
                AccountMeta::new_readonly(System::id(), false),
            ],
            data: crate::instruction::InteractWithLlm {}.data(),
        };

        let (compiled_tx, _) = compile_transaction(vec![interact_ix], vec![]).unwrap();

        queue_task_v0(
            CpiContext::new_with_signer(
                self.tuktuk_program.to_account_info(),
                tuktuk_program::tuktuk::cpi::accounts::QueueTaskV0 {
                    payer: self.payer.to_account_info(),
                    queue_authority: self.queue_authority.to_account_info(),
                    task_queue: self.task_queue.to_account_info(),
                    task_queue_authority: self.task_queue_authority.to_account_info(),
                    task: self.task.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
                &[&[QUEUE_AUTHORITY_PDA_SEED, &[bumps.queue_authority]]],
            ),
            tuktuk_program::types::QueueTaskArgsV0 {
                id: task_id,
                trigger: TriggerV0::Now,
                transaction: TransactionSourceV0::CompiledV0(compiled_tx),
                crank_reward: Some(5_000_000),
                free_tasks: 0,
                description: "interact_with_llm".to_string(),
            },
        )?;
        Ok(())
    }
}