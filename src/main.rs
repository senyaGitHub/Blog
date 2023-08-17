use text_io::read;

fn main() {
    let mut numbers = [0; 10];
    for i in 0..10 {
        print!("Num: ");
        let num: i32 = read!();
        numbers[i] = num;
    }

    // Bubble sort
    for i in 0..9 {
        for j in 0..(9 - i) {
            if numbers[j] > numbers[j + 1] {
                let temp = numbers[j];
                numbers[j] = numbers[j + 1];
                numbers[j + 1] = temp;
            }
        }
    }
    println!("{:?}", numbers);
    println!("Sorted Numbers: {:?}", numbers);
}

  

