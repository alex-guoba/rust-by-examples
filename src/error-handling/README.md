# 错误处理

## panic

### 展开或者终止

当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。另一种选择是直接 终止（abort），这会不清理数据就退出程序。

两种设置方式：
- panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。例如，如果你想要在 release 模式中 panic 时直接终止：
``` toml
[profile.release]
panic = 'abort'
```

- 使用rustc编译时指定

``` shell
rustc  lemonade.rs -C panic=abort
```

可以设置 RUST_BACKTRACE 环境变量来得到一个 backtrace。backtrace 是一个执行到目前位置所有被调用的函数的列表。Rust 的 backtrace 跟其他语言中的一样：阅读 backtrace 的关键是从头开始读直到发现你编写的文件。这就是问题的发源地。

## 可恢复的错误

### option

An enum called Option<T> in the std library is used when absence is a possibility. It manifests itself as one of two "options":

- Some(T): An element of type T was found
- None: No element was found

Option 代表可能为空可能有值的一种类型，本质上是一个枚举，有两种分别是 Some 和 None。Some 代表有值，None 则类似于 null，代表无值。

- 为何要这么设计？而不是像其他语言一样直接判断null/nil之类即可？

因为处于安全考虑， rust希望编码和编译阶段就阻止可能的错误发生，而不是运行时才知道出错，Option 就能很好的做到这一点，不通过some判断是无法取到正确的结果的。比如：

``` rust
let val: Option<u32> = get_some_val();

//方法 match
match val {
    Some(num) => println!("val is: {}", num),
    None => println!("val is None")
}

//方法 if let
if let Some(num) = val {
    println!("val is: {}", num);
// } else {
//     println!("val is None");
// }
```

`Some()`能确保读取到正确的值后流程才能结束。避免了其他代码中为判断返回值有效性就继续的情况。

- 如何简化取值逻辑
直接使用unwrap来取代上面 `match` 那一段代码即可。

``` rust
let num: u32 = val.unwrap();
```
unwrap() 做的事就是有值返回值，如果值是 None 则直接 panic!。


### Result

跟Option类似，Result 内部本质是一个enum，内部分别是 Ok 和 Err，是 Ok 时则代表正常返回值，Err 则代表异常（Option是空或者非空）。

Result<T, E> 是一个枚举类型，定义如下：
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
泛型参数 T 代表成功时存入的正确值的类型，存放方式是 Ok(T)，E 代表错误时存入的错误值，存放方式是 Err(E)。

举例：
```rust
let sr: Result<u32, &str> = Ok(123); //or Err("You are wrong");

match sr {
    Ok(v) => println!("sr value: {}", v),
    Err(e) => println!("sr is error {}", e)
}
```

Result也支持unwrap，及错误时抛出异常。

```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```rust
expect 跟 unwrap 很像，也是遇到错误直接 panic, 但是会带上自定义的错误提示信息，相当于重载了错误打印的函数。

在实际的项目中通常要处理错误后继续，而非直接panic退出。所以通常需要将错误传递给上层的错误处理模块，用来记录、回滚等操作。典型的处理方式：

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_from_file(s :&str) -> Result<String, io::Error> {
    let f = File::open(s);

    let mut f = match f {
        Ok(file) => file,
        // 将打开错误向上传播
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        // 将读取错误向上传播
        Err(e) => Err(e),
    }
}
```

### ？

上面的代码中有两段类似的match处理逻辑，可以使用 `?` 来简化。
`?` 就是一个宏，跟上面的match处理逻辑基本一致，如果结果是 Ok(T)，则把 T 赋值给 f，如果结果是 Err(E)，则返回该错误。修改后的代码：
```rust
fn read_from_file(s :&str) -> Result<String, io::Error> {
    let mut f = File::open(s)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`?` 的另一大好处是自动类型转换。上面代码中 File::open 报错时返回的错误是 std::io::Error 类型，但是 open_file 函数返回的错误类型是 std::error::Error 的特征对象，可以看到一个错误类型通过 ? 返回后，变成了另一个错误类型，这就是 ? 的神奇之处。

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

