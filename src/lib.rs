
pub fn gen_ascii_keys(length : u32) -> Vec<String> {
    let mut keys : Vec<String> = Vec::new();
    let max = 128u32.pow(length);

    for i in 0..max {
        let mut value = i;
        let mut key = String::new();

        for j in (0..length).rev() {
            let digit = value / 128u32.pow(j);
            value = value - digit * 128u32.pow(j);
            key.push_str(format!("{}", (digit as u8) as char).as_str());
        }

        keys.push(key);
    }

    keys
}
