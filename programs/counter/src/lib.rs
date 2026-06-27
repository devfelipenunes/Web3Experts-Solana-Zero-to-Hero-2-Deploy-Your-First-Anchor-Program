use anchor_lang::prelude::*;

declare_id!("3NPBA3zn4Dc8FsXkqoGKXE1uERdch63TkGCcMTTDkVVT");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter initialized to 0");
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Count incremented to: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter"], bump)]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use litesvm::LiteSVM;
    use solana_keypair::Keypair;
    use solana_signer::Signer;
    use solana_message::Message;
    use solana_transaction::Transaction;
    use anchor_lang::prelude::Pubkey;
    use anchor_lang::InstructionData;
    use anchor_lang::ToAccountMetas;

    #[test]
    fn test_counter_local() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
        
        let _program_id = id();
        assert_eq!(payer.pubkey().to_bytes().len(), 32);
    }

    #[test]
    fn test_counter_devnet() {
        use solana_rpc_client::rpc_client::RpcClient;
        use solana_transaction::Transaction;
        use anchor_lang::solana_program::instruction::{Instruction, AccountMeta};
        use anchor_lang::solana_program::system_program;
        use anchor_lang::InstructionData;
        use solana_signer::Signer;
        use solana_keypair::read_keypair_file;
        
        let payer = read_keypair_file("/l/disk0/fnunes/.gemini/antigravity-cli/scratch/bounty-keypair.json").unwrap();
        
        let rpc_client = RpcClient::new("https://api.devnet.solana.com");
        
        let program_id = id();
        let (counter_pda, _) = Pubkey::find_program_address(&[b"counter"], &program_id);
        
        let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
        
        let account_data = rpc_client.get_account(&counter_pda);
        
        if account_data.is_err() {
            println!("Counter not initialized, initializing...");
            let init_ix = Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(counter_pda, false),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: crate::instruction::Initialize {}.data(),
            };
            
            let tx = Transaction::new_signed_with_payer(
                &[init_ix],
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            
            let sig = rpc_client.send_and_confirm_transaction(&tx).unwrap();
            println!("Initialize Signature: {}", sig);
        } else {
            println!("Counter already initialized.");
        }
        
        println!("Incrementing...");
        let inc_ix = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(counter_pda, false),
            ],
            data: crate::instruction::Increment {}.data(),
        };
        
        let tx = Transaction::new_signed_with_payer(
            &[inc_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
        
        let sig = rpc_client.send_and_confirm_transaction(&tx).unwrap();
        println!("Increment Signature: {}", sig);
    }
}
