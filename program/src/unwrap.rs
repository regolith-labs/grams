use grams_api::prelude::*;
use steel::*;

/// Unwrap all stORE in the receivables ATA into ORE. Permissionless.
pub fn process_unwrap(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer_info, receivables_info, receivables_ore_info, receivables_store_info, ore_mint_info, store_mint_info, stake_info, stake_tokens_info, treasury_info, treasury_tokens_info, vault_info, vault_tokens_info, vesting_info, system_program, token_program, associated_token_program, ore_stake_program, ore_lst_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts.
    payer_info.is_signer()?;
    let receivables_address = receivables_pda().0;
    receivables_info.has_address(&receivables_address)?;
    let store_account = receivables_store_info
        .as_associated_token_account(&receivables_address, &STORE_MINT_ADDRESS)?;
    ore_lst_program.is_program(&ore_lst_api::ID)?;

    // Unwrap all stORE into ORE via CPI.
    let amount = store_account.amount();
    if amount > 0 {
        let unwrap_ix =
            ore_lst_api::sdk::unwrap(*receivables_info.key, *payer_info.key, amount);
        invoke_signed(
            &unwrap_ix,
            &[
                receivables_info.clone(),
                payer_info.clone(),
                receivables_ore_info.clone(),
                receivables_store_info.clone(),
                ore_mint_info.clone(),
                store_mint_info.clone(),
                stake_info.clone(),
                stake_tokens_info.clone(),
                treasury_info.clone(),
                treasury_tokens_info.clone(),
                vault_info.clone(),
                vault_tokens_info.clone(),
                vesting_info.clone(),
                system_program.clone(),
                token_program.clone(),
                associated_token_program.clone(),
                ore_stake_program.clone(),
            ],
            &grams_api::ID,
            &[RECEIVABLES],
        )?;
    }

    Ok(())
}
