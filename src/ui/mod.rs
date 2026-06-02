use self::locale_button::LocaleButton;
use egui::{Response, Ui, Widget};

/// Extension methods for [`Ui`]
pub trait UiExt {
    fn locale_button(&mut self) -> Response;
}

impl UiExt for Ui {
    fn locale_button(&mut self) -> Response {
        LocaleButton::new().ui(self)
    }
}

pub mod locale_button;
