mod bury;
mod unwrap;

use bury::*;
use grams_api::instruction::*;
use solana_security_txt::security_txt;
use steel::*;
use unwrap::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&grams_api::ID, program_id, data)?;

    match ix {
        OreInstruction::Bury => process_bury(accounts, data)?,
        OreInstruction::Unwrap => process_unwrap(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

security_txt! {
    name: "Grams",
    project_url: "https://ore.supply",
    contacts: "email:hardhatchad@gmail.com,discord:hardhatchad",
    policy: "https://github.com/regolith-labs/grams/blob/master/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/regolith-labs/grams"
}
