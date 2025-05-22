use std::io;

//Todo complete tasks
fn main() {
    println!("---TODO APP ---");
    println!("Añade tu tarea");
    let input_user = input();

    let mut tasks : Vec<String> = Vec::new();

    tasks.push(input_user);


    println!("Tu tarea es: {}", tasks[0]);
}

fn input() -> String{
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read line");
    input_user
}
