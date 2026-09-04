mod access;
mod config;

#[tokio::main]
async fn main() {
    match access::compare_all_digest().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}")
        }
    }
}
