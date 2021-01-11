use crate::split::Split;
use chrono::{Date, Utc};
use std::fmt;

/// Single QIF transaction
#[derive(Debug)]
pub struct Transaction {
    /// Date of transaction, time is not supported in QIF format
    date: Date<Utc>,
    amount: f64,
    payee: String,
    memo: String,
    /// Category is used when transaction is spent in single piece, otherwise
    /// `splits` is used
    category: String,
    cleared_status: String,
    /// Parts of transaction used for description of different categories
    splits: Vec<Split>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            date: Utc::today(),
            amount: 0.0,
            payee: String::new(),
            memo: String::new(),
            category: String::new(),
            cleared_status: String::new(),
            splits: Vec::new(),
        }
    }

    pub fn date(mut self, val: Date<Utc>) -> Self {
        self.date = val;
        self
    }

    pub fn amount(mut self, val: f64) -> Self {
        self.amount = val;
        self
    }

    pub fn payee(mut self, val: &str) -> Self {
        self.payee = String::from(val);
        self
    }

    pub fn memo(mut self, val: &str) -> Self {
        self.memo = String::from(val);
        self
    }

    pub fn category(mut self, val: &str) -> Self {
        self.category = String::from(val);
        self
    }

    pub fn cleared_status(mut self, val: &str) -> Self {
        self.cleared_status = String::from(val);
        self
    }

    pub fn splits(mut self, val: Vec<Split>) -> Self {
        self.splits = val;
        self
    }

    pub fn build(self) -> Result<Transaction, String> {
        if (self.splits.iter().fold(0.0f64, |acc, e| acc + e.amount) - self.amount).abs()
            > f64::EPSILON
        {
            Err("Sum of splits is not equal resulting amount".to_string())
        } else {
            Ok(Transaction {
                date: self.date,
                amount: self.amount,
                payee: self.payee,
                memo: self.memo,
                category: self.category,
                cleared_status: self.cleared_status,
                splits: self.splits,
            })
        }
    }

    pub fn with_split(mut self, val: &Split) -> Self {
        self.amount += val.amount;
        self.splits.push(val.clone());
        self
    }

    pub fn with_splits(mut self, val: &[Split]) -> Self {
        let sum = val.iter().fold(0.0f64, |acc, e| acc + e.amount);
        self.amount = sum;
        self.splits = val.to_owned();
        self
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "D{0}\nP{1}\nM{2}\nL{3}\nC{4}\nT{5:.2}",
            self.date.format("%m/%d/%Y"),
            self.payee,
            self.memo,
            self.category,
            self.cleared_status,
            self.amount
        )?;

        if !self.splits.is_empty() {
            for s in self.splits.iter() {
                write!(f, "{}", s)?;
            }
        }
        writeln!(f, "^")
    }
}

#[cfg(test)]
mod receipt {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn transaction_format() {
        let t = Transaction::new()
            .date(Utc.ymd(2020, 11, 28))
            .category("testcat")
            .memo("testmemo")
            .amount(0.00)
            .build()
            .unwrap();

        assert_eq!(
            t.to_string(),
            r#"D11/28/2020
P
Mtestmemo
Ltestcat
C
T0.00
^
"#
        );
    }

    #[test]
    fn split_transaction_format() {
        let s1 = Split::new().category("Cat1").memo("Split1").amount(-10.00);
        let s2 = Split::new().category("Cat2").memo("Split2").amount(-20.00);

        let t = Transaction::new()
            .date(Utc.ymd(2020, 11, 28))
            .category("testcat")
            .memo("testmemo")
            .with_split(&s1)
            .with_split(&s2)
            .build()
            .unwrap();

        assert_eq!(
            t.to_string(),
            r#"D11/28/2020
P
Mtestmemo
Ltestcat
C
T-30.00
SCat1
ESplit1
$-10.00
SCat2
ESplit2
$-20.00
^
"#
        );
    }
}
