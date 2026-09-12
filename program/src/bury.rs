use grams_api::prelude::*;
use steel::*;

/// Burn all ORE in the receivables ATA. Permissionless.
pub fn process_bury(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [receivables_info, receivables_ore_info, mint_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts.
    let receivables_address = receivables_pda().0;
    receivables_info.has_address(&receivables_address)?;
    let ore_account = receivables_ore_info
        .is_writable()?
        .as_associated_token_account(&receivables_address, &MINT_ADDRESS)?;
    mint_info.has_address(&MINT_ADDRESS)?.is_writable()?;
    token_program.is_program(&spl_token::ID)?;

    // Burn all ORE in the receivables ATA.
    let amount = ore_account.amount();
    if amount > 0 {
        invoke_signed(
            &spl_token::instruction::burn(
                token_program.key,
                receivables_ore_info.key,
                mint_info.key,
                receivables_info.key,
                &[receivables_info.key],
                amount,
            )?,
            &[
                token_program.clone(),
                receivables_ore_info.clone(),
                mint_info.clone(),
                receivables_info.clone(),
            ],
            &grams_api::ID,
            &[RECEIVABLES],
        )?;
    }

    Ok(())
}
