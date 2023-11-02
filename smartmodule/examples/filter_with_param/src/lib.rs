use std::sync::OnceLock;

use fluvio_smartmodule::{
    smartmodule, SmartModuleRecord, Result, eyre,
    dataplane::smartmodule::{SmartModuleExtraParams},
};

static CRITERIA: OnceLock<String> = OnceLock::new();

#[smartmodule(init)]
fn init(params: SmartModuleExtraParams) -> Result<()> {
    if let Some(key) = params.get("key") {
        CRITERIA
            .set(key.clone())
            .map_err(|err| eyre!("failed setting key: {:#?}", err))
    } else {
        // set to default if not supplied
        CRITERIA
            .set("a".to_string())
            .map_err(|err| eyre!("failed setting key: {:#?}", err))
    }
}

#[smartmodule(filter)]
pub fn filter(record: &SmartModuleRecord) -> Result<bool> {
    let string = std::str::from_utf8(record.value.as_ref())?;
    Ok(string.contains(CRITERIA.get().unwrap()))
}
