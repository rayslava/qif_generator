use std::fmt;
use std::str::FromStr;

/// QIF Account
#[derive(Default, Debug)]
pub struct Account {
    /// Account name is used during QIF import
    name: String,
    account_type: AccountType,
    description: String,
}

/// QIF Account types
///
/// There are different versions of QIF format descriptions, so this is minimal
/// set
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccountType {
    Bank,
    Cash,
    CreditCard,
    Investment,
    AssetAccount,
    LiabilityAccount,
}

impl FromStr for AccountType {
    type Err = ();

    fn from_str(input: &str) -> Result<AccountType, Self::Err> {
        match input {
            "Bank" => Ok(AccountType::Bank),
            "Cash" => Ok(AccountType::Cash),
            "CreditCard" => Ok(AccountType::CreditCard),
            "Investment" => Ok(AccountType::Investment),
            "AssetAccount" => Ok(AccountType::AssetAccount),
            "LiabilityAccount" => Ok(AccountType::LiabilityAccount),
            _ => Err(()),
        }
    }
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Bank
    }
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_string: String = match self {
            AccountType::Bank => String::from("Bank"),
            AccountType::Cash => String::from("Cash"),
            AccountType::CreditCard => String::from("CCard"),
            AccountType::Investment => String::from("Invst"),
            AccountType::AssetAccount => String::from("Oth A"),
            AccountType::LiabilityAccount => String::from("Oth L"),
        };
        write!(f, "{0}", type_string)
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "!Account\nN{0}\nT{1}\n^", self.name, self.account_type)
    }
}

impl Account {
    pub fn new() -> Self {
        Account::default()
    }

    pub fn name(mut self, val: &str) -> Self {
        self.name = String::from(val);
        self
    }

    pub fn description(mut self, val: &str) -> Self {
        self.description = String::from(val);
        self
    }

    pub fn account_type(mut self, val: AccountType) -> Self {
        self.account_type = val;
        self
    }

    pub fn get_type(&self) -> AccountType {
        self.account_type
    }

    pub fn build(self) -> Account {
        Account {
            name: self.name,
            description: self.description,
            account_type: self.account_type,
        }
    }
}

#[cfg(test)]
mod account_test {
    use super::*;

    #[test]
    fn account_format() {
        let acc = Account::new()
            .name("TestAcc")
            .account_type(AccountType::Cash)
            .description("Test")
            .build();

        assert_eq!(
            acc.to_string(),
            r#"!Account
NTestAcc
TCash
^
"#
        );
    }

    fn cmp(s: &str, a: AccountType) -> bool {
        let result = AccountType::from_str(s).map_err(|e| e);
        if result.is_err() {
            false
        } else {
            return a == result.unwrap();
        }
    }

    #[test]
    fn account_parse() {
        assert!(cmp("Bank", AccountType::Bank));
        assert!(cmp("Cash", AccountType::Cash));
        assert!(cmp("CreditCard", AccountType::CreditCard));
        assert!(cmp("Investment", AccountType::Investment));
        assert!(cmp("AssetAccount", AccountType::AssetAccount));
        assert!(cmp("LiabilityAccount", AccountType::LiabilityAccount));
        assert_eq!(cmp("asdf", AccountType::Bank), false);
    }
}
