use std::fs;

fn main() {
    let sjis = fs::read("sjis.txt").expect("failed to read file");
    let (utf, _, _) = encoding_rs::SHIFT_JIS.decode(&sjis);

    println!("{}", utf);
}
