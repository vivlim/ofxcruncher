use rust_decimal::Decimal;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct OfxRoot {
  pub signonmsgsrsv1: OfxSignOnMsgs,
  pub bankmsgsrsv1: OfxBankMsgs,
}

#[derive(Deserialize, Debug)]
pub struct OfxSignOnMsgs {
  pub sonrs: OfxSignOn,
}

#[derive(Deserialize, Debug)]
pub struct OfxSignOn{
  pub status: OfxStatus,
  pub fi: OfxSignOnFinancialInstitution,
}

#[derive(Deserialize, Debug)]
pub struct OfxStatus {
    pub code: u32,
    pub severity: String
}

#[derive(Deserialize, Debug)]
pub struct OfxSignOnFinancialInstitution{
    pub org: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub fid: Decimal
}

#[derive(Deserialize, Debug)]
pub struct OfxBankMsgs{
    pub stmttrnrs: Vec<OfxStatement>,
}

#[derive(Deserialize, Debug)]
pub struct OfxStatement{
    #[serde(with = "rust_decimal::serde::str")]
    pub trnuid: Decimal,
    pub status: OfxStatus,
    pub stmtrs: OfxStatementTransactions
}

#[derive(Deserialize, Debug)]
pub struct OfxStatementTransactions{
    pub curdef: String,
    pub bankacctfrom: OfxBankAccount,
    pub banktranlist: OfxStatementTransactionList,
    pub ledgerbal: OfxBalance,
    pub availbal: Option<OfxBalance>,
}

#[derive(Deserialize, Debug)]
pub struct OfxBankAccount {
    pub bankid: String,
    pub acctid: String,
    pub accttype: String,
}

#[derive(Deserialize, Debug)]
pub struct OfxStatementTransactionList {

    #[serde(with = "rust_decimal::serde::str")]
    pub dtstart: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub dtend: Decimal,
    pub stmttrn: Vec<OfxStatementTransaction>,
}

#[derive(Deserialize, Debug)]
pub struct OfxStatementTransaction {
    pub trntype: String,
    pub dtposted: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub trnamt: Decimal,
    pub fitid: String,
    pub name: String,
    pub memo: String,
}

#[derive(Deserialize, Debug)]
pub struct OfxBalance {
    #[serde(with = "rust_decimal::serde::str")]
    pub balamt: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub dtasof: Decimal,
}



pub fn parse(input: &str) -> anyhow::Result<OfxRoot> {
    // Step 1: configure parser, then parse string
    let sgml = sgmlish::Parser::builder()
        .lowercase_names()
        .parse(input)?;
    // Step 2: normalization/validation
    let sgml = sgmlish::transforms::normalize_end_tags(sgml)?;
    // Step 3: deserialize into the desired type
    Ok(sgmlish::from_fragment::<OfxRoot>(sgml)?)
}