use rustychainlib::*;

fn main() {
    println!("Hello, world!");
    let block = Block::new(0,0,vec![0;32],0);

    println!("{:?}",&block);
}
