use anchor_lang::prelude::*;

#[constant]
pub const AGENT_PROFILE_SEED: &[u8] = b"agent";
pub const AGENT_PROMPT_TEXT: &str = "Rust is love";
pub const QUEUE_AUTHORITY_PDA_SEED: &[u8] = b"queue_authority";