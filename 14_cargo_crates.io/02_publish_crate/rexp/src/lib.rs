//! # Art
//!
//! A library for modeling artistic concepts

pub use self::kinds::SecondaryColor; // re-exporting allows user to directly find "SecondaryColor" without going through "kinds"
pub use self::utils::mix; // re-exporting "utils::mix" allows user of library to directly find "mix" without having to go through "utils"

pub mod kinds {
    /// The primary colors according to the RYB color model
    #[derive(PartialEq)]
    pub enum PrimaryColor{
        Red,
        Yellow,
        Blue
    }

    /// The secondary colors according to the RYB color model
    #[derive(PartialEq, Debug)]
    pub enum SecondaryColor{
        Orange,
        Green,
        Purple
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        let mut col = 0;
        if c1 == PrimaryColor::Red || c2 == PrimaryColor::Red {
            col = col | 1;
        }
        if c1 == PrimaryColor::Yellow || c2 == PrimaryColor::Yellow {
            col = col | 2;
        }
        if c1 == PrimaryColor::Blue || c2 == PrimaryColor::Blue {
            col = col | 4;
        }

        if col == 3 {
            SecondaryColor::Orange
        } else if col == 5 {
            SecondaryColor::Purple
        } else if col == 6 {
            SecondaryColor::Green
        } else {
            panic!("invalid combination");
        }
    }
}