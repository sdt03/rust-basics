#[derive(Debug)]
pub struct Square {
    pub side: u32
}

impl Square {
    pub fn area(&self) -> u32 {
        return self.side * self.side;
    }
}

