#![allow(dead_code)]

const MAX_SHAPES: usize = 20;
const MAX_COLORS: usize = 20;

mod color {
    use super::MAX_COLORS;
    use core::fmt;
    use serde::{Deserialize, Serialize};

    /// Flexible struct to allow for up to MAX_COLORS unique colors
    pub struct Color<'a> {
        pallet: &'a ColorPallet,
        num: u8,
    }

    impl<'a> fmt::Display for Color<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.name())
        }
    }

    impl<'a> Color<'a> {
        pub fn name(&self) -> &str {
            self.pallet.get_name(self.num)
        }

        pub fn rgba(&self) -> [u8; 4] {
            self.pallet.get_rgba(self.num)
        }
    }

    #[derive(Serialize, Deserialize)]
    struct ColorPallet {
        names: [(String, [u8; 4]); MAX_COLORS],
        unknown: (String, [u8; 4]),
    }

    impl ColorPallet {
        fn get_entry(&self, num: u8) -> &(String, [u8; 4]) {
            if num as usize >= MAX_COLORS {
                &self.unknown
            } else {
                &self.names[num as usize]
            }
        }

        fn get_name(&self, num: u8) -> &str {
            &self.get_entry(num).0
        }

        fn get_rgba(&self, num: u8) -> [u8; 4] {
            self.get_entry(num).1
        }
    }
}

mod shape {
    use super::MAX_SHAPES;
    use core::fmt;
    use serde::{Deserialize, Serialize};

    /// Flexible struct to allow for up to MAX_SHAPES unique shapes
    pub struct Shape<'a> {
        icons: &'a Icons,
        num: u8,
    }

    impl<'a> fmt::Display for Shape<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.name())
        }
    }

    impl<'a> Shape<'a> {
        pub fn name(&self) -> &str {
            self.icons.get_name(self.num)
        }

        pub fn icon_path(&self) -> &str {
            self.icons.get_icon_path(self.num)
        }

        pub fn mask_path(&self) -> &str {
            self.icons.get_mask_path(self.num)
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Icon {
        name: String,
        icon_path: String,
        mask_path: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Icons {
        shapes: [Icon; MAX_SHAPES],
        unknown: Icon,
    }

    impl Icons {
        fn get_icon(&self, num: u8) -> &Icon {
            if num as usize >= MAX_SHAPES {
                &self.unknown
            } else {
                &self.shapes[num as usize]
            }
        }

        fn get_name(&self, num: u8) -> &str {
            &self.get_icon(num).name
        }

        fn get_icon_path(&self, num: u8) -> &str {
            &self.get_icon(num).icon_path
        }

        fn get_mask_path(&self, num: u8) -> &str {
            &self.get_icon(num).mask_path
        }
    }
}

pub mod board {
    use crate::game::{color::Color, shape::Shape};

    enum Spot<'a> {
        Target { color: Color<'a>, shape: Shape<'a> },
        Bouncer { color: Color<'a>, direction: bool },
        Empty,
    }

    struct Board<'a> {
        background: &'a str, // Currently just path to background
        // size: (u8, u8),
    }
}

struct Bot {}
