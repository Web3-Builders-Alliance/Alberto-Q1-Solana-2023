<!-- https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/ -->
Step 1 - In the main dir , cargo new name_of_dir in this case 'paul_escrow' which will result in a src folder and a Cargo.toml

Step 2 - The main files will be in src

-coding challenge-

-Launch the Solana Test Validator
    paul_escrow solana-test-validator
    open new tab

-Add a Timelock to the PaulX escrow, You will need to use the Clock sysvar program
    by adding sysvar::clock into the scope, in state folder
    adding clock::Clock in processor

    adding two new fields in pub struct Escrow state.rs file
    pub time_out: u64,
    pub unlock_time: u64,

-When a escrow is initialized get the current slot, add 100 to it and save it to the account as unlock_time.
    adding the variable in processor


-Also add a time_out and make it 1000 slots after the unlock_time.
 -When the Exchange Instruction is called make sure the current slot is greater than the unlock_time but less than the time_out.
-Make sure to add a new Errors for the Timelock
-This will require you to add the two variables to the Escrow struct and adjust pack/unpack and LEN.

- explanation -
When instructions call to add X to the Program escrow, in this case paul escrow, you start by modiying state.rs pub struct Escrow and include the type of new elements added to the program, here it will be two u64, unlock_time and time_out.

Adding the new elements will require to update
array_refs![src, 1, 32, 32, 32, 8, 8, 8]; to include two new types

Ok(Escrow to include
            unlock_time: u64::from_le_bytes(*unlock_time),
            time_out: u64::from_le_bytes(*time_out),

            core::num
            pub const fn from_le_bytes(bytes: [u8; _]) -> Self
            Create a native endian integer value from its representation as a byte array in little endian
            well, the definiton of from_le_bytes was more confusing so lets circle back to this one later

The dts -> = mut_array_refs![dst, 1, 32, 32, 32, 8, 8, 8]; also gets update to include two new 8 types

let Escrow add  unlock_time,
                time_out,

and the last part in state.rs
I think the * means "everything in unlock_time_dst" is the to_le_bytes
        *unlock_time_dst = unlock_time.to_le_bytes();
        *time_out_dts = time_out.to_le_bytes();

    - processor.rs -
add clock::Clock, to the solana_program

add the variables after Err

        let clock = Clock::get()?.slot;         //this is to called the main clock
        let unlock_time: u64 = clock + 100;     //
        let time_out: u64 = unlock_time + 1000;

    - error.rs -

-coding challenge 2-
use AccountMeta and build public fn to call our ix
    How do you build new pub functions?
        - build new pub fn inside the init_escrow
    AccountMeta is in instrcutions


Add 2 new ix Cancel and ReseTimeLock
    you are adding the ixs in the instruction.rs file
    inside pub enum EscrowInstruction
    Neither of these ix will take any data
    STL will

Cancel will return any sent SPL tokens to the initiator and then close the account
so that the rent is also returned

ResetTimeLock will reset the time_lock and time_out

"add the instructions to the enum and the processor."
this means

