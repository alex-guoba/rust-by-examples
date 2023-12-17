#![allow(dead_code)]

// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn test_drink() {
    // 几种赋值方式
    let water  = Some("water");
    let lemonade = Option::Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);

    // FIXME: 这里会导致 unwrap 抛出异常。
    drink(nothing);
}

use std::fs::File;
use std::io::{self, Read};

fn read_from_file(file_to_open :&str) -> Result<String, io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = File::open(file_to_open);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        // 将读取错误向上传播
        Err(e) => Err(e),
    }
}

// 使用 ? 来简化错误处理代码。
fn short_read_from_file(file_to_open :&str) -> Result<String, io::Error> {
    let mut f = File::open(file_to_open)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // test_drink();
    // let result = read_from_file("test1.txt");
    let result = short_read_from_file("test1.txt");
    match result {
        Ok(fp) => {
            println!("success, content {}", fp);
        },
        Err(e) => {
            // output and ignore
            println!("failed to open file, {}", e);
        }
    }
}