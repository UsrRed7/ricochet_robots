#![allow(dead_code)]

pub mod board {
    use bit_vec::BitVec;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Icon {
        name: String,
        icon_path: String,
        mask_path: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Icons {
        shapes: [Icon; 16],
        unknown: Icon,
    }

    impl Icons {
        fn get_icon(&self, num: u8) -> &Icon {
            if num as usize >= 16 {
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

    #[derive(Serialize, Deserialize)]
    pub struct ColorPallet {
        names: [(String, [u8; 4]); 16],
        unknown: (String, [u8; 4]),
    }

    impl Default for ColorPallet {
        fn default() -> Self {
            Self {
                names: Default::default(), // TODO Define actual default colors
                unknown: ("invalid".to_string(), [0, 0, 0, 0xff]),
            }
        }
    }

    impl ColorPallet {
        fn get_entry(&self, num: u8) -> &(String, [u8; 4]) {
            if num as usize >= 16 {
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

    trait Colored {
        fn color<'a, 'b>(&'a self, pallet: &'b ColorPallet) -> &'b str;
        fn rgba(&self, pallet: &ColorPallet) -> [u8; 4];
    }

    /// Representation of each target type  
    /// Format: 0bKKKKCCCC  
    /// K: Kind (Those with special properties below)  
    /// 15 (0xfX) is wild (any color valid)  
    /// C: Color  
    struct Target {
        target: u8,
        x: u8,
        y: u8,
    }

    impl Colored for Target {
        fn color<'a, 'b>(&'a self, pallet: &'b ColorPallet) -> &'b str {
            pallet.get_name(self.target & 0xf)
        }

        fn rgba(&self, pallet: &ColorPallet) -> [u8; 4] {
            pallet.get_rgba(self.target & 0xf)
        }
    }

    impl Target {
        fn kind<'a, 'b>(&'a self, icons: &'b Icons) -> &'b str {
            icons.get_name(self.target & 0xf0)
        }

        fn shape<'a, 'b>(&'a self, icons: &'b Icons) -> &'b Icon {
            icons.get_icon(self.target & 0xf0)
        }

        // Checks first if the bot is on the correct spot, then true if wild, otherwise compare colors
        fn win(&self, bot: &Bot) -> bool {
            if self.x == bot.x && self.y == bot.y {
                if self.target & 0xf0 == 0xf0 {
                    return true;
                }
                self.target & 0xf == bot.bot &0xf
            } else {
                false
            }
        }
    }

    /// Representation of each feature type  
    /// Format: 0bKKKKCCCC  
    /// K: Kind (Each has different behaviour)  
    /// 0 Bouncer: if colors match does nothing, otherwise bounces bots for no extra  
    /// C: Color  
    struct Feature {
        feature: u8,
        x: u8,
        y: u8,
    }

    impl Colored for Feature {
        fn color<'a, 'b>(&'a self, pallet: &'b ColorPallet) -> &'b str {
            pallet.get_name(self.feature & 0xf)
        }

        fn rgba(&self, pallet: &ColorPallet) -> [u8; 4] {
            pallet.get_rgba(self.feature & 0xf)
        }
    }

    /// Representation of each bot type  
    /// Format: 0bKKKKCCCC  
    /// K: Kind (Some may have special properties)  
    /// 0 Basic: Move straight in any orthogonal direction till an object is hit  
    /// C: Color  
    struct Bot {
        bot: u8,
        x: u8,
        y: u8,
    }

    impl Colored for Bot {
        fn color<'a, 'b>(&'a self, pallet: &'b ColorPallet) -> &'b str {
            pallet.get_name(self.bot & 0xf)
        }

        fn rgba(&self, pallet: &ColorPallet) -> [u8; 4] {
            pallet.get_rgba(self.bot & 0xf)
        }
    }

    struct Board {
        color_pallet: ColorPallet,
        icons: Icons,
        background: String, // Currently just path to background
        /// Max of N by N
        size: u8,
        vertical_walls: Vec<BitVec>,
        horizontal_walls: Vec<BitVec>,
        targets: Vec<Target>,
        features: Vec<Feature>,
    }

    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    /// Compact object for move data  
    /// Up to 6 special types (S)  
    /// 4 directions (D)  
    /// 0bSSSSSSDD  
    struct Move(u8);

    impl Move {
        /// Currently no ability to make one with special flags TODO
        fn new(dir: Direction) -> Self {
            Move(match dir {
                Direction::Up => 0,
                Direction::Right => 1,
                Direction::Down => 2,
                Direction::Left => 3,
            })
        }

        fn direction(&self) -> Direction {
            match self.0 & 0b11 {
                0 => Direction::Up,
                1 => Direction::Right,
                2 => Direction::Down,
                _ => Direction::Left,
            }
        }

        /// TODO: will need more, but wait till there are actually special moves to impliment
        fn special(&self) -> bool {
            self.0 & 0b11111100 > 0
        }
    }

    struct BoardState(Vec<Bot>);

    impl BoardState {
        fn adjacent_states(&self, board: &Board) {
            for bot in self.0.iter() {

            }
        }
    }
}
