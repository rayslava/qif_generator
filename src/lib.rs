use std::fmt;
extern crate chrono;

/// QIF Account
/// It has a account_type and vector of items
pub struct Account {
    pub account_type: AccountType,
    pub items: Vec<Transaction>,
}

/// QIF Account types
pub enum AccountType {
    Bank,
    Cash,
    CreditCard,
    Investment,
    AssetAccount,
    LiabilityAccount,
}

/// Single QIF transaction
pub struct Transaction {
    pub date: chrono::Date<chrono::Utc>,
    pub amount: f64,
    pub payee: String,
    pub memo: String,
    pub category: String,
    pub cleared_status: String,
    pub splits: Vec<Split>,
}

/// Represent a Split, which is basically a portion of a transaction
pub struct Split {
    pub category: String,
    pub memo: String,
    pub amount: f64,
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
        // D11/28/2020
        // P
        // MCleaners
        // T-620.00
        // C
        // LExpenses:Transportation:Auto
        // ^
    }
}

#[cfg(test)]
mod receipt {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn split_format() {
        let s = Split {
            category: String::from("testcat"),
            memo: String::from("testmemo"),
            amount: -10.00,
        };
        let s2 = Split {
            category: String::from("testcat"),
            memo: String::new(),
            amount: -10.00,
        };

        assert_eq!(s.to_string(), "Stestcat\nEtestmemo\n$-10.00\n");
        assert_eq!(s2.to_string(), "Stestcat\nE\n$-10.00\n");
    }

    #[test]
    fn transaction_format() {
        let dt = Utc.ymd(2020, 11, 28);
        let t = Transaction {
            date: dt,
            category: String::from("testcat"),
            memo: String::from("testmemo"),
            amount: -10.00,
            payee: String::new(),
            cleared_status: String::new(),
            splits: Vec::new(),
        };
        assert_eq!(
            t.to_string(),
            r#"D11/28/2020
P
Mtestmemo
Ltestcat
C
T-10.00
^
"#
        );
    }

    #[test]
    fn split_transaction_format() {
        let dt = Utc.ymd(2020, 11, 28);
        let s1 = Split {
            category: String::from("Cat1"),
            memo: String::from("Split1"),
            amount: -10.00,
        };
        let s2 = Split {
            category: String::from("Cat2"),
            memo: String::from("Split2"),
            amount: -20.00,
        };
        let t = Transaction {
            date: dt,
            category: String::from("testcat"),
            memo: String::from("testmemo"),
            amount: -30.00,
            payee: String::new(),
            cleared_status: String::new(),
            splits: vec![s1, s2],
        };
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
