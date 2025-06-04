use risc0_zkvm::{
    guest::{env, sha::Impl},
    sha::{Digest, Sha256},
};

fn main() {
    let mut input = "hello worlp".as_bytes().to_vec();
    env::read_slice(&mut input);

    println!("Hello, world! (from println)");
    println!("Input 1: {:?}", input);
    println!("Input 1: as slice {:?}", input.as_slice());

    let input_str = String::from_utf8(input.clone()).unwrap();
    println!("Input 1 as string: {:?}", input_str);

    env::log("Hello world from env log!");
    let digest: Digest = *Impl::hash_bytes(input.as_slice());

    env::commit_slice(digest.as_bytes());
}