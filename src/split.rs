use std::fmt;

/// Represent a Split, which is basically a portion of a transaction
#[derive(Default, Debug, Clone)]
pub struct Split {
    category: String,
    memo: String,
    pub(in crate) amount: f64,
}

impl Split {
    pub fn new() -> Self {
        Split::default()
    }

    pub fn category(mut self, val: &str) -> Self {
        self.category = String::from(val);
        self
    }

    pub fn memo(mut self, val: &str) -> Self {
        self.memo = String::from(val);
        self
    }

    pub fn amount(mut self, val: f64) -> Self {
        self.amount = val;
        self
    }

    pub fn build(self) -> Split {
        Split {
            category: self.category,
            memo: self.memo,
            amount: self.amount,
        }
    }
}

impl fmt::Display for Split {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "S{0}\nE{1}\n${2:.2}",
            self.category, self.memo, self.amount
        )
    }
}

#[cfg(test)]
mod split_test {
    use super::*;

    #[test]
    fn split_format() {
        let s = Split::new()
            .amount(-10.00)
            .category("testcat")
            .memo("testmemo")
            .build();
        let s2 = Split::new()
            .amount(-10.00)
            .category("testcat")
            .memo("")
            .build();

        assert_eq!(s.to_string(), "Stestcat\nEtestmemo\n$-10.00\n");
        assert_eq!(s2.to_string(), "Stestcat\nE\n$-10.00\n");
    }
}
