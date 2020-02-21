extern crate rand;

fn main() {
    use rand::Rng;
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-";
    const LEN: usize = 8;
    let mut rng = rand::thread_rng();
    let ret: String = (0..LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARS.len());
            CHARS[idx] as char
        })
        .collect();
    println!("{:?}", ret)
}
