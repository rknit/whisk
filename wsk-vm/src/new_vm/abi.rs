use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reg(u8);
impl Reg {
    const UNPERSERVED_REGS_START: u8 = 0;
    const UNPERSERVED_REGS_END: u8 = 15;

    const PERSERVED_REGS_START: u8 = 16;
    const PERSERVED_REGS_END: u8 = 31;

    pub const fn r(index: u8) -> Self {
        match index {
            Self::UNPERSERVED_REGS_START..=Self::UNPERSERVED_REGS_END
            | Self::PERSERVED_REGS_START..=Self::PERSERVED_REGS_END => Self(index),
            _ => panic!("unknown register index"),
        }
    }

    pub const fn get_index(&self) -> u8 {
        self.0
    }
}

pub const fn reg(index: u8) -> Reg {
    Reg::r(index)
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.0)
    }
}
