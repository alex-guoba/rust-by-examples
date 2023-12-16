/*
Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:
    * const: An unchangeable value (the common case).
    * static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. 
        Accessing or modifying a mutable static variable is unsafe.


static的一些设计要点：
    * `static`关键字用于声明全局变量，这些变量默认没有特定的内存地址，并且可以被内联（inline）。
    * 使用`#[inline(never)]`属性可以给全局变量分配一个特定的地址。
    * `static mut`用于声明可变的静态变量，但访问这些变量通常被视为不安全的，因为它们可以被任何线程修改。—— 所以可变的全局变量在rust中是不推荐的，应使用Sync trait
    * 为了将静态值放置在只读内存中，它们的类型不能包含任何内部可变数据（例如`UnsafeCell`）。

More detail: https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md
*/


// Globals are declared outside all other scopes.
static /* mut */ LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}


fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;

    // error[E0133]: use of mutable static is unsafe and requires unsafe function or block
    // LANGUAGE = "Cpp"
}