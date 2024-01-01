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
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;

    let stuff: Vec<inventory::Data> = client.expect("REASON")
    .inventory()
    .find_many(vec![
        inventory::id::lt("10".to_string())
    ])
    .exec()
    .await
    .unwrap();

    println!("{:?} {:?}", action, stuff)
}

#[derive(Debug)]
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