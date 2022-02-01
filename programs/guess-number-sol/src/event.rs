use anchor_lang::prelude::*;

#[event]
pub struct NewGame {
    pub game_id: String
}

#[event]
pub struct NewNumberGuessed {
    pub number_guessed: u32,
    pub guessed_by: Pubkey
}

#[event]
pub struct GameFinished {
    pub number_to_guess: u32,
    pub number_of_guessed_numbers: u32,
    pub winner: Pubkey,
    pub game_id: String
}