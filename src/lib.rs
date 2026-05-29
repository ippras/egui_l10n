pub use self::{context::ContextExt, localization::Localization, response::ResponseExt, ui::UiExt};
pub use unic_langid::LanguageIdentifier;
#[cfg(feature = "macros")]
pub use unic_langid::langid;

pub const ID_SOURCE: &str = "EguiL10n";

pub mod prelude {
    pub use crate::{ContextExt, Localization, ResponseExt, UiExt};
}

pub mod ui;

mod context;
mod localization;
mod r#macro;
mod response;
