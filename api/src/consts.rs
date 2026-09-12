use solana_program::{pubkey, pubkey::Pubkey};

/// The decimal precision of the ORE token.
pub const TOKEN_DECIMALS: u8 = 11;

/// The seed of the receivables account PDA.
pub const RECEIVABLES: &[u8] = b"receivables";

/// The address of the ORE mint.
pub const MINT_ADDRESS: Pubkey = pubkey!("oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp");

/// The address of the stORE mint.
pub const STORE_MINT_ADDRESS: Pubkey = pubkey!("storenSbvkfzircixnaosc5CbzNZVrHJ6S3EKrS1yqR");
