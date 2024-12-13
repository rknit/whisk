use core::{
    cmp::{max, min},
    fmt,
};

#[derive(Clone)]
pub struct Located<T>(pub T, pub LocationRange);
impl<T> Located<T> {
    pub fn new_temporary(v: T) -> Self {
        Located(v, LocationRange::default())
    }
}
impl<T> From<(T, LocationRange)> for Located<T> {
    fn from(value: (T, LocationRange)) -> Self {
        Self(value.0, value.1)
    }
}
impl<T: fmt::Debug> fmt::Debug for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?} {:?}", self.0, self.1)
    }
}

pub trait Locatable {
    fn get_location(&self) -> LocationRange;
}

impl<T: Locatable> From<T> for Located<T> {
    fn from(value: T) -> Self {
        let loc = value.get_location();
        Located(value, loc)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct LocationRange {
    pub start: Location,
    pub end: Location,
}
impl LocationRange {
    pub fn next(&self) -> Location {
        self.end.next()
    }

    pub fn combine(left: Self, right: Self) -> Self {
        Self {
            start: min::<Location>(left.start, right.start),
            end: max::<Location>(left.end, right.end),
        }
    }
}
impl fmt::Debug for LocationRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "[{:#?}]", self.start)
        } else {
            write!(f, "[{:#?}-{:#?}]", self.start, self.end)
        }
    }
}
impl PartialOrd for LocationRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(if self.end < other.start {
            std::cmp::Ordering::Less
        } else if self.start > other.end {
            std::cmp::Ordering::Greater
        } else if self.start == other.start && self.end == other.end {
            std::cmp::Ordering::Equal
        } else {
            return None;
        })
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Ord)]
pub struct Location {
    pub line: u32,
    pub col: u32,
}
impl Location {
    pub fn front(&self) -> Self {
        Self {
            line: self.line,
            col: if self.col >= 1 { self.col - 1 } else { 0 },
        }
    }

    pub fn next(&self) -> Self {
        Self {
            line: self.line,
            col: self.col + 1,
        }
    }
}
impl Into<LocationRange> for Location {
    fn into(self) -> LocationRange {
        LocationRange {
            start: self,
            end: self,
        }
    }
}
impl fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line == 0 || self.col == 0 {
            write!(f, "-")
        } else {
            write!(f, "{:#?}:{:#?}", self.line, self.col)
        }
    }
}
impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(if self.line < other.line {
            std::cmp::Ordering::Less
        } else if self.line > other.line {
            std::cmp::Ordering::Greater
        } else if self.col < other.col {
            std::cmp::Ordering::Less
        } else if self.col > other.col {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        })
    }
}
