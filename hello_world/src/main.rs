//Rebekah K. Trevino
//3334-01

fn is_even (n: i32) -> bool{
    //checking if number is even if it is divisible by 2
    n % 2 == 0
}

fn main(){
    // an array of 10 integer nums 
    let numbers = [1,24,25,30,7,8,76,5,43,10];

    // to iterate through array
    for &num in &numbers{
        println!("Number:{}",num);

        //checking if even or odd
        if is_even(num){
            print!("EVEN:");
        }
        else{
            print!("ODD:");
        }
        //check if divisible by 3 and 5
        if num % 3 == 0{
            print!("FIZZ");
        }
        if num % 5 == 0{
            print!("BUZZ");
        }
        println!(); //new line 
    }
    //sum calculationusing while loop
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len(){
        sum += numbers[index];
    index += 1;
    } 
    println!("Sum of all numbers:{}", sum);

    //to find the largest num using a loop
    let mut largest = numbers[0];
    for &num in &numbers{
        if num > largest{
            largest = num;
        }
    }
        println!("Largest number:{}", largest);
}