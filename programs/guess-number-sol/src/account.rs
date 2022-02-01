use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub id: String,
    pub authority: Pubkey,
    pub finished: bool,
    pub number_to_guess: [u8; 32],
    pub number_to_guess_test: u32, // for testing purposes
    pub winner: Option<Pubkey>,
    pub guessed_numbers: Vec<GuessedNumber>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GuessedNumber {
    pub number: u32,
    pub guessed_by: Pubkey
}