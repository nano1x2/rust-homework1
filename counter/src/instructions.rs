use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
    Reset,
    Update(UpdateArgs),
}

#[derive(Debug)]
pub struct UpdateArgs {
    pub value: u32,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            0 => {
                let value = Self::unpack_u32(rest)?;
                Ok(CounterInstructions::Increment(value))
            }
            1 => {
                let value = Self::unpack_u32(rest)?;
                Ok(CounterInstructions::Decrement(value))
            }
            2 => {
                let value = Self::unpack_u32(rest)?;
                Ok(CounterInstructions::Update(UpdateArgs { value }))
            }
            3 => Ok(CounterInstructions::Reset),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    fn unpack_u32(input: &[u8]) -> Result<u32, ProgramError> {
        let bytes = input
            .get(..4)
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }
}
