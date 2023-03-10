
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    clock::Clock,
};

use spl_token::state::Account as TokenAccount;

//crate is bringing in state.rs and inside there is a pub struct Escrow
//  and inside pub struct Escrow there is a pub is_initialized: bool,
//  which is used here down below
use crate::{error::EscrowError, instruction::EscrowInstruction, state::Escrow};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            //the instruction for Alice to initialize the escrow account
            //and Bob to process the exchage?
            //InitEscrow and Exchange are located in instruction.rs
            //any new instruction added in instruction.rs muts also be added here
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, amount, program_id)
            }
            EscrowInstruction::Exchange { amount } => {
                msg!("Instruction: Exchange");
                Self::process_exchange(accounts, amount, program_id)
            }
            //adding ResetTimeLock and Cancel
            //then you will need to create fn process_reset_time_lock
            EscrowInstruction::ResetTimeLock { } => {
                msg!("Instruction: Reset time");
                Self::process_reset_time_lock(accounts, program_id)
            }
            EscrowInstruction::Cancel { } => {
                msg!("Instruction: Cancel");
                Self::process_cancel(accounts, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        //initializer is Alice
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let temp_token_account = next_account_info(account_info_iter)?;

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }


        //variables are added before escrow_info to follow the correct order of state.rs
        let clock = Clock::get()?.slot;
        let unlock_time: u64 = clock + 100;
        let time_out: u64 = unlock_time + 1000;

        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey = *initializer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;
        //new variables added
        escrow_info.unlock_time = unlock_time;
        escrow_info.time_out = time_out;

        Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;



        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }

    fn process_exchange(
        accounts: &[AccountInfo],
        amount_expected_by_taker: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        //taker is Bob
        let taker = next_account_info(account_info_iter)?;

        if !taker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let takers_sending_token_account = next_account_info(account_info_iter)?;

        let takers_token_to_receive_account = next_account_info(account_info_iter)?;

        let pdas_temp_token_account = next_account_info(account_info_iter)?;
        let pdas_temp_token_account_info =
            TokenAccount::unpack(&pdas_temp_token_account.try_borrow_data()?)?;
        let (pda, nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        if amount_expected_by_taker != pdas_temp_token_account_info.amount {
            return Err(EscrowError::ExpectedAmountMismatch.into());
        }

        let initializers_main_account = next_account_info(account_info_iter)?;
        let initializers_token_to_receive_account = next_account_info(account_info_iter)?;
        let escrow_account = next_account_info(account_info_iter)?;

        let escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;

        if escrow_info.temp_token_account_pubkey != *pdas_temp_token_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        if escrow_info.initializer_pubkey != *initializers_main_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        if escrow_info.initializer_token_to_receive_account_pubkey
            != *initializers_token_to_receive_account.key
        {
            return Err(ProgramError::InvalidAccountData);
        }

        //adding if statement
        let current_slot = Clock::get()?.slot;
        if !(current_slot >= escrow_info.unlock_time && current_slot <= escrow_info.time_out) {
            //need to create new Timelock Err in the error.rs
            return Err(EscrowError::Timelock.into());
        }

        let token_program = next_account_info(account_info_iter)?;

        let transfer_to_initializer_ix = spl_token::instruction::transfer(
            token_program.key,
            takers_sending_token_account.key,
            initializers_token_to_receive_account.key,
            taker.key,
            &[&taker.key],
            escrow_info.expected_amount,
        )?;
        msg!("Calling the token program to transfer tokens to the escrow's initializer...");
        invoke(
            &transfer_to_initializer_ix,
            &[
                takers_sending_token_account.clone(),
                initializers_token_to_receive_account.clone(),
                taker.clone(),
                token_program.clone(),
            ],
        )?;

        let pda_account = next_account_info(account_info_iter)?;

        let transfer_to_taker_ix = spl_token::instruction::transfer(
            token_program.key,
            pdas_temp_token_account.key,
            takers_token_to_receive_account.key,
            &pda,
            &[&pda],
            pdas_temp_token_account_info.amount,
        )?;
        msg!("Calling the token program to transfer tokens to the taker...");
        invoke_signed(
            &transfer_to_taker_ix,
            &[
                pdas_temp_token_account.clone(),
                takers_token_to_receive_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[nonce]]],
        )?;

        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pdas_temp_token_account.key,
            initializers_main_account.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pdas_temp_token_account.clone(),
                initializers_main_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[nonce]]],
        )?;

        msg!("Closing the escrow account...");
        **initializers_main_account.try_borrow_mut_lamports()? = initializers_main_account
            .lamports()
            .checked_add(escrow_account.lamports())
            .ok_or(EscrowError::AmountOverflow)?;
        **escrow_account.try_borrow_mut_lamports()? = 0;
        *escrow_account.try_borrow_mut_data()? = &mut [];

        Ok(())
    }

    ///must be called by the initiator and load the escrow state
    ///
    fn process_reset_time_lock(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let escrow_account = next_account_info(account_info_iter)?;
        if escrow_account.owner != program_id || escrow_account.is_writable == false {
            return Err(ProgramError::IllegalOwner);
        }
        //unpacking will allow to check the escrow
        let mut escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;

        if escrow_info.initializer_pubkey != *initializer.key {
            return Err(ProgramError::InvalidAccountData);
        }

        //reset the values
        escrow_info.unlock_time = 0;
        escrow_info.time_out = 0;

        Ok(())

    }
