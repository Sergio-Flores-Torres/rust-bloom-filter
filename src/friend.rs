use rand::Rng;
use bloomfilter::Bloom;

#[derive(Debug, Default)]
pub struct Friend {
	/// A dynamic array of photo identifiers as unique integers.
	pub photos: Vec<i32>,

	/// This friend identifier
	pub name: String,
}

impl Friend {
	/// Initializes the id's collection and the name for this friend, for human readability purposes
    pub fn init(&mut self, name: String) -> () {
        self.photos = create_random_photo_collection_ids();
		self.name = name;

		println!("{} starts with these photo id's -> {:?}", self.name, self.photos);
    }

	/// The Master node uses this, to send to its slaves
	pub fn init_bloom(&mut self) -> Bloom<i32> {
		let num_items: usize = env!("FILTER_ITEMS").parse().unwrap();
		let fp_rate: f64 = env!("FALSE_POSITIVE_RATE").parse().unwrap();
		
		let mut bloom = Bloom::new_for_fp_rate(num_items, fp_rate);

		for photo in self.photos.iter() {
			bloom.set(photo); 
		}

		bloom
	}

	/// The slaves use this to find the id's they need to send back to Master
	pub fn check_bloom(&self, bloom: &Bloom<i32>, master: &Friend) {
		let mut v: Vec<i32> = Vec::new();

		for photo in self.photos.iter() {
			if !bloom.check(photo) {
				v.push(*photo);
			}
		}

		master.report(&self.name, v);
	}

	/// Simulation of network messages ocurring
	pub fn report(&self, name: &String, v: Vec<i32>) {
		println!("{} reports these id's missing -> {:?}", name, v);		
	}

	/// Obtains a Blake hash for the byte concat of all the photo id's in this friend.
	pub fn hash(&mut self) -> [u8;32] {
		let mut v: Vec<u8> = Vec::new();

		// Since the hash uses a byte array, the order matters
		self.photos.sort();

		// Convert i32 vector to u8 slice to pass to blake
		for photo in self.photos.iter() {
			let bytes = photo.to_ne_bytes(); // platform native endianesss
			v.extend(bytes);
		}
		
		let data = v.as_slice();	
		let mut result_256 = [0; 32];
		blake::hash(256, data, &mut result_256).unwrap();

		println!("Blake {} -> {:?}", self.name, result_256);		

		result_256
	}

	/// Compare the master hash with this friend hash to see if they have the same items...
	pub fn compare_hashes(&mut self, master_hash: [u8; 32]) -> bool {
		if self.hash() == master_hash {
			true
		}
		else {
			false
		} 
	}
}

/// Generates a random number of random integer photo id's for use by a friend.
fn create_random_photo_collection_ids() -> Vec<i32> {
	let max_elems: i32 = env!("MAX_NUM_OF_ELEMS_PER_FRIEND").parse().unwrap();
	let mut v: Vec<i32> = Vec::new();

	println!("Max elems = {} ", max_elems);

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
