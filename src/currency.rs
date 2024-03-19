use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, PartialEq, Eq)]
pub struct Currency {
    name: String,
    short_code: String,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.short_code, self.name)
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.short_code.cmp(&other.short_code)
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Currency {
    pub fn new(name: String, short_code: String) -> Self {
        Currency { name, short_code }
    }
    pub fn get_short_code(&self) -> &str {
        &self.short_code
    }
}
