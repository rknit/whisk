#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register(usize);
impl Register {
    const UNPERSERVED_REGS_START: usize = 0;
    const UNPERSERVED_REGS_END: usize = 15;

    const PERSERVED_REGS_START: usize = 16;
    const PERSERVED_REGS_END: usize = 31;

    pub const fn r(index: usize) -> Self {
        match index {
            Self::UNPERSERVED_REGS_START..=Self::UNPERSERVED_REGS_END
            | Self::PERSERVED_REGS_START..=Self::PERSERVED_REGS_END => Self(index),
            _ => panic!("unknown register index"),
        }
    }

    pub const fn get_index(&self) -> usize {
        self.0
    }
}

pub const fn reg(index: usize) -> Register {
    Register::r(index)
}
