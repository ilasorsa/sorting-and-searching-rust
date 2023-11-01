use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}

fn main() {
    let num_items = get_i32("num_items: ");
    let max = get_i32("max: ");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, num_items);
    bubble_sort(&mut vec);
    print_vec(&vec, num_items);
    check_sorted(&vec);
}

fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    return vec;
}

fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>()
        .expect("Error parsing integer");
}

fn bubble_sort(vec: &mut Vec<i32>) {
    for i in 0..(vec.len() - 2) {
        for j in 0..(vec.len() - i - 1) {
            if vec[j] > vec[j + 1] {
                let tmp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = tmp;
            }
        }
    }
}

fn check_sorted(vec: &Vec<i32>) {
    for (i, item) in vec.iter().enumerate() {
        if i == vec.len() - 1 {
            break;
        }
        if item > &vec[i + 1] {
            println!("The vector is NOT sorted!");
            return;
        }
    }
    println!("The vector is sorted!");
}
