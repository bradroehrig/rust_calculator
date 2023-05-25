fn main() {
    
    let mut number1: String = String::new();
    let mut number2: String = String::new();
    let mut operator: String = String::new();

    println!("Please enter the first number");
    std::io::stdin()
        .read_line(&mut number1 )
        .expect("Failed to read first number.");

    println!("Please enter the second number");
    std::io::stdin()
        .read_line(&mut number2 )
        .expect("Failed to read second number.");

    println!("Please enter the operator for addition, subtraction, multiplication, division, or exponents.");
    
    std::io::stdin()
        .read_line(&mut operator)
        .expect("failed to read operator input.");

    let num1 :u32 = number1
        .trim()
        .parse()
        .expect("Failed to convert number 1, make sure it is a valid number.");

    let num2 :u32 = number2
        .trim()
        .parse()
        .expect("failed to convert number 2, make sure it is a valid number.");

    if operator.trim()=="+" {
        println!("The sum of {} and {} is {}.", num1, num2, num1+num2);
    }

    else if operator.trim()=="-"{
        println!("The subtraction of {} and {} is {}.", num1, num2, num1-num2);
    }

    else if operator.trim()=="*"{
        println!("The multiplication of {} and {} is {}.", num1, num2, num1*num2);
    }
    else if operator.trim()=="/"{
        println!("The division of {} and {} is {}.", num1, num2, num1/num2);
    }
    else{
        println!("You input something incorrectly!")
    }

}

//We need to read two numbers from the user.
//We need to read the operation - addition, subtraction, multiplication or division, exponents.
//We need to perform that operation depending upon the choice of operation by the user.
//Display the results