use std::fmt::Display;

pub const UNPERSERVED_REGS: RegSequence<16> = RegSequence::range(0, 16);
pub const PERSERVED_REGS: RegSequence<16> = RegSequence::range(16, 32);

pub const RETURN_VALUE_REG: Reg = reg(0);
pub const CALL_REGS: RegSequence<8> = UNPERSERVED_REGS.subrange(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reg(u8);
impl Reg {
    pub const fn r(index: u8) -> Self {
        if (UNPERSERVED_REGS.first().index() <= index && index <= UNPERSERVED_REGS.last().index())
            || (PERSERVED_REGS.first().index() <= index && index <= PERSERVED_REGS.last().index())
        {
            Self(index)
        } else {
            panic!("unknown register index")
        }
    }

    pub const fn index(&self) -> u8 {
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
impl From<u8> for Reg {
    fn from(value: u8) -> Self {
        Self::r(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegSequence<const N: usize>([Reg; N]);
impl<const N: usize> RegSequence<N> {
    pub const fn new(seq: [u8; N]) -> Self {
        assert!(N > 0, "sequence must not be empty");
        let mut regs: [Reg; N] = [Reg(0); N];
        let mut i: usize = 0;
        while i < N {
            regs[i] = Reg(seq[i]);
            i += 1;
        }
        Self(regs)
    }

    pub const fn range(begin: u8, end: u8) -> Self {
        assert!(
            begin < end,
            "sequence must be in increasing order and is not empty"
        );
        let mut regs: [Reg; N] = [Reg(0); N];
        let mut i: usize = 0;
        while i < (end - begin) as usize {
            regs[i] = Reg(begin + i as u8);
            i += 1;
        }
        Self(regs)
    }

    pub const fn subrange<const LEN: usize>(&self, index: usize) -> RegSequence<LEN> {
        assert!(index + LEN <= N, "range out of bound");
        let mut regs: [Reg; LEN] = [Reg(0); LEN];
        let mut i: usize = 0;
        while i < LEN {
            regs[i] = self.0[i + index];
            i += 1;
        }
        RegSequence::<LEN>(regs)
    }

    pub const fn first(&self) -> Reg {
        self.0[0]
    }

    pub const fn last(&self) -> Reg {
        self.0[N - 1]
    }
}
impl<'a, const N: usize> IntoIterator for &'a RegSequence<N> {
    type Item = Reg;

    type IntoIter = RegSequenceIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        RegSequenceIter(self, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegSequenceIter<'a, const N: usize>(&'a RegSequence<N>, usize);
impl<const N: usize> Iterator for RegSequenceIter<'_, N> {
    type Item = Reg;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == N {
            return None;
        }
        let v = self.0 .0[self.1];
        self.1 += 1;
        Some(v)
    }
}
