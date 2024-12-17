use std::{mem::size_of, ops::Deref};

use crate::{
    inst::{Cmp, Inst},
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
            Inst::And => out.push(0x14),
            Inst::Or => out.push(0x15),
            Inst::Cmp(cmp) => match cmp {
                Cmp::Equal => out.push(0x16),
                Cmp::Less => out.push(0x17),
                Cmp::Greater => out.push(0x18),
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
        }
    }

    fn next_bytes<T, const N: usize>(
        it: &mut impl Iterator<Item: Deref<Target = T>>,
    ) -> Option<[T; N]>
    where
        [T; N]: Default,
        T: ToOwned<Owned = T>,
    {
        let mut le_bytes: [T; N] = Default::default();
        for i in 0..le_bytes.len() {
            le_bytes[i] = (*it.next()?).to_owned();
        }
        Some(le_bytes)
    }

    pub fn decode(bytes: Vec<u8>) -> Option<Vec<Self>> {
        const USIZE_BYTES: usize = size_of::<usize>();
        const ISIZE_BYTES: usize = size_of::<isize>();
        const I64_BYTES: usize = size_of::<i64>();

        let mut insts = Vec::new();

        let mut it = bytes.iter();
        while let Some(byte) = it.next() {
            match byte {
                0x00 => insts.push(Inst::Halt),
                0x01 => {
                    let int_bytes = Self::next_bytes::<u8, I64_BYTES>(&mut it)
                        .expect("Not enough bytes to parse an integer");
                    insts.push(Inst::Push(i64::from_le_bytes(int_bytes).into()));
                }
                0x02 => insts.push(Inst::Push(true.into())),
                0x03 => insts.push(Inst::Push(false.into())),
                0x04 => insts.push(Inst::Pop),
                0x05 => {
                    let index_bytes = Self::next_bytes::<u8, USIZE_BYTES>(&mut it)
                        .expect("Not enough bytes to parse an index");
                    insts.push(Inst::Load(usize::from_le_bytes(index_bytes).into()));
                }
                0x06 => {
                    let index_bytes = Self::next_bytes::<u8, USIZE_BYTES>(&mut it)
                        .expect("Not enough bytes to parse an index");
                    insts.push(Inst::Store(usize::from_le_bytes(index_bytes).into()));
                }
                0x10 => insts.push(Inst::Add),
                0x11 => insts.push(Inst::Sub),
                0x12 => insts.push(Inst::Mul),
                0x13 => insts.push(Inst::Div),
                0x14 => insts.push(Inst::And),
                0x15 => insts.push(Inst::Or),
                0x16 => insts.push(Cmp::Equal.into()),
                0x17 => insts.push(Cmp::Less.into()),
                0x18 => insts.push(Cmp::Greater.into()),
                0x20 => insts.push(Inst::Neg),
                0x21 => insts.push(Inst::Not),
                0x30 => {
                    let index_bytes = Self::next_bytes::<u8, ISIZE_BYTES>(&mut it)
                        .expect("Not enough bytes to parse an offset");
                    insts.push(Inst::Jmp(isize::from_le_bytes(index_bytes).into()));
                }
                0x31 => {
                    let index_bytes = Self::next_bytes::<u8, ISIZE_BYTES>(&mut it)
                        .expect("Not enough bytes to parse an offset");
                    insts.push(Inst::JmpTrue(isize::from_le_bytes(index_bytes).into()));
                }
                0x32 => {
                    let index_bytes = Self::next_bytes::<u8, ISIZE_BYTES>(&mut it)
                        .expect("Not enough bytes to parse an offset");
                    insts.push(Inst::JmpFalse(isize::from_le_bytes(index_bytes).into()));
                }
                _ => {
                    debug_assert!(false, "unimplemented inst byte {:#04x}", byte);
                    return None;
                }
            }
        }

        Some(insts)
    }
}
