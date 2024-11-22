use solana_account_balance::Cluster;
use std::thread;
use std::time;
use std::thread::sleep;


fn main() {


    // make sure to change 1 to the number of public keys you want to check balances for.
    const pubkeys: [&str;1] = ["H2NQktZKBNQNAo2roJxTchWoWoztUHpKYKnF5y1X29Zt"];


    loop {
        
        // Define a delay between checks to avoid spamming the Solana network.
        // Spawn threads and then collect their results.
        // If this script helped you feel free to donate SOL to the following address:
        // 2mvLe15m6EF7jVzKWVYSFvBdeXw5iBbUn7Gstf91cQi6
        let delay = time::Duration::from_secs(5);
        let handles: Vec<_> = pubkeys.iter().map(|pubkey| {
            thread::spawn(move || {
                match solana_account_balance::get_solana_balance(pubkey, Cluster::MainnetBeta) {
                    Ok(balance) => println!("{}: {:?} SOL", pubkey, balance.sol),
                    Err(err) => println!("Error getting balance for {}: {:?}", pubkey, err),
                }
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        sleep(delay);  // Wait 5 seconds before checking again.
        println!("============ Checking balances ==========\n");  // Print a message to indicate the balance check is in progress.
    }
    
}