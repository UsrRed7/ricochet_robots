/// Flexible struct to allow for up to 256 unique "colors"
struct Color {
    color: u8,
}

impl Color {
    pub fn get_name(&self) -> &str {
        match self.color {
            0 => "Red",
            1 => "Yellow",
            2 => "Green",
            3 => "Blue",
            4 => "Black",
            5 => "Silver",
            255 => "Other",
            _ => "Unknown",
        }
    }    
}

pub mod Board {
    use crate::game::Color;

    enum Shape {
        Circle,
        Triange,
        Square,
        Hexagon,
    }

    enum Spot {
        Target { color: Color, shape: Shape },
        Bouncer { color: Color, direction: bool },
        Empty,
    }

    struct Board<'a> {
        background: &'a str, // Currently just path to background
        // size: (u8, u8),

    }
}

struct Bot {}
