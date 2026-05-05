use crate::item::Item;
use postgrest::Postgrest;
use std::env;

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

    pub async fn put(&self, item : &Item) {
        let res = self.client.from("sm2_lc_items")
            .insert(serde_json::to_string(item).unwrap())
            .execute()
            .await
            .unwrap();
    }
    pub async fn update(&self, item : &Item) {
        let res = self.client.from("sm2_lc_items")
            .eq("name", item.name())
            .update(serde_json::to_string(item).unwrap())
            .select("*")
            .execute()
            .await.unwrap();
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

    #[tokio::test]
    async fn add_item() {
        let db = DB::new();
        let item = Item::new("Add item test".to_string(), "https://example.com/addingitem".to_string());
        db.put(&item).await;
        let items = db.get_all_items().await.unwrap();
        println!("{:?}", items);
        assert!(items.contains(&item));
    }

    #[tokio::test]
    async fn update_item() {
        let db = DB::new();
        let mut item = Item::new("Add item to update".to_string(), "https://example.com/addingitem".to_string());
        db.put(&item).await;
        item.assess_quality(5.0);
        db.update(&item).await;
        let items = db.get_all_items().await.unwrap();
        assert!(items.contains(&item));
    }
} 
