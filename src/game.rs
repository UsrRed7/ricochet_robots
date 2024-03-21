#![allow(dead_code)]

const MAX_SHAPES: usize = 20;
const MAX_COLORS: usize = 20;

mod color {
    use serde::{Serialize, Deserialize};
    use core::fmt;
    use super::MAX_COLORS;

    /// Flexible struct to allow for up to MAX_COLORS unique colors
    pub struct Color<'a> {
        pallet: &'a ColorPallet,
        id: u8,
    }
    
    impl<'a> fmt::Display for Color<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.pallet.get_name(self.id))
        }
    }

    impl<'a> Color<'a> {
        pub fn name(&self) -> &str {
            self.pallet.get_name(self.id)
        }

        pub fn rgba(&self) -> [u8; 4] {
            self.pallet.get_rgba(self.id)
        }
    }

    #[derive(Serialize, Deserialize)]
    struct ColorPallet {
        names: [(String, [u8; 4]); MAX_COLORS],
        oob: (String, [u8; 4]),
    }

    impl ColorPallet {
        fn get_entry(&self, id: u8) -> &(String, [u8; 4]) {
            if id as usize > MAX_COLORS {
                &self.oob
            } else {
                &self.names[id as usize]
            }
        }

        fn get_name(&self, id: u8) -> &str {
            &self.get_entry(id).0
        }

        fn get_rgba(&self, id: u8) -> [u8; 4] {
            self.get_entry(id).1
        }
    }
}

mod shape {
    use serde::{Serialize, Deserialize};
    use core::fmt;
    use super::MAX_SHAPES;

    /// Flexible struct to allow for up to MAX_SHAPES unique shapes
    pub struct Shape<'b> {
        shape_sources: &'b ShapeSources,
        shape: u8,
    }
    
    impl<'b> Shape<'b> {
        // pub fn get_color
    }
    
    #[derive(Serialize, Deserialize)]
    struct ShapeSources {
        
    }
}

pub mod board {
    use crate::game::{color::*, shape::*};

    enum Spot<'a, 'b> {
        Target { color: Color<'a>, shape: Shape<'b> },
        Bouncer { color: Color<'a>, direction: bool },
        Empty,
    }

    struct Board<'a> {
        background: &'a str, // Currently just path to background
        // size: (u8, u8),
    }
}

struct Bot {}
