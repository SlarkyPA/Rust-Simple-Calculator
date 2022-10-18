fn main() {

    let mut choice = String::new();

    loop{
        calculator();
        println!("Please enter the choice(Y to continue and N to exit)");
        std::io::stdin().read_line(&mut choice).expect("Please enter the choice(Y to continue and N to exit)");
        if choice.trim()=="N" {
            break;
        }
    }
    
}

fn calculator(){
    let mut number1 = String::new();
    let mut number2 = String::new();
    let mut operator = String::new();
    
    println!("Please enter the first number");
    std::io::stdin().read_line(&mut number1).expect("Failed to read first number.");

    println!("Please enter the second number");
    std::io::stdin().read_line(&mut number2).expect("Failed to read second number.");

    println!("Please enter the operator(+ for addition, - for subtraction, * for multiplication, / for division)");
    std::io::stdin().read_line(&mut operator).expect("Failed to read operator.");


    let num1: f32 = number1.trim().parse().expect("Conversion Failed, Make sure number 1 is correct.");
    let num2: f32 = number2.trim().parse().expect("Conversion Failed, Make sure number 2 is correct.");

    if operator.trim()=="+"{
        println!("The sum of {} and {} is {}", num1,num2,num1+num2);
    }
    else if operator.trim()=="-"{
        println!("The subtraction of {} and {} is {}", num1,num2,num1-num2);
    }
    else if operator.trim()=="*"{
        println!("The multiplication of {} and {} is {}", num1,num2,num1*num2);
    }
    else if operator.trim()=="/"{
        println!("The devision of {} and {} is {}", num1,num2,num1/num2);
    }
    else {
        println!("Invalid Operator,Please enter the operator(+ for addition, - for subtraction, * for multiplication, / for division)");
    }
}



//we need to read two numbers from the user.
//we need to read the operation
//we need to perform the operation depending upon the choice of operation by user.
//display the result
