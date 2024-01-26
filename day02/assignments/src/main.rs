// Create three variables with the names: val1, val2, and ans. We want to perform a simple operation of generating the modulo of val1 and val2. Set val1 to 5 and val2 to 2. Assign the answer to the ans variable. Before executing your code, what do you think the answer will be?

fn main() {

    let val1 = 5;
    let val2 = 2;

    let ans = val1 % val2;

    println!("Ans is {}", ans);    
    second();
    concat_string(" World");
    control_flow(31);
}

//Create a vector and put in the values "2, 4, 6, 8, 10". Once you have created the vector perform the following: print out the current values, remove the value 10, add the value 12, and then print the vector back out to confirm your results.

fn second(){
    let mut odd = vec![2, 4, 6, 8, 10];
    odd.pop();
    odd.push(12);
    println!("{:?}",odd);
}

// Create a function called "concat_string". Create a string variable and assign the value "Hello" to it. The function is going to take one argument that is of type string and is going to return a String. Inside this function, concatenate the string " World". Print out the results in main() to confirm your results.

fn concat_string(w2:&str){
    let w1 = "Hello".to_string();

    println!("{}",w1+w2);
}

// Create a function called control_flow. This is going to take one argument that is an integer. Based on this integer, print out the following: "The value is one", "The value is greater than 50", "The value is less than 25", or "The value is greater than 25 but less than 50".

fn control_flow(a: i8){

    if a == 1{
        println!("The value is one");
    }else if a > 50 {
        println!("The value is greater than 50");
    }else if a < 25 {
        println!("The value is less than 25");
    }else {
        println!("The value is greater than 25 but less than 50");
    }

}