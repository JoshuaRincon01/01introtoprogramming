// Number Analyzer

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [4, 7, 15, 23, 30, 42, 55, 60, 81, 100];

    println!("Number analysis:");
    println!("----------------");

    let mut i = 0;
    while i < 10 {
        let n = numbers[i];

        if n % 3 == 0 && n % 5 == 0 {
            println!("{} → FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{} → Fizz", n);
        } else if n % 5 == 0 {
            println!("{} → Buzz", n);
        } else if is_even(n) {
            println!("{} → even", n);
        } else {
            println!("{} → odd", n);
        }

        i = i + 1;
    }

    // sum with while loop
    let mut sum = 0;
    i = 0;
    while i < 10 {
        sum = sum + numbers[i];
        i = i + 1;
    }
    println!("\nSum of all numbers: {}", sum);

    // largest number with loop
    let mut largest = numbers[0];
    i = 1;
    while i < 10 {
        if numbers[i] > largest {
            largest = numbers[i];
        }
        i = i + 1;
    }
    println!("Largest number: {}", largest);
}