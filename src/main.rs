use std::io;

fn main() {
    // Read in the user's array
    let mut input = String::new();
    println!("Enter a space-separated list of numbers:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    // Read in the user's target number
    let mut input = String::new();
    println!("Enter the target number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: i32 = input.trim().parse().unwrap();

    // Perform binary search on the array
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            println!("Found the target number at index {}!", mid);
            return;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    println!("The target number was not found in the array.");
}
