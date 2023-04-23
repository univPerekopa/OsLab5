fn main() {
    let mut x = vec![0x1u8, 0x10];
    println!("{:x}", x.as_ptr() as usize);
    loop {
        x[0] += 1;
        println!("{:?}", x);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
