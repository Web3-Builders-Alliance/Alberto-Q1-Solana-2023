//new modules. similar to when calling clock
use borsh::BorshDeserialize;
use lever::SetPowerStatus;
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{ AccountMeta, Instruction },
    pubkey::Pubkey,
    program::invoke,
};


entrypoint!(pull_lever);


fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    ///these variables might be found in another folder the same order
    /// accounts_iter, power, lever_program, set_power_status_instruction, ix
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let lever_program = next_account_info(accounts_iter)?;

    let set_power_status_instruction = SetPowerStatus::try_from_slice(instruction_data)?;

    let ix = Instruction::new_with_borsh(
        lever_program.key.clone(),                          // Our lever program's ID
        &set_power_status_instruction,                      // Passing instructions through
        vec![AccountMeta::new(power.key.clone(), false)],   // Just the required account for the other program
    );

    invoke(&ix, &[power.clone()])
}
/*
What are the concepts (borrowing, ownership, vectors etc)
    Two new concepts are borsh and lever

What is the organization? How the code is organized?
    The code is organized as one single function called pull lever,
    I can think this is just to check if the lever is true or false
    but I don't see the boolean to turn it opposite as it currently is
What is the contract doing? What is the mechanism?
    checking if a lever is true or false, althoug I don't see the
    ix that will turn the boolean true or false

How could it be better? More efficient? Safer?
    better I guess adding a second ix so the program can change the lever
*/