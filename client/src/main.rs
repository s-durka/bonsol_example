use bonsol_example::BonsolExampleInstruction;
use bonsol_sdk::{deployment_address, execution_address};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::{AccountMeta, Instruction},
    system_program,

};
use std::str::FromStr;
use borsh::to_vec;
use rand::{
    Rng, 
    distributions::Alphanumeric
};

// say_hello zk program
const IMAGE_ID: &str = "faf0deac826c8b954716be338e35117cca60c1177d825b736f5957630161e80f"; // Image ID of the zk program
const MY_ID: &str = "GDBi9xt8A5bZKYTEU6DDYFufCmoBRFoyehS2GCYpwmQq"; // My program ID

#[tokio::main]
async fn main() {
    // Program ID of the Solana program in lib.rs
    let my_program = Pubkey::from_str(MY_ID).unwrap();
    let bonsol_program = bonsol_interface::ID;

    let rpc_url = String::from("http://127.0.0.1:8899");
    let rpc_client = RpcClient::new_with_commitment(&rpc_url, CommitmentConfig::confirmed());
    let payer = Keypair::new();

    let signature = rpc_client
        .request_airdrop(&payer.pubkey(), 100_000_000_000)
        .expect("Failed to request airdrop");

    // Wait for airdrop confirmation
    loop {
        let confirmed = rpc_client.confirm_transaction(&signature).unwrap();
        if confirmed {
            println!("Received airdrop");
            break;
        }
    }

    invoke_execute(my_program, bonsol_program, payer, &rpc_client);
}

#[allow(dead_code)]
async fn invoke_callback(my_program: Pubkey, payer: Keypair, rpc_client: &RpcClient) {
    // Callback instruction to invoke the Bonsol program ourselves (not via BonsolClient)
    // Just an example, not used in main()
    let instruction1 = BonsolExampleInstruction::Callback;
    let data1 = to_vec(&instruction1).unwrap();
    let callback_instruction = Instruction::new_with_bytes(
        my_program, 
        &data1, 
        vec![],
    );
    let mut transaction = Transaction::new_with_payer(
        &[callback_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], rpc_client.get_latest_blockhash().unwrap());
    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Callback Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending callback transaction: {}", err),
    }
}

fn invoke_execute(
    my_program: Pubkey,
    bonsol_program: Pubkey,
    payer: Keypair,
    rpc_client: &RpcClient,
) {
        let execution_id = rand_id(16);
    let input = "hello world";

    let (requester, bump) =
        Pubkey::find_program_address(&[execution_id.as_bytes()], &my_program);

    let (execution_account, _) = execution_address(&requester, execution_id.as_bytes());
    let (deployment_account, _) = deployment_address(IMAGE_ID);

    let instruction_data = BonsolExampleInstruction::Execute {
        execution_id: execution_id.to_string(),
        input1: input.to_string(),
        bump,
    };
    let data = to_vec(&instruction_data).unwrap();

    let accounts = vec![
        AccountMeta::new(requester, false),
        AccountMeta::new_readonly(payer.pubkey(), true),
        AccountMeta::new(execution_account, false),
        AccountMeta::new_readonly(deployment_account, false),
        AccountMeta::new_readonly(my_program, false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(bonsol_program, false),
    ];

    let execute_instruction = Instruction::new_with_bytes(
        my_program, 
        &data, 
        accounts
    );

    let mut transaction = Transaction::new_with_payer(
        &[execute_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], rpc_client.get_latest_blockhash().unwrap());

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}

fn rand_id(chars: usize) -> String {
    let mut rng = rand::thread_rng();
    (&mut rng)
        .sample_iter(Alphanumeric)
        .take(chars)
        .map(char::from)
        .collect()
}