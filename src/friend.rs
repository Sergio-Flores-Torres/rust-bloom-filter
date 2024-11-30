use rand::Rng;

#[derive(Debug, Default)]
pub struct Friend {
	/// A dynamic array of photo identifiers as unique integers.
	pub photos: Vec<i32>,

	/// This friend identifier
	pub name: String,
}

impl Friend {
    pub fn init(&mut self, name: String) -> () {
        self.photos = create_random_photo_collection_ids();
		self.name = name;

		println!("{}, -> {:?}", self.name, self.photos);
    }
}

/// Generates a random number of random integer photo id's for use by a friend.
fn create_random_photo_collection_ids() -> Vec<i32> {
	let max_elems: i32 = env!("MAX_NUM_OF_ELEMS_PER_FRIEND").parse().unwrap();
	let mut v: Vec<i32> = Vec::new();

	// Generate random number in the range [1, 100]. This is the number of photos the friend will have.
   	let mut num = rand::thread_rng().gen_range(1..(max_elems + 1));

	while num > 0 {
		// Generate random number in the range [1, 100]. This is a particular photo id.
		let elem = rand::thread_rng().gen_range(1..(max_elems + 1));

		// However, it may generate duplicates, within the same friend, which makes no sense for the scenario
		match v.iter().find(|&&x| x == elem) {
			Some(_e) => continue,
			None => {
				v.push(elem);
				num -= 1;
			} 
		}
	}

	v
}
