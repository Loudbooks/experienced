use crate::{cards::Card, Error, Font, Toy};

#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Customizations {
    pub username: Color,
    pub rank: Color,
    pub level: Color,
    pub border: Color,
    pub background: Color,
    pub progress_foreground: Color,
    pub progress_background: Color,
    pub background_xp_count: Color,
    pub foreground_xp_count: Color,
    pub font: Font,
    pub toy: Option<Toy>,
    pub card: Card,
}

macro_rules! add_output {
    ($f:expr, $name:expr, $val:expr, $default:expr) => {
        write!($f, "{}: `{}`", $name, $val)?;
        if $val == $default {
            writeln!($f, " (default)")?;
        } else {
            writeln!($f)?;
        };
    };
}

impl std::fmt::Display for Customizations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let defaults = self.card.default_customizations();
        add_output!(f, "Important text", self.username, defaults.username);
        add_output!(f, "Rank", self.rank, defaults.rank);
        add_output!(f, "Level", self.level, defaults.level);
        add_output!(f, "Border", self.border, defaults.border);
        add_output!(f, "Background", self.background, defaults.background);
        add_output!(
            f,
            "Progress bar completed",
            self.progress_foreground,
            defaults.progress_foreground
        );
        add_output!(
            f,
            "Progress bar remaining",
            self.progress_background,
            defaults.progress_background
        );
        add_output!(
            f,
            "Progress bar foreground overlay",
            self.foreground_xp_count,
            defaults.foreground_xp_count
        );
        add_output!(
            f,
            "Progress bar background overlay",
            self.background_xp_count,
            defaults.background_xp_count
        );
        add_output!(f, "Font", self.font, defaults.font);
        writeln!(
            f,
            "Toy: `{}`",
            self.toy
                .map_or_else(|| "None".to_owned(), |v| v.to_string())
        )?;
        add_output!(f, "Card", self.card, defaults.card);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// Takes hex-color input and converts it to a Color.
    /// # Errors
    /// Errors if the hex color is invalid
    pub fn from_hex(hex: &impl ToString) -> Result<Self, Error> {
        let hex = hex.to_string();
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(Error::InvalidLength);
        }
        Ok(Self {
            red: u8::from_str_radix(&hex[0..=1], 16)?,
            green: u8::from_str_radix(&hex[2..=3], 16)?,
            blue: u8::from_str_radix(&hex[4..=5], 16)?,
        })
    }

    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
