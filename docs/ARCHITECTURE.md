# Tuktuk GPT Oracle - Architecture

## Overview

Tuktuk GPT Oracle coordinates interactions with a remote LLM oracle, allowing on-chain agents to request AI-generated responses. Integrates with Solana GPT Oracle for inference and Tuktuk for scheduled autonomous callbacks.

![Architecture Diagram](./assets/tuktuk-gptoracle.png)

## Accounts

| Account | Purpose |
|---------|----------|
| **Agent Profile PDA** | Stores agent state and context reference |
| **LLM Context** | Persistent prompt/history on Oracle program |
| **Oracle Program** | External oracle for LLM processing |
| **Interaction Account** | Single request/response pair managed by Oracle |
| **Queue Authority PDA** | Signs Tuktuk scheduler tasks |

## User Stories

### Initialize Agent with LLM Context
As a developer, I need to bootstrap an on-chain agent with LLM context.
- Agent Profile PDA is created and linked
- LLM Context initialized on Oracle via CPI
- Agent ready to process interactions

### Request Real-Time LLM Response
As an operator, I need to submit queries and receive LLM responses on-chain.
- Interaction request sent to Oracle via CPI
- Response stored in Interaction account
- Callback discriminator routes response correctly

### Process Asynchronous Oracle Responses
As a program developer, I need to handle LLM responses asynchronously.
- Receive response data from Oracle
- Validate and store in agent state
- Trigger downstream logic based on content

### Schedule Deferred Agent Interactions
As an operator, I need autonomous scheduled interactions without manual payer trigger.
- Schedule instruction enqueues with Tuktuk
- Task executes at specified time
- Oracle processes request automatically
- No payer action required after scheduling

## Core Instructions

| Instruction | Action |
|------------|--------|
| **Initialize** | Create Agent Profile, init LLM Context via Oracle |
| **Interact** | Send query to Oracle, get Interaction account |
| **Callback** | Process response from Oracle |
| **Schedule** | Enqueue deferred interaction with Tuktuk |

## Technology Stack

- **Blockchain**: Solana
- **Framework**: Anchor 0.32.1
- **Oracle**: Solana GPT Oracle
- **Scheduler**: Tuktuk v0.3.2
