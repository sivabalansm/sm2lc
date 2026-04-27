use crate::item;
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

    pub async fn get_all_items(&self) -> String {
        let res = self.client.from("sm2_lc_items")
            .select("*")
            .execute()
            .await.unwrap();
        res.text().await.unwrap()
    }

}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn item_fields() {
        assert_eq!(item.ef, DEFAULT_EF);
    }
    
} 
