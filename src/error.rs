use thiserror::Error;


use solana_program::program_error::ProgramError;


#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
//invalidinstruction
#[error("Invalid Instruction")]
InvalidInstruction,

//NotRentExempt
#[error("Not Rent Exempt")]
NotRentExempt,



}




impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

