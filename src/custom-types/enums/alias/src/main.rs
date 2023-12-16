// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// 类型别名用来创建类型同义词，目的：
//  1. 为了减少输入(长名称改为短名称)
//  2. 有意义的别名可以更加方便对代码的理解
// 注意：Types must have UpperCamelCase names, or the compiler will raise a warning. 
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;


// impl的Self 也是alias的一种
enum VeryVerboseEnumOfNumsOp {
    Add,
    Subtract,
}
// 在一个 impl 块中，Self 类型是 impl 块的类型的别名。
// 注意：&self 实际上是 self: &Self 的缩写。
impl VeryVerboseEnumOfNumsOp {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let _x = Operations::Add;

    // type alias：Kilometers 类型的值将被完全当作 i32 类型值来对待
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // 即使混用 i32 与 Kilometers 编译器也不会报错，即可以认为二者是等价的。
    let s = VeryVerboseEnumOfNumsOp::Subtract;
    println!("substract(x,y) = {}", s.run(x, y))
}