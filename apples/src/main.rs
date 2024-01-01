#[allow(warnings, unused)]
mod prisma;

use std::env;
use prisma::{PrismaClient, inventory};
use prisma_client_rust::NewClientError;
 
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
            let stuff: Vec<inventory::Data> = client
            .inventory()
            .find_many(vec![
                inventory::id::lt("10".to_string())
            ])

            .with(inventory::apple::fetch())
            .exec()
            .await
            .unwrap();
            println!("Here is the current inventory of apple varieties {:?}", stuff);
            for inventory in stuff {
                let apple = inventory.apple.unwrap();
                println!("{} {} {}", inventory.count, apple.variety_name, apple.color);
            }
        }
        Action::CREATE => {
            if args.len() < 5 {
                panic!("Usage: CREATE <variety_name> <color> <inventory_count>");
            }
            println!("Created apple {} {}", args[2], args[3]);
        }
    }
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