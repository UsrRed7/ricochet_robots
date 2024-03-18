mod color {
    use core::fmt;

    /// Flexible struct to allow for up to 256 unique "colors"
    #[derive(PartialEq, Debug)]
    pub struct ColorID {
        color: u8,
    }

    impl ColorID {
        // comparison, casting
    }

    impl fmt::Display for ColorID {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}",
                match self.color {
                    0 => "Red",
                    1 => "Yellow",
                    2 => "Green",
                    3 => "Blue",
                    4 => "Black",
                    5 => "Silver",
                    _ => "Unknown",
                }
            )
        }
    }

    struct Pallet {
        // Be able to switch, import, change?
    }
}

pub mod Board {
    use crate::game::color::ColorID;

    enum Shape {
        Circle,
        Triange,
        Square,
        Hexagon,
    }

    enum Spot {
        Target { color: ColorID, shape: Shape },
        Bouncer { color: ColorID, direction: bool },
        Empty,
    }

    struct Board<'a> {
        background: &'a str, // Currently just path to background
        // size: (u8, u8),
    }
}

struct Bot {}
