use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::{theme::Theme, Confirm};

//

pub fn init(theme: &dyn Theme) -> Option<ClipboardContext> {
    Confirm::with_theme(theme)
        .with_prompt("Select to copy?")
        .default(true)
        .interact()
        .unwrap()
        .then(|| ClipboardProvider::new().ok())
        .flatten()
}
