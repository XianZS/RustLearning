use rand::Rng;
use std::io;

fn main() {
    main1();
    main2();
    main3();
    main4();
    main5();
    main6();
    main7();
}

fn main1() {
    // 输出hello world
    println!("hello world!");
}

fn main2() {
    // 1.cargo build
    // 构建rust项目。
    // 2.cargo check
    // 检查整个项目，确保整个项目可以被编译，但是并不会产生任何的文件。
    // 3.cargo run
    // 一步构建项目，并且运行整个项目。
}

fn main3() {
    // 当项目最终准备好发布时，可以使用 cargo build --release 来优化编译项目。
    // 这会在 target/release 而不是 target/debug 下生成可执行文件。
    // 这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。
    // 这也就是为什么会有两种不同的配置：一种是为了开发，你需要经常快速重新构建；
    // 另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。
    // 如果你要对代码运行时间进行基准测试，请确保运行 cargo build --release 并使用 target/release 下的可执行文件进行测试。
}

fn main4() {
    println!("Guess the number!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read_line");
    println!("You guessed: {}", guess);
}

fn main5() {
    let number_1 = 5;
    let mut number_2 = 5;
    // number_1 += 1;   可以体现出未使用mut修饰的变量是不可变的
    number_2 += 1;
    println!("number_1: {}", number_1);
    println!("number_2: {}", number_2);
}

// fn main6() {
//     let mut number = String::new();
//     io::stdin().read_line(&mut number).expect("Error");
// }

fn main6() {
    // 随机数生成
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret_number is : {}", secret_number);
    println!("Please input your guess number:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    println!("Your guessed : {}", guess);
}

fn main7() {
    println!("The numbers : ");
    let mut index = 1;
    let mut input_number = String::new();
    loop {
        println!("[index] = {}", index);
        index += 1;
        io::stdin().read_line(&mut input_number).expect("Error");
        println!("input_number : {}", input_number);
    }
}
