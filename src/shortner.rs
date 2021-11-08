use harsh::Harsh;
use rand::Rng;
use redis::{self, Commands};

pub struct Shortener {
    connection_url: String,
    generator: Harsh,
}

impl Shortener {
    pub fn new(connection_url: String) -> Shortener {
        let harsh = Harsh::builder().salt("Url shortner").build().unwrap();

        Shortener {
            generator: harsh,
            connection_url,
        }
    }

    pub fn next_id(&mut self, url: String) -> String {
        let mut connection_url = self.connection_url.clone();
        let client = redis::Client::open(connection_url).unwrap();
        let mut con = client.get_connection().unwrap();

        let idx = con.get("idx").unwrap_or(0u64);
        let mut hashed: String;

        loop {
            let mut rng = rand::thread_rng();
            let n1: u64 = rng.gen();
            hashed = self.generator.encode(&[idx, idx + n1]);

            let res = con.get(&hashed).unwrap_or("NULL".to_string());

            if res == "NULL" {
                let _: () = con.set(&hashed, &url).unwrap();
                break;
            }
        }
        let _: () = con.set("idx", idx + 1).unwrap();

        hashed
    }

    pub fn get_value(&mut self, key: String) -> String {
        let mut connection_url = self.connection_url.clone();
        let client = redis::Client::open(connection_url).unwrap();

        let mut con = client.get_connection().unwrap();
        let id = con.get(&key).unwrap_or(String::from("NULL"));
        id
    }
}
