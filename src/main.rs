use text_io::read;

fn bubble_sort(mut numbers: [i32; 10]) -> [i32; 10]{
    for i in 0..9 {
        for j in 0..(9 - i) {
            if numbers[j] > numbers[j + 1] {
                let temp = numbers[j];
                numbers[j] = numbers[j + 1];
                numbers[j + 1] = temp;
            }
        }
    }
    return numbers;
}



fn main() {
    let mut numbers = [0; 10];
    for i in 0..10 {
        print!("Num: ");
        let num: i32 = read!();
        numbers[i] = num;
    }
    
    println!("{:?}", numbers);
    println!("Sorted Numbers: {:?}", bubble_sort(numbers));
}




#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_bubble_sort() {
        assert_eq!(bubble_sort([10,9,8,7,6,5,4,3,2,1]) ,[1,2,3,4,5,6,7,8,9,10]);
    }
}
  

