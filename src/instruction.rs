use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = MovieReviwPayload::try_from_slice(rest)?;

        Ok(match variant {
            0 => Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

#[derive(BorshDeserialize)]
struct MovieReviwPayload {
    title: String,
    rating: u8,
    description: String,
}
