pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(v: i32) {
        if v < 1 || v > 100 {
            panic!("The number will always be between 1 and 100.");
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}