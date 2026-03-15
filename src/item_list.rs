use crate::item;

struct ItemList {
    items: Vec<item::Item>, 
}

impl ItemList {
    pub fn new() -> Self {
        ItemList { items: Vec::new() }
    }

    pub fn remove_top(&mut self) -> item::Item {
        self.items.remove(0)
    }

    pub fn put(&mut self, item : item::Item) {
        // Thought about using binary search to insert
        // however, vectors' insert in rust have O(n) time complexity anyways
        // so it would be O(n + log n) ~ O(n + n) (linear find and insert)
        self.items.push(item);
        self.items.sort_by_key(| item_element : &item::Item | item_element.order());
    }
}

#[cfg(test)]
mod test {
    use crate::item::Item;

    use super::*;

    #[test]
    fn test_put() {
        let mut itemlist = ItemList::new();
        itemlist.put(Item::new("Test name".to_string(), "Test description".to_string()));
    }

    #[test]
    fn test_remove_top() {
        let mut itemlist = ItemList::new();
        itemlist.put(Item::new("Test name".to_string(), "Test description".to_string()));
        let top_elem = itemlist.remove_top();
        assert_eq!(top_elem, Item::new("Test name".to_string(), "Test description".to_string()));
    }

    #[test]
    fn test_remove_top_quality_assess() {
        let mut itemlist = ItemList::new();
        itemlist.put(Item::new("Test name".to_string(), "Test description".to_string()));
        let mut top_elem = itemlist.remove_top();
        top_elem.assess_quality(3.0);

        assert_eq!(top_elem.ef(), item::DEFAULT_EF - 0.14);
        assert_eq!(top_elem.order(), 1);
        assert_eq!(top_elem.repetitions(), 1);
    }

    #[test]
    fn test_remove_top_quality_assess_add_item() {
        let mut itemlist = ItemList::new();
        itemlist.put(Item::new("Test name".to_string(), "Test description".to_string()));
        let mut top_elem = itemlist.remove_top();
        top_elem.assess_quality(3.0);

        let top_elem_clone = top_elem.clone();
        assert_eq!(top_elem.ef(), item::DEFAULT_EF - 0.14);
        assert_eq!(top_elem.order(), 1);
        assert_eq!(top_elem.repetitions(), 1);
        itemlist.put(top_elem);
        itemlist.put(Item::new("Test name 2".to_string(), "Test description 2".to_string()));
        assert_eq!(itemlist.items, vec![Item::new("Test name 2".to_string(), "Test description 2".to_string()), top_elem_clone]);

    }
}
