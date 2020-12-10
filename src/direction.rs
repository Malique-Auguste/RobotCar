use std::fmt;

#[derive(Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,

    ForwardLeft,
    ForwardRight,
    BackwardLeft,
    BackwardRight,

    None,
    Stop
}

impl Direction {
    pub fn from_str(input: &str) -> Direction {
        match input {
            "F" => Direction::Forward,
            "B" => Direction::Backward,
            "L" => Direction::Left,
            "R" => Direction::Right,

            "FR" => Direction::ForwardRight,
            "FL" => Direction::ForwardLeft,
            "BR" => Direction::BackwardRight,
            "BL" => Direction::BackwardLeft,
            "S" => Direction::Stop,

            e => {
                println!("{} is not a suitable input", e);
                Direction::None
            }
        }
    }
}

impl fmt::Display for Direction  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Forward => write!(f, "F"),
            Direction::Backward => write!(f, "B"),
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),

            Direction::ForwardLeft => write!(f, "FL"),
            Direction::ForwardRight => write!(f, "FR"),
            Direction::BackwardLeft => write!(f, "BL"),
            Direction::BackwardRight => write!(f, "BR"),

            Direction::None => write!(f, "N"),
            Direction::Stop => write!(f, "S")
        }
    }
}

impl fmt::Debug for Direction  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Forward => write!(f, "Forward"),
            Direction::Backward => write!(f, "Backward"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),

            Direction::ForwardLeft => write!(f, "Forward-Left"),
            Direction::ForwardRight => write!(f, "Forward-Right"),
            Direction::BackwardLeft => write!(f, "Backward-Left"),
            Direction::BackwardRight => write!(f, "Backward-Right"),

            Direction::None => write!(f, "None"),
            Direction::Stop => write!(f, "Stop")
        }
    }
}