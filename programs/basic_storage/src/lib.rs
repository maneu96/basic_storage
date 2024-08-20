use anchor_lang::prelude::*;
use std::mem::size_of;
declare_id!("6EvbUStiPLFAoGxVXx3pMFGokNw4daay39ZXtZyMm19f");

#[program]
pub mod basic_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.my_storage.x = 0;
        ctx.accounts.my_storage.y = 0;
        msg!("(x,y) = ({},{})", ctx.accounts.my_storage.x, ctx.accounts.my_storage.y);
        Ok(())
    }

    pub fn change_coordinates(ctx: Context<Set> , x: i64, y: i64) -> Result<()> {
        ctx.accounts.my_storage.x = x;
        ctx.accounts.my_storage.y = y;
        msg!("(x,y) = ({},{})", ctx.accounts.my_storage.x, ctx.accounts.my_storage.y);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
            payer = signer, //payer is the wallet corresponding to the signer
            space = size_of::<MyStorage>() + 8, // The storage space required is the size of MyStorage struct + 8, 
            seeds = [], // which corresponds to a 8 byte discriminator, which is calculated with a seed, which then iterates through multiple accounts that the program can own.
            bump)]
    pub my_storage: Account<'info, MyStorage>, // initialization of struct of type MyStorage.
    
    #[account(mut)]
    pub signer: Signer<'info>, // the wallet that is paying for the gas for storage of the struct my_storage.

    pub system_program: Program<'info, System>, // A program built into the solana runtime that transfers SOL from one account to another.
                                                // This will allow, in this case to transfer SOL away from the signer
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}


#[account]
pub struct MyStorage {
    x: i64,
    y: i64
}

