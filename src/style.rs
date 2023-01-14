use dialoguer::{theme::Theme, MultiSelect};

//

pub fn select(theme: &dyn Theme) -> Style {
    #[rustfmt::skip]
    let mut list = [
        (
            "Append title - Star Wars: Episode V - The Empire Strikes Back",
            true,
        ),
        (
            "Append year  - (1980)",
            true
        ),
        (
            "Append DB id - [imdbid-tt0080684]",
            false
        ),
    ];

    loop {
        let style = MultiSelect::with_theme(theme)
            .with_prompt("Select style")
            .items_checked(&list)
            .interact()
            .unwrap();
        list.iter_mut().for_each(|(_, v)| *v = false);
        for s in style {
            list[s].1 = true;
        }

        if !list.iter().any(|(_, v)| *v) {
            eprintln!("At least one has to be selected");
            continue;
        }

        break;
    }

    Style {
        title: list[0].1,
        year: list[1].1,
        id: list[2].1,
    }
}

//

#[derive(Clone, Copy)]
pub struct Style {
    pub title: bool,
    pub year: bool,
    pub id: bool,
}
