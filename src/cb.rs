use std::error::Error;

use copypasta::ClipboardProvider;
use dialoguer::{theme::Theme, Confirm};
use wl_clipboard_rs::copy::{MimeType, Source};

//

pub fn init(theme: &dyn Theme) -> Option<Clipboard> {
    Confirm::with_theme(theme)
        .with_prompt("Select to copy?")
        .default(true)
        .interact()
        .unwrap()
        .then_some(Clipboard)
}

//

pub struct Clipboard;

impl Clipboard {
    pub fn set_contents(&self, str: String) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let opts = wl_clipboard_rs::copy::Options::new();
        let Err(err) = opts.copy(Source::Bytes(str.as_bytes().into()), MimeType::Text) else {
            return Ok(());
        };

        eprintln!("Wayland failed, trying x11: {err} {err:?}");

        let mut ctx = copypasta::ClipboardContext::new()?;
        ctx.set_contents(str)?;

        Ok(())
    }
}
