pub mod handlers;

use handlers::{generate_token, revoke_token};
use inquire::Select;

#[tokio::main]
async fn main() {
    loop {
        let options: Vec<&str> = vec!["Generate Token", "Revoke Token", "Exit"];
        let ans = Select::new("Choose one option!", options).prompt().unwrap();

        if ans == "Generate Token" {
            generate_token().await;
        };

        if ans == "Revoke Token" {
            revoke_token().await;
        };
        if ans == "Exit" {
            println!("Exiting...");
            std::process::exit(0);
        }
    }
}
