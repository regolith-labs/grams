use crate::consts::*;
use steel::*;

pub fn receivables_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[RECEIVABLES], &crate::ID)
}
