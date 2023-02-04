use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    //why there is two entrypoint?
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

//this is attributes
//cfg stands for config
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);


///three fn and all three use the ProgramResult
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    match PowerStatus::try_from_slice(&instruction_data) {
        Ok(power_status) => return initialize(program_id, accounts, power_status),
        Err(_) => {},
    }

    match SetPowerStatus::try_from_slice(&instruction_data) {
        Ok(set_power_status) => return switch_power(accounts, set_power_status.name),
        Err(_) => {},
    }

    Err(ProgramError::InvalidInstructionData)
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = (power_status.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    //invoke allows your program to call ix of other program
    invoke(
        &system_instruction::create_account(
            &user.key,
            &power.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            user.clone(), power.clone(), system_program.clone()
        ]
    )?;

    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}

pub fn switch_power(
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;

    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    power_status.is_on = !power_status.is_on;
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    msg!("{} is pulling the power switch!", &name);

    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    Ok(())
}


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool,
}

/*
What are the concepts (borrowing, ownership, vectors etc)
    this ons is using use borsh::{ BorshDeserialize, BorshSerialize };
What is the organization? How the code is organized?
    starts by laying the instructions in process_instruction
    then initializing said instructions, program_id and accounts still needed.


What is the contract doing? What is the mechanism?
    I think is a program that checks if the power status is on or off, but I don't see
    the bool to return false if is_on, maybe that can be an improvement?

How could it be better? More efficient? Safer?
    I'm not sure how it could be safer, maybe make it better by
    adding  power_status: PowerStatus into the switch_power function,
    now that I think about it, switch_power function uses PowerStatus::try_from_slice
    which is not in scope, I wonder if that could cause error
 */