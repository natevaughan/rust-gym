use reqwest;

#[tokio::main]
async fn main() {
	let client = reqwest::Client::new();
	let body = fetch_body(client).await;

	println!("{:?}", body.unwrap())
}

async fn fetch_body(client: reqwest::Client) -> Result<String, reqwest::Error> {
	Ok(client.get("https://catfact.ninja/fact") 
		.header("Accept", "application/xml")
		.send()
		.await?
		.text()
		.await?)
}