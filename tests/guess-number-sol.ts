import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { GuessNumberSol } from '../target/types/guess_number_sol';
import assert from 'assert';

describe('guess-number-sol', () => {

  async function setupProgram() {
    anchor.setProvider(anchor.Provider.env());

    const { SystemProgram } = anchor.web3;

    //@ts-ignore
    const program = anchor.workspace.GuessNumberSol as Program<GuessNumberSol>;
    const provider = anchor.Provider.env();

    //const baseAccount = anchor.web3.Keypair.generate();

    const gameId = "game_id";
    const [gameAccountPubKey, gameBump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(gameId)], program.programId);

    return {
      gameId,
      gameAccountPubKey,
      gameBump,
      program,
      SystemProgram,
      //baseAccount
    };
  }

  it('Game is initialized', async () => {
    const { program, gameBump, SystemProgram, gameAccountPubKey } = await setupProgram();

    let listener = program.addEventListener("NewGame", (event, slot) => {
      assert.equal(true, true);
    });

    const tx = await program.rpc.createNewGame(
      "game_id",
      gameBump,
      {
        accounts: {
          authority: program.provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
          game: gameAccountPubKey
        }
      },
    );

    program.removeEventListener(listener);

    //Game should be initialized with proper values
    const gameAccountResult = await program.account.game.fetch(gameAccountPubKey);

    assert.equal(gameAccountResult.finished, false);
    assert.equal(gameAccountResult.id, "game_id");
    assert.equal(gameAccountResult.winner, null);
    assert.deepEqual(gameAccountResult.authority, program.provider.wallet.publicKey);
    console.log(typeof gameAccountResult.guessedNumbers);
    assert.equal(gameAccountResult.guessedNumbers.length, 0);

    console.log("Your transaction signature", tx);
  });

  it('Non winning number is guessed', async () => {
    const { program, gameAccountPubKey } = await setupProgram();

    let listener = program.addEventListener("NewNumberGuessed", (event, slot) => {
      assert.equal(true, true);
    });

    const tx = await program.rpc.guessNumber(
      0,
      {
        accounts: {
          guesser: program.provider.wallet.publicKey,
          game: gameAccountPubKey
        }
      },
    );

    program.removeEventListener(listener);

    //Game should be initialized with proper values
    const gameAccountResult = await program.account.game.fetch(gameAccountPubKey);

    assert.equal(gameAccountResult.finished, false);
    assert.equal(gameAccountResult.winner, null);
    assert.equal(gameAccountResult.guessedNumbers.length, 1);
    
    console.log("Your transaction signature", tx);
  });

  it('Winning number is guessed', async () => {
    const { program, gameAccountPubKey } = await setupProgram();

    let listener = program.addEventListener("NewNumberGuessed", (event, slot) => {
      assert.equal(true, true);
    });

    let listener2 = program.addEventListener("GameFinished", (event, slot) => {
      assert.equal(true, true);
    });

    const tx = await program.rpc.guessNumber(
      (await program.account.game.fetch(gameAccountPubKey)).numberToGuessTest,
      {
        accounts: {
          guesser: program.provider.wallet.publicKey,
          game: gameAccountPubKey
        }
      },
    );

    program.removeEventListener(listener);
    program.removeEventListener(listener2);

    //Game should be initialized with proper values
    const gameAccountResult = await program.account.game.fetch(gameAccountPubKey);

    assert.equal(gameAccountResult.finished, true);
    assert.deepEqual(gameAccountResult.winner, program.provider.wallet.publicKey);
    assert.equal(gameAccountResult.guessedNumbers.length, 2);
    
    console.log("Your transaction signature", tx);
  });

});
