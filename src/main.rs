fn main() {
    env_logger::init();
    minecraft::fetch_assets("1.8.8").unwrap();
    println!("Hello, world!");
}
