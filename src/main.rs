use rand::Rng;

fn main() {
    println!("{}", generate_password())
}

fn generate_password() -> String {
    let mut rng = rand::thread_rng();
    let pass_length = rng.gen_range(10..=15);
    let mut password: Vec<char> = Vec::new();
    for _ in 0..pass_length {
        password.push(char::from(rng.gen_range(33..=126)));
    }
    strengthen(password).iter().collect()
}

fn strengthen(mut pass: Vec<char>) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let mut used_positions: Vec<usize> = Vec::new();
    for i in 0..5 {
        let mut random_position = rng.gen_range(0..pass.len());
        while used_positions.contains(&random_position) {
            random_position = rng.gen_range(0..pass.len());
        }
        used_positions.push(random_position);
        pass[random_position] = get_unique_char(i);
    }
    pass
}

fn get_unique_char(char_type: i32) -> char {
    let mut rng = rand::thread_rng();
    match char_type {
        0 => char::from(rng.gen_range(33..=47)),
        1 => char::from(rng.gen_range(48..=57)),
        2 => char::from(rng.gen_range(58..=64)),
        3 => char::from(rng.gen_range(65..=90)),
        4 => char::from(rng.gen_range(91..=126)),
        _ => char::from(rng.gen_range(33..=126)),
    }
}