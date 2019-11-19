use rustychainlib::*;

fn main() {
    println!("Hello, world!");
    let block = Block::new(0,now(),vec![0;32],0, "test block".to_owned());
    println!("{:?}",&block);
    let block_hash = block.hashable();
    println!("{:?}", &block_hash);
    let block2 = Block::new(1,now(),block_hash,3,"second block".to_owned());
    println!("{:?}",&block2);
}
//