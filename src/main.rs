use rust_decimal::Decimal;
use serde::Deserialize;

mod sample;

#[derive(Deserialize, Debug)]
struct OfxRoot {
  signonmsgsrsv1: OfxSignOnMsgs,
  bankmsgsrsv1: OfxBankMsgs,
}

#[derive(Deserialize, Debug)]
struct OfxSignOnMsgs {
  sonrs: OfxSignOn,
}

#[derive(Deserialize, Debug)]
struct OfxSignOn{
  status: OfxStatus,
  fi: OfxSignOnFinancialInstitution,
}

#[derive(Deserialize, Debug)]
struct OfxStatus {
    code: u32,
    severity: String
}

#[derive(Deserialize, Debug)]
struct OfxSignOnFinancialInstitution{
    org: String,
    #[serde(with = "rust_decimal::serde::str")]
    fid: Decimal
}

#[derive(Deserialize, Debug)]
struct OfxBankMsgs{
    stmttrnrs: Vec<OfxStatement>,
}

#[derive(Deserialize, Debug)]
struct OfxStatement{
    #[serde(with = "rust_decimal::serde::str")]
    trnuid: Decimal,
    status: OfxStatus,
    stmtrs: OfxStatementTransactions
}

#[derive(Deserialize, Debug)]
struct OfxStatementTransactions{
    curdef: String,
    bankacctfrom: OfxBankAccount,
    banktranlist: OfxStatementTransactionList,
    ledgerbal: OfxBalance,
    availbal: OfxBalance,
}

#[derive(Deserialize, Debug)]
struct OfxBankAccount {
    bankid: String,
    acctid: String,
    accttype: String,
}

#[derive(Deserialize, Debug)]
struct OfxStatementTransactionList {

    #[serde(with = "rust_decimal::serde::str")]
    dtstart: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    dtend: Decimal,
    stmttrn: Vec<OfxStatementTransaction>,
}

#[derive(Deserialize, Debug)]
struct OfxStatementTransaction {
    trntype: String,
    dtposted: String,
    #[serde(with = "rust_decimal::serde::str")]
    trnamt: Decimal,
    fitid: String,
    name: String,
    memo: String,
}

#[derive(Deserialize, Debug)]
struct OfxBalance {
    #[serde(with = "rust_decimal::serde::str")]
    balamt: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    dtasof: Decimal,
}



fn main() -> anyhow::Result<()> {
    // Step 1: configure parser, then parse string
    let sgml = sgmlish::Parser::builder()
        .lowercase_names()
        .parse(sample::input)?;
    // Step 2: normalization/validation
    let sgml = sgmlish::transforms::normalize_end_tags(sgml)?;
    // Step 3: deserialize into the desired type
    let example = sgmlish::from_fragment::<OfxRoot>(sgml)?;
    println!("{:?}", example);
    Ok(())
}
