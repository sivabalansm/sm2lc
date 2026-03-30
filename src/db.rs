use crate::item;
use postgrest::Postgrest;

const SUPABASE_ENDPOINT : &str = "";
const API_KEY : &str = "";

pub struct DB {
    client: Postgrest,
}

impl DB {
    pub fn new() -> Self {
        DB { client: Postgrest::new(SUPABASE_ENDPOINT).insert_header("apikey", API_KEY) }
    }

    pub async fn get_all_items(&self) -> String {
        let res = self.client.from("sm2_lc_items")
            .select("*")
            .execute()
            .await.unwrap();
        res.text().await.unwrap()
    }

}

