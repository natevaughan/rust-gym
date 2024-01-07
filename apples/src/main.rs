#[allow(warnings, unused)]
mod prisma;
#[macro_use]
extern crate enum_display_derive;

use std::env;
use prisma::{PrismaClient, inventory, apple};
use prisma_client_rust::NewClientError;
use uuid::Uuid;
use std::fmt::Display;
 
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: ./apples <create|read> <name> <color>");
    }

    let action = parse_action(args[1].to_owned()).expect("Please select either \"create\" or \"read\" for the first agrument");
    let client_res: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    let client = client_res.expect("Error creating client");

    match action {
        Action::READ => {
            let all_inventory: Vec<inventory::Data> = client
                .inventory()
                .find_many(vec![])
                .with(inventory::apple::fetch())
                .exec()
                .await
                .unwrap();
            println!("Here is the current inventory of apple varieties");
            for inventory in all_inventory {
                let apple = inventory.apple.unwrap();
                println!("{} {} {}", inventory.count, apple.variety_name, apple.color);
            }
            
        }
        Action::CREATE => {
            if args.len() < 5 {
                panic!("Usage: CREATE <variety_name> <color> <inventory_count>");
            }
            let apple = create_apple(&client, &args[2], &args[3]).await;
            let inventory = create_inventory(&client, &apple.id, &args[4]).await;

            println!("Created apple {} {} {}", apple.variety_name, apple.color, inventory.count);
        }
    }
}

async fn create_apple(client: &PrismaClient, name: &str, color: &str) -> apple::Data {
    let apple = client
                .apple()
                .create(
                    Uuid::new_v4().to_string(),
                    name.to_string(),
                    vec![
                        apple::color::set(color.to_string()),
                    ]
                )
                .exec()
                .await;
    apple.unwrap()
}

async fn create_inventory(client: &PrismaClient, apple_id: &str, count: &str) -> inventory::Data {
    let count = count.parse::<i32>().expect("Invalid integer for count");
    let inventory = client
        .inventory()
        .create(
            Uuid::new_v4().to_string(),
            apple::id::equals(apple_id.to_string()),
            vec![
                inventory::count::set(count),
            ]
        )
        .exec()
        .await;
    inventory.unwrap()
}

#[derive(Debug)]
#[derive(Display)]
enum Action {
    CREATE,
    READ
}

fn parse_action(action: String) -> Option<Action> {
    return match action.to_lowercase().as_ref() {
        "create" => Some(Action::CREATE),
        "read" => Some(Action::READ),
        _ => None
    }
}