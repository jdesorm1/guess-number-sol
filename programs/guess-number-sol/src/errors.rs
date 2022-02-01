use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("This number was already guessed. Try again.")]
    AlreadyGuessed,

    #[msg("This game is finished. Create a new game.")]
    GameFinished
}