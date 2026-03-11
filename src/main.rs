const DEFAULT_EF : f32 = 2.5;

struct Item {
    name : Option<String>,
    description: Option<String>,
    ef : f32,
    order : i32,
    repetitions: i32,
}

impl Default for Item {
    fn default() -> Self {
        Item { name: None, description: None, ef: DEFAULT_EF, order: 0, repetitions: 0 }
    }
}

impl Item {
    pub fn new(name : Option<String>, description : Option<String>) -> Self {
        Item { name, description, ef: DEFAULT_EF, order: 0, repetitions: 0 }
    }

    fn update_order_from_repetitions(&mut self, repetitions : i32) -> f32 {
        if repetitions == 1 {
            1.0
        } else if repetitions == 2 {
            6.0
        } else {
            self.update_order_from_repetitions(repetitions - 1) * self.ef
        }
    }

    pub fn assess_quality(&mut self, q : f32) {
        self.ef = self.ef + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02));
        if self.ef < 1.3 {
            self.ef = 1.3;
        }
        if q < 3.0 {
            self.order = 1;
            self.repetitions = 0;
        } else {
            self.repetitions += 1;
            self.order = self.update_order_from_repetitions(self.repetitions).round() as i32;
        }
    }

}
fn main() {
    let s : String = String::from("hi");
    println!("Hello, world!");
}
