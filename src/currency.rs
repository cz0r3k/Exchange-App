use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Currency {
    short_code: String,
    name: Option<String>,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.name.is_some() {
            write!(f, "{} ({})", self.short_code, self.name.clone().unwrap())
        } else {
            write!(f, "{}", self.short_code)
        }
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
    pub fn new(short_code: &str, name: Option<String>) -> Self {
        let short_code = short_code.to_string();
        Currency { short_code, name }
    }
    pub fn get_short_code(&self) -> &str {
        &self.short_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn currency_without_name() {
        let currency = Currency::new("PLN", None);
        assert_eq!("PLN", format!("{currency}"))
    }

    #[test]
    fn currency_with_name() {
        let currency = Currency::new("PLN", Some("Zloty".to_string()));
        assert_eq!("PLN (Zloty)", format!("{currency}"))
    }
}
