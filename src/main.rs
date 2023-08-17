use std::io;
use rand::{Rng};

fn bubble_sort(mut numbers: Vec<i32>) -> Vec<i32>{
    for i in 0..numbers.len() {
        for j in 0..numbers.len()-i-1 {
            if numbers[j] > numbers[j + 1] {
                let temp = numbers[j];
                numbers[j] = numbers[j + 1];
                numbers[j + 1] = temp;
            }
        }
    }
    numbers
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("Enter the length of the array:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input);
    
    let length: usize = input.trim().parse().expect("Invalid input.");

    let mut numbers: Vec<i32> = Vec::with_capacity(length);

    for _ in 0..length {
        numbers.push(rng.gen_range(0..10));
    }

    println!("Unsorted array: {:?}", numbers);
    let sorted_numbers = bubble_sort(numbers);
    println!("Sorted array: {:?}", sorted_numbers);
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_bubble_sort() {
        assert_eq!(bubble_sort(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(bubble_sort(vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 0]), vec![0, 1, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(bubble_sort(vec![5, 2, 7, 4, 1, 3]), vec![1, 2, 3, 4, 5, 7]);
        assert_eq!(bubble_sort(vec![1]), vec![1]);
        assert_eq!(bubble_sort(vec![3, 5, 8, 2, 1, 6]), vec![1, 2, 3, 5, 6, 8]);
}
}
  

