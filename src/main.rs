pub(crate) use serde::{Serialize, Deserialize};

 #[derive(Debug, Serialize, Deserialize)]


 struct Todo {
	#[serde(rename = "userId")]
    user_id: i32,
	id: i32,
	title: String,
	completed: bool,
 }

#[tokio::main]

//This is a basic API Caller. Gets JSON from URL, stored as string
async fn main() -> Result<(), reqwest::Error> {
	let todos: Vec<Todo> = reqwest::Client::new()
	.get ("https://jsonplaceholder.typicode.com/todos")
	.send()
	.await?
	.json()
	.await?;
	println!("{:#?}", todos);

	Ok(())
}