mod bury;
mod unwrap;

use bury::*;
use grams_api::instruction::*;
use unwrap::*;
use steel::*;

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
