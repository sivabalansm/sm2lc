mod item;

struct ItemList {
    items: Vec<item::Item>, 
}

impl ItemList {
    pub fn new() -> Self {
        ItemList { items: Vec::new() }
    }

    pub fn remove_top(mut self) -> item::Item {
        self.items.remove(0)
    }

    pub fn put(&mut self, item : item::Item) {
        // Thought about using binary search to insert
        // however, vectors in rust have O(n) time complexity anyways
        self.items.push(item);
        self.items.sort_by_key(| item | item.get_order());
    }
}


fn main() {
    println!("Hello, world!");
}