///most of the times you would want program_id , just ti make sure as the
/// functions run, you are talking to the right program.
    fn process_cancel(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer { //check to see if initializer is a signer
            return Err(ProgramError::MissingRequiredSignature);
        }

        let pda_token_account = next_account_info(account_info_iter)?;
        let initializer_main_account = next_account_info(account_info_iter)?;
        let initializer_sent_token_account = next_account_info(account_info_iter)?;
        let escrow_account = next_account_info(account_info_iter)?;

        if escrow_account.owner != program_id || escrow_account.is_writable == false {
            return Err(ProgramError::IllegalOwner);
        }

        let escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;

        if escrow_info.initializer_pubkey != *initializer.key {
            return Err(ProgramError::InvalidAccountData);
        }

        let token_program = next_account_info(account_info_iter)?;
        let pda_account_info = next_account_info(account_info_iter)?;
        let pda_token_account_info =
            TokenAccount::unpack(&pda_token_account.try_borrow_data()?)?;

        let (pda, nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        //transfer tokens back to initializer
        let transfer_to_initializer_ix = spl_token::instruction::transfer(
            token_program.key,
            pda_token_account.key,
            initializer_sent_token_account.key,
            &pda,
            &[&pda],
            pda_token_account_info.amount
        )?;
        msg!("Calling the token program to transfer tokens back to the initializer...");
        invoke_signed(
            &transfer_to_initializer_ix,
            &[
                pda_token_account.clone(),
                initializer_sent_token_account.clone(),
                pda_account_info.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[nonce]]],
        )?;

        let close_escrow_token_acct_ix = spl_token::instruction::close_account(
            token_program.key,
            pda_token_account.key,
            initializer_main_account.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close the escrow token account...");
        invoke_signed(
            &close_escrow_token_acct_ix,
            &[
                pda_token_account.clone(),
                initializer_main_account.clone(),
                pda_account_info.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[nonce]]],
        )?;

        msg!("Closing the escrow account...");
        **initializer_main_account.try_borrow_mut_lamports()? = initializer_main_account
            .lamports()
            .checked_add(escrow_account.lamports())
            .ok_or(EscrowError::AmountOverflow)?;
        **escrow_account.try_borrow_mut_lamports()? = 0;
        *escrow_account.try_borrow_mut_data()? = &mut [];

        Ok(())
    }
}

