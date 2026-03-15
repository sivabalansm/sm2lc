pub const DEFAULT_EF : f32 = 2.5;

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    name : String,
    description: String,
    ef : f32,
    order : i32,
    repetitions: i32,
}


impl Item {
    pub fn new(name : String, description : String) -> Self {
        Item { name, description, ef: DEFAULT_EF, order: 0, repetitions: 0 }
    }

    pub fn order(&self) -> i32 {
        self.order
    }

    pub fn repetitions(&self) -> i32 {
        self.repetitions
    }

    pub fn ef(&self) -> f32 {
        self.ef
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
        self.ef = ((self.ef + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02))) * 100.0).round() / 100.0;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn item_fields() {
        let item = Item::new(String::from("Test item"), String::from("Test description"));
        assert_eq!(item.name, "Test item");
        assert_eq!(item.description, "Test description");
        assert_eq!(item.order, 0);
        assert_eq!(item.repetitions, 0);
        assert_eq!(item.ef, DEFAULT_EF);
    }
    
    #[test]
    fn update_repitions() {
        let mut item = Item::new(String::from("Test item"), String::from("Test description"));
        item.assess_quality(4.0);
        assert_eq!(item.repetitions, 1);
    }
    
    #[test]
    fn check_new_order_base_case() {
        let mut item = Item::new(String::from("Test item"), String::from("Test description"));
        item.assess_quality(4.0);
        assert_eq!(item.order, 1);
        item.assess_quality(4.0);
        assert_eq!(item.order, 6);
    }
    
    #[test]
    fn check_new_order_ef_mut() {
        let mut item = Item::new(String::from("Test item"), String::from("Test description"));
        item.assess_quality(4.0);
        item.assess_quality(4.0);
        item.assess_quality(4.0);
        assert_eq!(item.order, (6.0 * item.ef).round() as i32);
    }
    
    #[test]
    fn check_ef_after_5_assess_quality() {
        let mut item = Item::new(String::from("Test item"), String::from("Test description"));
        item.assess_quality(5.0);
        assert_eq!(item.ef, DEFAULT_EF + 0.1);
    }
    
    #[test]
    fn check_ef_after_4_assess_quality() {
        let mut item = Item::new(String::from("Test item"), String::from("Test description"));
        item.assess_quality(4.0);
        item.assess_quality(4.0);
        item.assess_quality(4.0);
        assert_eq!(item.ef, DEFAULT_EF);
    }
    
    #[test]
    fn check_ef_after_3_assess_quality() {
        let mut item = Item::new(String::from("Test item"), String::from("Test description"));
        item.assess_quality(3.0);
        assert_eq!(item.ef, DEFAULT_EF - 0.14);
    }
    
    #[test]
    fn check_ef_after_2_assess_quality() {
        let mut item = Item::new(String::from("Test item"), String::from("Test description"));
        item.assess_quality(2.0);
        assert_eq!(item.ef, DEFAULT_EF - 0.32);
    }
    
    #[test]
    fn check_ef_after_1_assess_quality() {
        let mut item = Item::new(String::from("Test item"), String::from("Test description"));
        item.assess_quality(1.0);
        assert_eq!(item.ef, DEFAULT_EF - 0.54);
    }
}
