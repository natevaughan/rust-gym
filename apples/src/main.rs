#[allow(warnings, unused)]
mod prisma;
 
use prisma::{PrismaClient, inventory};
use prisma_client_rust::NewClientError;
 
#[tokio::main]
async fn main() {
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;

    let stuff: Vec<inventory::Data> = client.expect("REASON")
    .inventory()
    .find_many(vec![
        inventory::id::lt("10".to_string())
    ])
    .exec()
    .await
    .unwrap();

    println!("{:?}", stuff)
}