// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // 初始化和打印
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);


    // 结构体更新语法（struct update syntax）
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);


    // 解构结构体
    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;
    println!("destructure point {:?} and {:?}", left_edge, top_edge);


    // 注意：变量以"_"开头，未使用时不会报“unused variable: `rectangle`”错
    // 嵌套结构体初始化
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };


    // 类单元结构体(unit-like struct)常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
    // Instantiate a unit struct
    let _unit = Unit;


    // 元组结构体（tuple structs）有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
    // 当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);


    // 结构元祖结构体
    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

