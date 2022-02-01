use anchor_lang::prelude::*;
use crate::account::*;

#[derive(Accounts)]
#[instruction(id: String, _bump:u8)]
pub struct InitGame<'info> {
    pub authority: Signer<'info>,
    //Everytime you create a new account (account init), you need
    //to pay for it and to pay for it you need system_program 
    pub system_program: Program<'info, System>,
    #[account(init,
        seeds=[id.as_ref()],
        bump=_bump,
        payer=authority,
        space=9000
    )]
    pub game: Account<'info, Game>
}

#[derive(Accounts)]
pub struct GuessNumber<'info> {
    #[account(mut)] // If you'll modify the value in your code (the account), you need that
    pub game: Account<'info, Game>,
    pub guesser: Signer<'info>,
}

// #[derive(Accounts)]
// pub struct Debug {
// }