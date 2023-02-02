//https://www.youtube.com/watch?v=O0uhZEfVPt8
///To me this is the skeleton of what a basic solana program includes, I'm not sure why the file
/// name is lib when in the paul x escrow and the solana bootcamp lecture is under processor
/// so the AccountInfo, ProgramResult, ProgramError, Pubkey are objects in the solana program
///
use solana_program::{
    //Every invocation of a solana_program executes a single instruction, in this case
    //each line is a vector, and then inside each vector, there are ?
    account_info::{ AccountInfo, next_account_info },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

//this is a macro that got exported in the solana_program above
entrypoint!(process_instruction);

//it looks like the same format on the bootcamp video at 28:30, difference is the code is located
//in entrypoint.rs
fn process_instruction(
    program_id: &Pubkey,        //identifies the program
    accounts: &[AccountInfo],   //Data to read / write in a list of data buffers
    _instruction_data: &[u8],   // parameter input or bytes that specify which IX to execute
) -> ProgramResult {

    // You can verify the program ID from the instruction is in fact
    //      the program ID of your program.
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can verify the list has the correct number of accounts.
    // This error will get thrown by default if you
    //      try to reach past the end of the iter.
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Accounts passed in a vector must be in the expected order.
    //  Where is the account vector order?
    //  Accounts info for the program being invoke
    let accounts_iter = &mut accounts.iter();
    //on the video 34:20, it represents _payer as an AccountInfo object, which contains
    //information about the account, lmports etc
    //so in a way, there will be four accounts: _payer, account_to_create, account_to_change,
    //system_program since all of them have (accounts_iter)
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.

    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (Create account...)

    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    //I was wondering why it left these two last since both have the same error format ProgramError::IncorrectProgramId
    //  then I realized it is because account_to_change and system_program are not created yet.
    //  Perhaps at the first error check, is checking for program_id
    //  and system_program.key is not yet available until it iterates through the accounts vector
    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    Ok(())
}

/*
What are the concepts (borrowing, ownership, vectors etc)
    solana_program is an instruction and inside there are vectors

What is the organization?, How the code is organized?

What is the contract doing? What is the mechanism?
    The program is processing the instructions provided by the entrypoint of what is connected
    with ProgramResult.

    This program requires four accounts, if there are not present then it will throw NotEnoughAccountKeys
    error.

How could it be better? More efficient? Safer?
    I think the way I've seen this layout has been where there is a
    lib.rs file and inside are public modules
    pub mod entrypoint;
    pub mod error;
    pub mod instruction;
    pub mod processor;
    pub mod state;

    and the the program code (like this one) is in processo.rs

The code could be safer and better if…..
 */