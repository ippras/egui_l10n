#[macro_export]
macro_rules! l10n {
    ($id:expr) => {
        const_format::formatcp!("{}_{}", env!("CARGO_PKG_NAME"), $id)
    };
    ($id:expr; abbreviation) => {
        const_format::formatcp!("{}_{}.abbreviation", env!("CARGO_PKG_NAME"), $id)
    };
    ($id:expr; hover) => {
        const_format::formatcp!("{}_{}.hover", env!("CARGO_PKG_NAME"), $id)
    };
}
