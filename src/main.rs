use crate::friend::Friend;


mod friend;

fn main() {
    println!("Photo Collection Sync Start...");

	let mut friend1 = Friend::default();
	friend1.init("F.1/MASTER".to_string());
}

