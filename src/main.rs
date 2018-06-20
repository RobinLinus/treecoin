// mod primitives;
// mod blockchain;
extern crate primitives;
use primitives::KeyPair;

fn main() {
	let key_pair = KeyPair::generate().unwrap();
    println!("Hello World! {:?}", key_pair.public_key.to_hex() );
}
