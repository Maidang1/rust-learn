mod file_system;
use file_system::read;

fn main() {
    let mut reader = read::Read::new("src/lib/lib.rs".to_string());
    reader.start_read();
    let data = reader.get_data();
    println!("data: {}", data);
}
