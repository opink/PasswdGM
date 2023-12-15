pub fn mersenne_hash(seed: &str) -> usize {
    let mut hash:usize = 0;
    for (i,c) in seed.chars().enumerate() {
        hash += (i + 1) * (c as usize);
    }
    (hash % 127).pow(3) - 1
}