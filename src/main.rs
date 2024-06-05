use secrets::SecretVec;

fn main() {
    let secret = SecretVec::<u8>::new(10, |_| {});
    println!("v {:?}", secret.borrow());
}
