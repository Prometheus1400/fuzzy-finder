mod result_item;
mod score;
mod ui;

use ui::UI;
use std::io::{self, Read};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_buffer = String::new();
    io::stdin().read_to_string(&mut input_buffer).unwrap();
    let items: Vec<String> = input_buffer.lines().map(|s| String::from(s.trim_start())).collect();
    
    let mut ui = UI::new(items);
    ui.start().await
}
