pub struct World {
    pub count: u64,
}

impl World {
    pub fn new() -> World {
        World {
            count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.count += 1;
    }
}
