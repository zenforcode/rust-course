fn main() {
    println!("Loop construct test!");
    let mut counter = 0;
    loop {
        counter +=1;
        println!("Counter {}", counter);
        if counter == 100 {
            break;
        }
    }
    println!("End of the first loop");
    // we want to transform the loop from an expression to a statement
    let mut cool_counter = 0;
    let result = loop  {
        if cool_counter > 5 {
            break cool_counter * 10;
        }
        cool_counter+=1;
    };
    println!("The cool counter multipled for 10 is {}", result)
}
