use std::fmt::Formatter;

use crate::customizations::{Color, Customizations};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub enum Card {
    #[default]
    Classic,
    Vertical,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Self::Classic => "Classic",
            Self::Vertical => "Vertical",
        };
        f.write_str(data)
    }
}

impl Card {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match *self {
            Self::Classic => "classic.svg",
            Self::Vertical => "vertical.svg",
        }
    }

    #[must_use]
    pub fn from_name(data: &str) -> Option<Self> {
        let out = match data {
            "classic.svg" => Self::Classic,
            "vertical.svg" => Self::Vertical,
            _ => return None,
        };
        Some(out)
    }

    #[must_use]
    pub const fn default_customizations(&self) -> Customizations {
        match *self {
            Self::Classic => CLASSIC_CUSTOMIZATIONS,
            Self::Vertical => VERTICAL_CUSTOMIZATIONS,
        }
    }
}

const CLASSIC_CUSTOMIZATIONS: Customizations = Customizations {
    username: Color::new(255, 255, 255),
    rank: Color::new(255, 255, 255),
    level: Color::new(143, 202, 92),
    border: Color::new(133, 79, 43),
    background: Color::new(97, 55, 31),
    progress_foreground: Color::new(71, 122, 30),
    progress_background: Color::new(143, 202, 92),
    background_xp_count: Color::new(0, 0, 0),
    foreground_xp_count: Color::new(255, 255, 255),
    font: crate::Font::Mojang,
    toy: None,
    card: Card::Classic,
};

const VERTICAL_CUSTOMIZATIONS: Customizations = Customizations {
    username: Color::new(255, 255, 255),
    rank: Color::new(255, 255, 255),
    level: Color::new(251, 72, 196),
    border: Color::new(0, 0, 0),
    background: Color::new(10, 10, 10),
    progress_foreground: Color::new(251, 72, 196),
    progress_background: Color::new(199, 58, 157),
    background_xp_count: Color::new(255, 255, 255),
    foreground_xp_count: Color::new(255, 255, 255),
    font: crate::Font::Roboto,
    toy: Some(crate::Toy::Airplane),
    card: Card::Vertical,
};
