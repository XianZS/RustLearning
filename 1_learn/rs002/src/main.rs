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
    println!("=== Main1 Function ===");
    // 默认情况下，变量是不可变的。
    // 体现
    let x = 5;
    println!("The value of x is {}.", x);
    // x = 6; // 此处代码会报错
    println!("The value of x is {}.", x);
}

fn main2() {
    println!("=== Main2 Function ===");
    let mut x = 5;
    println!("The value of x is {}.", x);
    x = 6;
    println!("The value of x is {}.", x);
}

fn main3() {
    /*
    首先，常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变。
    常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须注明。
    */
    println!("=== Main3 Function ===");
    const NUMBER: u32 = 1024 * 1024;
    println!("The value of NUMBER is {}.", NUMBER);
    // NUMBER += 1; // 此处会报错
    println!("The value of NUMBER is {}.", NUMBER);
}

fn main4() {
    /*
    对比main3和main4代码，可以看到const和let关键词修饰的变量的最本质区别：
    const 修饰的变量，是不允许“覆盖修改的”。
    let 修饰的变量，是允许“覆盖修改的”。
    ---
    遮蔽和将变量标记为 mut 的方式不同，
    因为除非我们再次使用 let 关键字，
    否则若是我们不小心尝试重新赋值给这个变量，
    我们将得到一个编译错误。
    通过使用 let，我们可以对一个值进行一些转换，
    但在这些转换完成后，变量将是不可变的。
    */
    println!("=== Main4 Function ===");
    let x = 1;
    println!("The value of x is {}.", x);
    let x = x + 1;
    println!("The value of x is {}.", x);
}

fn main5() {
    // mut let 和 let 之间的区别：
    // - mut let 可以直接进行修改，变量的类型是不可变的。
    // - let 只允许覆盖修改，变量的类型是可变的。
    println!("=== Main5 Function ===");
    // --- 演示 let 修饰的变量的类型是可变的，从 string ——> int
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is {}.", spaces);
    // --- 演示 let mut 修饰的变量的类型是不可变的。
    let mut spaces_1 = "   ";
    // spaces_1 = spaces_1.len();  // 此处会报错：expected `&str`, found `usize`
    println!("The length of spaces_1 is {}.", spaces_1);
}

fn main6() {
    // Rust 的每个值都有确切的数据类型（data type），
    // 该类型告诉 Rust 数据是被指定成哪类数据，
    // 从而让 Rust 知道如何使用该数据。
    // 在本节中，我们将介绍两种数据类型：标量类型和复合类型。
    // ---
    // 请记住 Rust 是一种静态类型（statically typed）的语言，
    // 这意味着它必须在编译期知道所有变量的类型。
    // 编译器通常可以根据值和使用方式推导出我们想要使用的类型。
    println!("=== Main6 Function ===");
}
fn main7() {
    println!("=== Main7 Function ===");
    // --- 标量类型
    // （1）整数类型
    //  长度        ：  8 位    16位    32位    64位    128位   arch
    //  有符号类型  ：  i8      i16     i32     i64     i128    isize
    //  无符号类型  ：  u8      u16     u32     u64     u128    usize
    // 示例：有符号8位可以存储的范围为（-128~127），无符号存储的范围为（0~255）。
    // 注意：arch
    // 表示程序运行的计算机体系结构，比如在64位系统中可表示为64位，在32位系统中可表示为32位。
}
