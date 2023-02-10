#[derive(Debug, Default)]
pub struct Animal {
    pub hungry: bool,
}

impl Animal {
    pub fn feed(&mut self) {
        self.hungry = false;
    }
}