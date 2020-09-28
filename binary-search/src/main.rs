use standard_input;
use std::cmp::Ordering;

const VEC_LEN: i8 = 100;

fn main() {
    loop {
        let number = standard_input::read_number();
        let vec = create_vector(VEC_LEN);

        if let Some((position, steps)) = binary_search(number, vec) {
            println!("Target {} found at position {} in {} steps\n", number, position, steps);
        } else {
            println!("Target {} not found\n", number);
        }
    }
}

fn create_vector(len: i8) -> Vec<i32> {
    let vec: Vec<i32> = (0..len as i32).collect();
    return vec;
}

fn binary_search(target: i32, vec: Vec<i32>) -> Option<(usize, i8)> {
    let mut low = 0;
    let mut high = vec.len() - 1;
    let mut counter: i8 = 0;

    while low <= high {
        let mid = ((high - low) / 2) + low as usize;
        let guess = vec[mid];
        counter += 1;

        match guess.cmp(&target) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => return Some((mid, counter))
        }
    }
    None
}
