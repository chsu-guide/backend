use unitracker_chsu::schedule::request;
#[tokio::main]
async fn main() {
    let r = request::get_weeks().await;
    dbg!(r);
}
