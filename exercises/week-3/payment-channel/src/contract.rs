use crate::utils;
use crate::{Error, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use solana_sdk::transaction::Transaction;

/// Establishes a RPC connection with the solana cluster configured by
/// `solana config set --url <URL>`. Information about what cluster
/// has been configured is gleened from the solana config file
/// `~/.config/solana/cli/config.yml`.
pub fn establish_connection() -> Result<RpcClient> {
    let rpc_url = utils::get_rpc_url()?;
    Ok(RpcClient::new_with_commitment(
        rpc_url,
        CommitmentConfig::confirmed(),
    ))
}

/// Determines the amount of lamports that will be required to execute
/// this smart contract. The minimum balance is calculated assuming
/// that the user would like to make their account rent exempt.
///
/// For more information about rent see the Solana documentation
/// [here](https://docs.solana.com/implemented-proposals/rent#two-tiered-rent-regime)
pub fn get_balance_requirement(connection: &RpcClient) -> Result<u64> {
    let account_fee =
        connection.get_minimum_balance_for_rent_exemption(utils::get_greeting_data_size()?)?;

    let (_, fee_calculator) = connection.get_recent_blockhash()?;
    let transaction_fee = fee_calculator.lamports_per_signature * 100;

    Ok(transaction_fee + account_fee)
}

/// Gets the balance of PLAYER in lamports via a RPC call over
/// CONNECTION.
pub fn get_player_balance(player: &Keypair, connection: &RpcClient) -> Result<u64> {
    Ok(connection.get_balance(&player.pubkey())?)
}

/// Requests that AMOUNT lamports are transfered to PLAYER via a RPC
/// call over CONNECTION.
///
/// Airdrops are only avaliable on test networks.
pub fn request_airdrop(player: &Keypair, connection: &RpcClient, amount: u64) -> Result<()> {
    let sig = connection.request_airdrop(&player.pubkey(), amount)?;
    loop {
        let confirmed = connection.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(())
}

/// Loads keypair information from the file located at KEYPAIR_PATH
/// and then verifies that the loaded keypair information corresponds
/// to an executable account via CONNECTION. Failure to read the
/// keypair or the loaded keypair corresponding to an executable
/// account will result in an error being returned.
pub fn get_program(keypair_path: &str, connection: &RpcClient) -> Result<Keypair> {
    let program_keypair = read_keypair_file(keypair_path).map_err(|e| {
        Error::InvalidConfig(format!(
            "failed to read program keypair file ({}): ({})",
            keypair_path, e
        ))
    })?;

    let program_info = connection.get_account(&program_keypair.pubkey())?;
    if !program_info.executable {
        return Err(Error::InvalidConfig(format!(
            "program with keypair ({}) is not executable",
            keypair_path
        )));
    }

    Ok(program_keypair)
}


pub fn create_greeting_account(
    player: &Keypair,
    program: &Keypair,
    connection: &RpcClient,
) -> Result<()> {
    let greeting_pubkey = utils::get_greeting_public_key(&player.pubkey(), &program.pubkey())?;

    if let Err(_) = connection.get_account(&greeting_pubkey) {
        println!("creating greeting account");
        let lamport_requirement =
            connection.get_minimum_balance_for_rent_exemption(utils::get_greeting_data_size()?)?;

        
        let instruction = solana_sdk::system_instruction::create_account_with_seed(
            &player.pubkey(),
            &greeting_pubkey,
            &player.pubkey(),
            utils::get_greeting_seed(),
            lamport_requirement,
            utils::get_greeting_data_size()? as u64,
            &program.pubkey(),
        );
        let message = Message::new(&[instruction], Some(&player.pubkey()));
        let transaction =
            Transaction::new(&[player], message, connection.get_recent_blockhash()?.0);

        connection.send_and_confirm_transaction(&transaction)?;
    }

    Ok(())
}


pub fn say_hello(player: &Keypair, program: &Keypair, connection: &RpcClient) -> Result<()> {
    let greeting_pubkey = utils::get_greeting_public_key(&player.pubkey(), &program.pubkey())?;

    let instruction = Instruction::new_with_bytes(
        program.pubkey(),
        &[],
        vec![AccountMeta::new(greeting_pubkey, false)],
    );
    let message = Message::new(&[instruction], Some(&player.pubkey()));
    let transaction = Transaction::new(&[player], message, connection.get_recent_blockhash()?.0);

    connection.send_and_confirm_transaction(&transaction)?;

    Ok(())
}


pub fn count_greetings(player: &Keypair, program: &Keypair, connection: &RpcClient) -> Result<u32> {
    let greeting_pubkey = utils::get_greeting_public_key(&player.pubkey(), &program.pubkey())?;
    let greeting_account = connection.get_account(&greeting_pubkey)?;
    Ok(utils::get_greeting_count(&greeting_account.data)?)
}