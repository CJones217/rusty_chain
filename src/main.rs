use rustychainlib::*;

fn main() {
    println!("Hello, world!");
    let block = Block::new(2,4,vec![0;32],6, "test block".to_owned());
    println!("{:?}",&block);
    let block_hash = block.hash;
    println!("{:?}", &block_hash);
}
//