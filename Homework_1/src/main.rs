
fn A_1(){

    const Freezing_Point_F: f64 = 35.0;

    fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
    }

    fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
    }

    let mut temp_F: f64 = Freezing_Point_F;
    println!("The temperature is {}°F", temp_F);
    println!("In Celsius: {:.2}°C", fahrenheit_to_celsius(temp_F));
    println!("Next temperatures:");

    for _ in 0..5{
        temp_F += 1.0;
        let temp_C = fahrenheit_to_celsius(temp_F);
        println!("{temp_F}°F = {:.2}°C", temp_C);
    }

}


fn A_2(){
    
    fn is_even(n: i32) -> bool{
    n % 2 == 0
    }

    let nums = [5, 2, 8, 4, 11, 12, 7, 3, 9, 10];

    for n in nums {
        if n % 3 == 0 && n % 5 == 0{
            println!("{n}: FizzBuzz");
        }   
        else if n % 3 == 0{
            println!("{n}: Fizz");
        }   
        else if n % 5 == 0{
            println!("{n}: Buzz");
        } 
        else{
            if is_even(n){
                println!("{n}: EVEN");
            } 
            else{
                println!("{n}: ODD");
            }
    }}

    let mut index = 0;
    let mut sum = 0;

    while index < nums.len(){
        sum += nums[index];
        index += 1;
    }

    println!("The SUM of all numbers is: {sum}");

    let mut largest = nums[0];
    let mut i = 1;
    
    loop{
        if i >= nums.len(){
            break;
        }

        if nums[i] > largest{
            largest = nums[i];
        }

        i += 1;
    }
    println!("The LARGEST number is: {largest}");
}


fn A_3(){
    
    fn check_guess(guess:i32, secret: i32) -> i32{
    if guess == secret{
        0
    }
    else if guess > secret{
        1
    }
    else{
        -1
    }
}

    let secret:i32 = 9;
    let mut attempts:i32 =0;
    let mut guess:i32 = 1;

    loop{
        attempts +=1;

        let result = check_guess(guess, secret);

        if result == 0{
            println!("Guess {guess}: Correct!");
            break;
        } 
        else if result == 1{
            println!("Guess {guess}: Too high!")
        } 
        else{
            println!("Guess {guess}: Too low");
        }
        guess += 1;
    }
    println!("It took {attempts} guesses to find the secret number.");
}

fn main(){
    println!(" ");
    A_1();
    println!(" ");
    A_2();
    println!(" ");
    A_3();
    println!(" ");
}