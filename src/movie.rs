use crate::style::Style;
use std::fmt;

//

pub struct Movie {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub year: Option<u16>,

    pub db: &'static str,
    pub style: Style,
}

//

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`")?;

        let mut first = true;
        let mut sep = |f: &mut fmt::Formatter| {
            if !first {
                write!(f, " ")?;
            }
            first = false;
            Ok::<_, fmt::Error>(())
        };

        if self.style.title {
            sep(f)?;
            write!(f, "{}", self.title)?;
        }

        if let (true, Some(year)) = (self.style.year, self.year.as_ref()) {
            sep(f)?;
            write!(f, "({year})")?;
        }

        if self.style.id {
            sep(f)?;
            write!(f, "[{}-{}]", self.db, self.id)?;
        }

        write!(f, "`")?;

        if !f.alternate() {
            write!(
                f,
                "\n - {}",
                self.desc.chars().take(100).collect::<String>()
            )?;
        }

        Ok(())
    }
}
