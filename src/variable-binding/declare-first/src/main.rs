

// * 变量必须初始化后才能使用，没有默认值一说。。
fn declare_first() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    // a binding: 4
    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}


// * Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// * 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// * 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
fn freezing() {
    let mut _mutable_integer = 7i32;
    let mut _s = String::from("hello");  // s 进入作用域

    {
        // 任何基本类型(的组合)可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的.
        //  但是String等非基本类型不会copy，所以这里转移了所有权（move）。转移后不允许再次使用原资源
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;
        let _mutable_str = _s;

        // i32 读：允许
        println!("frozen but can be read {}", _mutable_integer);
        // i32 写：不允许
        // _mutable_integer = 50;

        // String： 读写均不允许
        // 转移所有权后访问会报错： ^ value borrowed here after move
        // println!("can‘t be read {}", _s);

    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    _s = String::from("world");
}

fn main() {
    declare_first();

    freezing();
}