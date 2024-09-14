use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    // 環境変数を取得
    let key = "MY_SECRET_KEY";
    match env::var(key) {
        Ok(value) => println!("{}: {}", key, value),
        Err(e) => println!("Couldn't read {}: {}", key, e),
    }
}
