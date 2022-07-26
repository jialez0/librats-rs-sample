use lib::{get_evidence, AttestationEv};
use anyhow::Result;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let report_data = "test".as_bytes().to_vec();
    let ev = get_evidence(report_data).await?;

    fs::write("../../fixtures/quote", &ev.quote)?;
    fs::write("../../fixtures/eventlog_info", &ev.eventlog_info)?;
    fs::write("../../fixtures/eventlog_data", &ev.eventlog_data)?;

    Ok(())
}
