#[macro_export]
macro_rules! asset {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $path))
    };
}

#[macro_export]
macro_rules! ftl {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ftl/", $path))
    };
}

// #[macro_export]
// macro_rules! l10n {
//     ($id:expr) => {
//         const_format::formatcp!("{}_{}", env!("CARGO_PKG_NAME"), $id)
//     };
//     ($id:expr; abbreviation) => {
//         const_format::formatcp!("{}_{}.abbreviation", env!("CARGO_PKG_NAME"), $id)
//     };
//     ($id:expr; hover) => {
//         const_format::formatcp!("{}_{}.hover", env!("CARGO_PKG_NAME"), $id)
//     };
// }
