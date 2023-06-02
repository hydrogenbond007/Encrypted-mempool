extern crate crypto;
extern crate tlock_age;


use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::{Duration, SystemTime};


fn txn_hash(data: &[u8] , duration:Duration) ->Vec<u8> {
	let mut hash = Sha256::new();
	let now = SystemTime:now();

	
	std::thread::sleep(duration);

	hash.input(data);
	
	hash.input(now.elapse().unwrap().as_secs().to_string(0.as_bytes());
	let mut output = vec![Ou8;hash.output_bytes()];
}



