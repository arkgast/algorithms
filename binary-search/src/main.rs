use standard_input;

const VEC_LEN: i32 = 100;

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

fn create_vector(len: i32) -> Vec<i32> {
    let vec: Vec<i32> = (0..len).collect();
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

        if guess == target {
            return Some((mid, counter));
        }

        if guess < target {
            low = mid + 1;
        }

        if guess > target {
            high = mid - 1;
        }
    }
    None
}
