#[macro_export]
macro_rules! l10n {
    ($id:expr) => {
        const_format::formatcp!("{}_{}", $crate::r#const::WIDGETS, $id)
    };
    ($id:expr; abbreviation) => {
        const_format::formatcp!("{}_{}.abbreviation", $crate::r#const::WIDGETS, $id)
    };
    ($id:expr; hover) => {
        const_format::formatcp!("{}_{}.hover", $crate::r#const::WIDGETS, $id)
    };
}
