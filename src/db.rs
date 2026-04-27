use crate::item::Item;
use postgrest::Postgrest;
use std::env;

pub struct DBItem {
    item: Item,
    client: Postgrest,
}

impl DBItem {
    pub fn from(item : Item, client : Postgrest) -> Self {
        DBItem { item, client }
    }

    pub fn assess_quality(&mut self, q : f32) {
        self.item.assess_quality(q);
        self.client
            .from("sm2_lc_items")
            .eq("name", self.item.name())
            .update(serde_json::to_string(&self.item).unwrap())
            .select("*");
    }
}

pub struct DB {
    client: Postgrest,
}

impl DB {
    pub fn new() -> Self {
        let supabase_endpoint : String = env::var("SUPABASE_ENDPOINT").unwrap();
        let api_key : String = env::var("API_KEY").unwrap();
        DB { client: Postgrest::new(supabase_endpoint).insert_header("apikey", api_key) }
    }

    pub async fn get_all_items(&self) -> Result<Vec::<Item>, reqwest::Error> {
        let res = self.client.from("sm2_lc_items")
            .select("*")
            .execute()
            .await.unwrap();
        res.json::<Vec::<Item>>().await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn more_than_one_item() {
        let db = DB::new();
        let items = db.get_all_items().await.unwrap();
        assert_ne!(items.len(), 0);
    }
    
} 
