#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: 16, y: 16) -> Self {
        Self { x, y }
    }

    pub fn tranform(&self, direction: Direction, times: u16) -> Self {
        let times = times as i16;
        let tranformation = match direction {
            Direction::Up => (0, -times),
            Direction::Right => (times, 0),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0),
        };

        Self::new(
            Self::tranform_value(self.x, tranformation.0),
            Self::tranform_value(self.y, tranformation.1),
            )
    }

    fn tranform_value(value: u16, by: i16) -> u16 {
        if by.is_negative() && by.abs() as u16 > value {
            panic!("Transforming value {} by {} would result in a negative number", value, by);
        } else {
            (value as i16 + by) as u16
        }
    }
}
