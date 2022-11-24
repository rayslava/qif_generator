use crate::account::Account;
use crate::split::Split;
use chrono::{DateTime, Utc};
use std::fmt;

/// Single QIF transaction
#[derive(Debug)]
pub struct Transaction<'a> {
    account: &'a Account,
    /// Date of transaction, time is not supported in QIF format
    date: DateTime<Utc>,
    /// Last two digits is cents
    amount: i64,
    payee: String,
    memo: String,
    /// Category is used when transaction is spent in single piece, otherwise
    /// `splits` is used with local categorization
    category: String,
    cleared_status: String,
    /// Parts of transaction used for description of different categories.
    /// `Transaction` owns this vector since all splits do only have meaning in
    /// scope of the transaction.
    splits: Vec<Split>,
}

impl<'a> Transaction<'a> {
    pub fn new(acc: &'a Account) -> Self {
        Transaction {
            account: acc,
            date: Utc::now(),
            amount: 0,
            payee: String::new(),
            memo: String::new(),
            category: String::new(),
            cleared_status: String::new(),
            splits: Vec::new(),
        }
    }

    pub fn date(mut self, val: DateTime<Utc>) -> Self {
        self.date = val;
        self
    }

    pub fn amount(mut self, val: i64) -> Self {
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

    pub fn splits(mut self, val: &[Split]) -> Self {
        let sum = val.iter().fold(0, |acc, e| acc + e.amount);
        self.amount = sum;
        self.splits = val.to_owned();
        self
    }

    pub fn build(self) -> Result<Transaction<'a>, String> {
        if self.splits.iter().fold(0, |acc, e| acc + e.amount) != self.amount {
            Err("Sum of splits is not equal resulting amount".to_string())
        } else {
            Ok(Transaction {
                account: self.account,
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

    pub fn sum(&self) -> i64 {
        self.amount
    }
}

impl<'a> fmt::Display for Transaction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let amount_line = format!("{0:03}", self.amount);

        writeln!(
            f,
            "!Type:{0}\nD{1}\nP{2}\nM{3}\nL{4}\nC{5}\nT{6}.{7}",
            self.account.get_type(),
            self.date.format("%m/%d/%Y"),
            self.payee,
            self.memo,
            self.category,
            self.cleared_status,
            &amount_line[..amount_line.len() - 2],
            &amount_line[amount_line.len() - 2..]
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
    use crate::account::AccountType;
    use chrono::prelude::*;

    #[test]
    fn transaction_format() {
        let a = Account::new().account_type(AccountType::Cash);
        let t = Transaction::new(&a)
            .date(Utc.with_ymd_and_hms(2020, 11, 28, 0, 0, 0).unwrap())
            .category("testcat")
            .memo("testmemo")
            .amount(0)
            .build()
            .unwrap();

        assert_eq!(
            t.to_string(),
            r#"!Type:Cash
D11/28/2020
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
        let a = Account::new().account_type(AccountType::Investment);

        let s1 = Split::new().category("Cat1").memo("Split1").amount(-1000);
        let s2 = Split::new().category("Cat2").memo("Split2").amount(-2000);

        let t = Transaction::new(&a)
            .date(Utc.with_ymd_and_hms(2020, 11, 28, 0, 0, 0).unwrap())
            .category("testcat")
            .memo("testmemo")
            .with_split(&s1)
            .with_split(&s2)
            .build()
            .unwrap();

        assert_eq!(
            t.to_string(),
            r#"!Type:Invst
D11/28/2020
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

    #[test]
    fn split_list_check() {
        let a = Account::new().account_type(AccountType::Investment);

        let s1 = Split::new().category("Cat1").memo("Split1").amount(-1000);
        let s2 = Split::new().category("Cat2").memo("Split2").amount(-2000);

        let splits = vec![s1, s2];

        let t = Transaction::new(&a)
            .date(Utc.with_ymd_and_hms(2020, 11, 28, 0, 0, 0).unwrap())
            .category("testcat")
            .memo("testmemo")
            .splits(&splits)
            .build()
            .unwrap();

        assert_eq!(
            t.to_string(),
            r#"!Type:Invst
D11/28/2020
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
