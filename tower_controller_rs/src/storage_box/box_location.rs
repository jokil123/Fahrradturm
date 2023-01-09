use std::fmt::{Display, Formatter};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct BoxLocation {
    pub level: u32,
    pub index: u32,
}

impl BoxLocation {
    pub fn value(&self) -> u64 {
        (self.level as u64) << 32 + self.index as u64
    }
}

impl Display for BoxLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "l{}i{}", self.level, self.index)
    }
}
