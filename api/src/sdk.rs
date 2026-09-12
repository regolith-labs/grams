use ore_lst_api::state::vault_pda;
use ore_stake_api::state::{stake_pda, treasury_pda, vesting_pda};
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use steel::*;

use crate::{consts::*, instruction::*, state::*};

pub fn bury() -> Instruction {
    let receivables_address = receivables_pda().0;
    let receivables_ore_address = get_associated_token_address(&receivables_address, &MINT_ADDRESS);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(receivables_address, false),
            AccountMeta::new(receivables_ore_address, false),
            AccountMeta::new(MINT_ADDRESS, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Bury {}.to_bytes(),
    }
}

pub fn unwrap(payer: Pubkey) -> Instruction {
    let receivables_address = receivables_pda().0;
    let receivables_ore_address = get_associated_token_address(&receivables_address, &MINT_ADDRESS);
    let receivables_store_address =
        get_associated_token_address(&receivables_address, &STORE_MINT_ADDRESS);
    let vault_address = vault_pda().0;
    let vault_tokens = get_associated_token_address(&vault_address, &MINT_ADDRESS);
    let stake_address = stake_pda(vault_address).0;
    let stake_tokens_address = get_associated_token_address(&stake_address, &MINT_ADDRESS);
    let treasury_address = treasury_pda().0;
    let treasury_tokens_address = get_associated_token_address(&treasury_address, &MINT_ADDRESS);
    let vesting_address = vesting_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(receivables_address, false),
            AccountMeta::new(receivables_ore_address, false),
            AccountMeta::new(receivables_store_address, false),
            AccountMeta::new(MINT_ADDRESS, false),
            AccountMeta::new(STORE_MINT_ADDRESS, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(stake_tokens_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new(vault_address, false),
            AccountMeta::new(vault_tokens, false),
            AccountMeta::new(vesting_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(ore_stake_api::ID, false),
            AccountMeta::new_readonly(ore_lst_api::ID, false),
        ],
        data: Unwrap {}.to_bytes(),
    }
}
