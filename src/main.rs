use pkru_unsafe_lib;

fn main() {
    let mut vec = vec![0u64];
    println!("Value of Vec before call: {}", vec[0]);
    pkru_unsafe_lib::write_vec_wrapper(&mut vec);
    println!("Value of Vec after call: {}", vec[0]);
}
