use std::convert::TryInto;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::*;

mod errors;
mod context;
mod account;
mod event;

use account::*;
use errors::*;
use context::*;
use event::*;

//Always update or test won't work
declare_id!("CabdhrHZ3qDuwB6HDkda41SZmiSR3iXE5y1R52UZP6y7");


#[program]
pub mod guess_number_sol {

    use super::*;

    pub fn create_new_game(ctx: Context<InitGame>, id: String, _bump:u8) -> ProgramResult {
        let game = &mut ctx.accounts.game;

        game.id = id.clone();
        game.finished = false;
        game.authority = ctx.accounts.authority.key();

        let number_to_guess = get_random_u32();

        game.number_to_guess_test = number_to_guess;
        game.number_to_guess = hash(&number_to_guess.to_be_bytes()).to_bytes();
        game.guessed_numbers = vec![];

        emit!(NewGame {
            game_id: id.clone()
        });

        Ok(())
    }

    pub fn guess_number(ctx: Context<GuessNumber>, number: u32) -> ProgramResult {
        let game = &mut ctx.accounts.game;

        if game.finished {
            return Err(ErrorCode::GameFinished.into())
        }

        if game.guessed_numbers.iter().any(|n| n.number == number) {
            return Err(ErrorCode::AlreadyGuessed.into())
        }

        let guessed_number = GuessedNumber{
            number: number,
            guessed_by: ctx.accounts.guesser.key()
        };

        game.guessed_numbers.push(guessed_number.clone());

        emit!(NewNumberGuessed {
            number_guessed: number,
            guessed_by: guessed_number.guessed_by.key()
        });

        let hashed_number_to_guess = Hash::new_from_array(game.number_to_guess);

        if hash(&guessed_number.number.to_be_bytes()) == hashed_number_to_guess {
            game.finished = true;
            game.winner = Some(guessed_number.guessed_by.key());

            emit!(GameFinished{
                game_id: game.id.clone(),
                number_of_guessed_numbers: game.guessed_numbers.len() as u32,
                number_to_guess: number,
                winner: guessed_number.guessed_by.key()
            });
        }

        Ok(())
    }

    // pub fn debug(ctx: Context<Debug>) -> ProgramResult {
    //     //msg!("{:?}", ctx.accounts.authority.key());

    //     //msg!("{:?}", ctx.accounts.game.key());

    //     //msg!("{}", &get_random_u32());

    //     Ok(())
    // }
}

pub fn get_random_u32() -> u32 {
    let clock = Clock::get().unwrap();
    
    let slice = &hash(&clock.slot.to_be_bytes()).to_bytes()[0..4];
    
    let random_number: u32 = u32::from_be_bytes(slice.try_into().unwrap());

    return random_number;
}