use crate::friend::Friend;

mod friend;

fn main() {
	println!("\n------Photo Collection Sync Test Start...");

	for i in 1..6 {
		test_run(i);
	}
}

fn test_run(num: i32) {
    println!("\n------Test Start {}", num);

	let max_elems = env!("MAX_NUM_OF_ELEMS_PER_FRIEND");
	let num_items = env!("FILTER_ITEMS");
	let fp_rate = env!("FALSE_POSITIVE_RATE");

    println!("CONFIG: Max number of photos per 'friend' = {max_elems}");
    println!("CONFIG: Bloom filter number of items = {num_items}");
    println!("CONFIG: Bloom filter FP rate = {fp_rate}");

	println!("\n------Initializing test data...");
	
	let mut friend1 = Friend::default();
	friend1.init("F.1/MASTER".to_string());

	let mut friend2 = Friend::default();
	friend2.init("F.2/SLAVE".to_string());

	let mut friend3 = Friend::default();
	friend3.init("F.3/SLAVE".to_string());

	let bloom = friend1.init_bloom();	
	println!("\n----Bloom filter for MASTER ready");

	println!("\n----Compare hashes to decide if sync is needed...");
	let master_hash = friend1.hash();

	println!("\n----Next hash...");	

	if !friend2.compare_hashes(master_hash) {
		println!("\n{} does not match -> proceeding with sync", friend2.name);	
		friend2.check_bloom(&bloom, &friend1);
	} else {
		println!("\n{} matches -> skipping sync!", friend2.name);
	}

	println!("\n----Next hash...");	

	if !friend3.compare_hashes(master_hash) {
		println!("\n{} does not match -> proceeding with sync", friend3.name);	
		friend3.check_bloom(&bloom, &friend1);
	} else {
		println!("\n{} matches -> skipping sync!", friend3.name);
	}

	println!("\n----End Test");	
}
