// use bonsol_interface::prover_version;
use borsh::{BorshSerialize, BorshDeserialize, to_vec};
use bonsol_interface::callback::{handle_callback, BonsolCallback};
use bonsol_interface::instructions::{execute_v1, CallbackConfig, ExecutionConfig, InputRef};

// use solana_program::address_lookup_table::program;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::{
    account_info::{
        next_account_info, 
        AccountInfo
    }, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey,
    program_error::ProgramError,
};
use solana_program::{
    clock::Clock, 
    sysvar::Sysvar,
    program::{invoke_signed, invoke},
};
use solana_pubkey::declare_id;

declare_id!("GDBi9xt8A5bZKYTEU6DDYFufCmoBRFoyehS2GCYpwmQq");

const IMAGE_ID: &str = "faf0deac826c8b954716be338e35117cca60c1177d825b736f5957630161e80f"; // Image ID of the zk program say_hello

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum BonsolExampleInstruction {
    Execute { 
        execution_id: String,
        input1: String,
        bump: u8, // Bump seed for the PDA
    },
    Callback,
}

impl BonsolExampleInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Get the instruction variant from the first byte
        let (&variant, _rest) = input
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    
        match variant {
            0 => {
                Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
            }
            1 => Ok(Self::Callback), // No additional data needed
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

entrypoint!(process_instruction);
pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = BonsolExampleInstruction::unpack(instruction_data)?;
        
    match instruction {
        BonsolExampleInstruction::Execute { execution_id, input1, bump } => {
            msg!("Calling process_bonsol_execute");
            process_bonsol_execute(program_id, accounts, &execution_id, &input1, bump)
        }
        BonsolExampleInstruction::Callback => {
            msg!("calling process_bonsol_callback");
            process_bonsol_callback(program_id, accounts, instruction_data)
        }
    }
}

pub fn create_program_account<'a>(
    account: &'a AccountInfo<'a>,
    seeds: &[&[u8]],
    space: u64,
    payer: &'a AccountInfo<'a>,
    system: &'a AccountInfo<'a>,
    additional_lamports: Option<u64>,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    let lamports = Rent::get()?.minimum_balance(space as usize) + additional_lamports.unwrap_or(0);
    let create_pda_account_ix =
        system_instruction::create_account(payer.key, account.key, lamports, space, program_id);
    invoke_signed(
        &create_pda_account_ix,
        &[account.clone(), payer.clone(), system.clone()],
        &[seeds],
    )
    .map_err(|_e| ProgramError::Custom(0))
}


pub fn process_bonsol_execute<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    execution_id: &str,
    input1: &str,
    bump: u8,
) -> ProgramResult {
        msg!("Program ID: {}", program_id);
        msg!("Execution ID = {}", execution_id);
        msg!("Image ID = {}", IMAGE_ID);
        msg!("Input 1 = {}", input1);
        msg!("Bump = {}", bump);

        let accounts_iter = &mut accounts.iter();
        let requester = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let execution_account = next_account_info(accounts_iter)?;
        let deployment_account = next_account_info(accounts_iter)?;
        let my_program = next_account_info(accounts_iter)?;
        let system = next_account_info(accounts_iter)?;
        let bonsol_program = next_account_info(accounts_iter)?;

        msg!("Requester: {}", requester.key);
        msg!("Payer: {}", payer.key);
        msg!("Execution Account: {}", execution_account.key);
        msg!("Deployment Account: {}", deployment_account.key);
        msg!("My Program: {}", my_program.key);
        msg!("System Program: {}", system.key);
        msg!("Bonsol Program: {}", bonsol_program.key);

        create_program_account(
            &requester,
            &[execution_id.as_bytes(), &[bump]],
            32,
            &payer,
            &system,
            None,
            program_id,
        )?;

        msg!("Created requester account: {}", requester.key);

        let tip = 12000;
        let expiration = Clock::get()?.slot + 5000;

        let ix = execute_v1(
            requester.key,
            payer.key,
            IMAGE_ID,
            execution_id,
            vec![InputRef::public(input1.as_bytes())],
            tip,
            expiration,
            ExecutionConfig {
                verify_input_hash: false,
                input_hash: None,
                forward_output: true,
            },
            Some(CallbackConfig {
                program_id: *program_id,
                instruction_prefix: vec![1],
                extra_accounts: vec![],
            }),
            None
        )?;

        msg!("Bonsol execute instruction created");

        let _execute_v1_accounts = [
            requester.clone(),
            payer.clone(),
            execution_account.clone(),
            deployment_account.clone(),
            my_program.clone(),
            system.clone(),
            bonsol_program.clone(),
        ];
    
        msg!("Executing Bonsol with execution ID: {}", execution_id);
        invoke_signed(
            &ix,
            &accounts,
            &[&[execution_id.as_bytes(), &[bump]]]
        )?;
        msg!("Bonsol execution invoked successfully");
        Ok(())
}

pub fn process_bonsol_callback(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    msg!("Bonsol callback invoked");
    Ok(())
}