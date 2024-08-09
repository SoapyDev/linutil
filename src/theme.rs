use clap::builder::PossibleValue;
use clap::ValueEnum;
use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub dir_color: Color,
    pub cmd_color: Color,
    pub dir_icon: &'static str,
    pub cmd_icon: &'static str,
    pub success_color: Color,
    pub fail_color: Color,
}

// Clarify what is the default theme
impl Default for Theme {
    fn default() -> Self {
        Self {
            dir_color: Color::Blue,
            cmd_color: Color::Rgb(204, 224, 208),
            dir_icon: "  ",
            cmd_icon: "  ",
            fail_color: Color::Rgb(199, 55, 44),
            success_color: Color::Rgb(5, 255, 55),
        }
    }
}

// List of themes
// This is better than the previous list because we eliminate the risk of
// indexing out of bounds even if we modify the list and its directly
// reflected in the --help output
#[derive(Clone, Default, Debug)]
pub enum ThemeType {
    #[default]
    Default,
    Compatible,
    Colorblind,
}

// This is for the clap parser to be able to parse the theme type
impl ValueEnum for ThemeType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Default, Self::Compatible, Self::Colorblind]
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };

        match input.as_str() {
            "default" => Ok(Self::Default),
            "compatible" => Ok(Self::Compatible),
            "colorblind" => Ok(Self::Colorblind),
            _ => Err(format!("Invalid theme type: {input}")),
        }
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Default => PossibleValue::new("default"),
            Self::Compatible => PossibleValue::new("compatible"),
            Self::Colorblind => PossibleValue::new("colorblind"),
        })
    }
}

// Convert ThemeType into Theme
impl Into<Theme> for ThemeType {
    fn into(self) -> Theme {
        match self {
            ThemeType::Default => Theme::default(),
            ThemeType::Compatible => Theme {
                dir_color: Color::Blue,
                cmd_color: Color::LightGreen,
                dir_icon: "[DIR]",
                cmd_icon: "[CMD]",
                success_color: Color::Green,
                fail_color: Color::Red,
            },
            ThemeType::Colorblind => {
                // Bang Wong color palette - Slightly tweaked to be lighter for readability
                // https://davidmathlogic.com/colorblind/#%23000000-%23E69F00-%2356B4E9-%23009E73-%23F0E442-%230072B2-%23D55E00-%23CC79A7
                Theme {
                    dir_color: Color::Rgb(86, 180, 233),
                    cmd_color: Color::Rgb(0, 114, 178),
                    dir_icon: "  ",
                    cmd_icon: "  ",
                    fail_color: Color::Rgb(213, 94, 0),
                    success_color: Color::Rgb(0, 158, 115),
                }
            }
        }
    }
}
