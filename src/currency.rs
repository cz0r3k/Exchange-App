use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Currency {
    name: String,
    short_code: String,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.short_code, self.name)
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
