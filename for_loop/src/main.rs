fn main() {
    let message = ['H','e','l','l','o'];
    for item in message {
        println!("Char : {}", item);
    }
    println!("Let's print also the index");
    for (index, item) in message.iter().enumerate() {
        println!("{} {}", index, item);
    }
    println!("I wanna print until i don't find l");
    for (index, &item) in message.iter().enumerate() {
        if item == 'l' {
            break;
        }
        println!("{} {}", index, item);
    }
    println!("i want to print the even number till 20");
    for n in 1..20 {
        if n % 2 == 0 {
            println!("{}", n);
        }
    }

}
