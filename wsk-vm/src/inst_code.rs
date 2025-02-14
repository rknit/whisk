use std::{io::Read, mem::size_of};

use crate::{
    inst::{Cmp, Inst},
    program::ProgramParseError,
    value::Value,
};

impl Inst {
    pub fn encode(self, out: &mut Vec<u8>) {
        match self {
            Inst::Halt => out.push(0x00),
            Inst::Push(value) => match value {
                Value::Int(v) => {
                    out.push(0x01);
                    out.extend(v.to_le_bytes());
                }
                Value::Bool(v) => match v {
                    true => out.push(0x02),
                    false => out.push(0x03),
                },
            },
            Inst::Pop => out.push(0x04),
            Inst::Load(i) => {
                out.push(0x05);
                out.extend(i.to_le_bytes());
            }
            Inst::Store(i) => {
                out.push(0x06);
                out.extend(i.to_le_bytes());
            }
            Inst::Add => out.push(0x10),
            Inst::Sub => out.push(0x11),
            Inst::Mul => out.push(0x12),
            Inst::Div => out.push(0x13),
            Inst::Mod => out.push(0x14),
            Inst::And => out.push(0x15),
            Inst::Or => out.push(0x16),
            Inst::Cmp(cmp) => match cmp {
                Cmp::Equal => out.push(0x17),
                Cmp::Less => out.push(0x18),
                Cmp::Greater => out.push(0x19),
            },
            Inst::Neg => out.push(0x20),
            Inst::Not => out.push(0x21),
            Inst::Jmp(offset) => {
                out.push(0x30);
                out.extend(offset.to_le_bytes());
            }
            Inst::JmpTrue(offset) => {
                out.push(0x31);
                out.extend(offset.to_le_bytes());
            }
            Inst::JmpFalse(offset) => {
                out.push(0x32);
                out.extend(offset.to_le_bytes());
            }
            Inst::Call(fi) => {
                out.push(0x40);
                out.extend(fi.to_le_bytes());
            }
            Inst::Ret => out.push(0x41),
        }
    }

    fn next_bytes<const N: usize>(it: &mut &[u8]) -> Result<[u8; N], ProgramParseError>
    where
        [u8; N]: Default,
    {
        let mut le_bytes: [u8; N] = Default::default();
        it.read(&mut le_bytes)
            .map_err(|_| ProgramParseError::InsufficientBytes)?;
        Ok(le_bytes)
    }

    pub fn decode(bytes: &mut &[u8]) -> Result<Self, ProgramParseError> {
        const USIZE_BYTES: usize = size_of::<usize>();
        const ISIZE_BYTES: usize = size_of::<isize>();
        const I64_BYTES: usize = size_of::<i64>();

        let mut byte: [u8; 1] = [0];
        bytes
            .read(&mut byte)
            .map_err(|_| ProgramParseError::InsufficientBytes)?;
        Ok(match byte[0] {
            0x00 => Inst::Halt,
            0x01 => {
                let int_bytes = Self::next_bytes::<I64_BYTES>(bytes)?;
                Inst::Push(i64::from_le_bytes(int_bytes).into())
            }
            0x02 => Inst::Push(true.into()),
            0x03 => Inst::Push(false.into()),
            0x04 => Inst::Pop,
            0x05 => {
                let index_bytes = Self::next_bytes::<USIZE_BYTES>(bytes)?;
                Inst::Load(usize::from_le_bytes(index_bytes))
            }
            0x06 => {
                let index_bytes = Self::next_bytes::<USIZE_BYTES>(bytes)?;
                Inst::Store(usize::from_le_bytes(index_bytes))
            }
            0x10 => Inst::Add,
            0x11 => Inst::Sub,
            0x12 => Inst::Mul,
            0x13 => Inst::Div,
            0x14 => Inst::And,
            0x15 => Inst::Or,
            0x16 => Cmp::Equal.into(),
            0x17 => Cmp::Less.into(),
            0x18 => Cmp::Greater.into(),
            0x20 => Inst::Neg,
            0x21 => Inst::Not,
            0x30 => {
                let index_bytes = Self::next_bytes::<ISIZE_BYTES>(bytes)?;
                Inst::Jmp(isize::from_le_bytes(index_bytes))
            }
            0x31 => {
                let index_bytes = Self::next_bytes::<ISIZE_BYTES>(bytes)?;
                Inst::JmpTrue(isize::from_le_bytes(index_bytes))
            }
            0x32 => {
                let index_bytes = Self::next_bytes::<ISIZE_BYTES>(bytes)?;
                Inst::JmpFalse(isize::from_le_bytes(index_bytes))
            }
            0x40 => {
                let index_bytes = Self::next_bytes::<USIZE_BYTES>(bytes)?;
                Inst::Call(usize::from_le_bytes(index_bytes))
            }
            0x41 => Inst::Ret,
            _ => {
                unimplemented!("unimplemented inst byte {:#04x}", byte[0]);
            }
        })
    }
}
