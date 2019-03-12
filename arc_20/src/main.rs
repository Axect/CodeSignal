fn main() {
    println!("{}", arrayMaximalAdjacentDifference(vec![-1, 1, -3, -4]));
}

fn arrayMaximalAdjacentDifference(inputArray: Vec<i32>) -> i32 {
    let mut iter_arr = inputArray.into_iter();
    let mut diff = 0i32;
    let mut stacked_val = iter_arr.next().unwrap();

    for val in iter_arr {
        let new_diff = (val - stacked_val).abs();
        if diff < new_diff {
            diff = new_diff;
        }
        stacked_val = val;
    }
    diff
}