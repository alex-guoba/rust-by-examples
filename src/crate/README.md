# 定义

- **包（Packages）** ：Cargo 的一个功能，它允许你构建、测试和分享 crate。
- **Crates** ：一个模块的树形结构，它形成了库或二进制项目。
- **模块（Modules）和 use** ：允许你控制作用域和路径的私有性。
- **路径（path）**：一个命名例如结构体、函数或模块等项的方式

package、crate以及module的关系， [参考图](https://ask.qcloudimg.com/http-save/yehe-1177036/c7bc42df6e3f531a39fe176992f2357f.png)

- 1个Package里，至少要有1种Crate(要么是Library Crate，要么是Binary Crate)
- 1个Package里，最多只能有1个Library Crate，但是可以有0或多个Binary Crate
- 1个Crate里，可以创建0或多个mod

## crate 

crate 是 Rust 在编译时最小的代码单位。如果你用 rustc 而不是 cargo 来编译一个文件，编译器还是会将那个文件认作一个 crate。crate 可以包含模块，模块可以定义在其他文件，然后和 crate 一起编译。

crate 有两种形式：Binary 和 Library。

## modules

- **从 crate 根节点开始** : 当编译一个 **crate**, 编译器首先在 crate 根文件（通常，对于一个库 crate 而言是src/lib.rs，对于一个二进制 crate 而言是src/main.rs）中寻找需要被编译的代码。

- **声明模块** : 在 crate 根文件中，你可以声明一个新模块；比如，你用 `mod garden` 声明了一个叫做 `garden` 的模块。编译器会在下列路径中寻找模块代码：
    - 内联，在大括号中，当 `mod garden` 后方不是一个分号而是一个大括号
    - 在文件 `src/garden.rs`
    - 在文件 `src/garden/mod.rs`

- **声明子模块**: 在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在 `src/garden.rs` 中定义了 `mod vegetables`。编译器会在以父模块命名的目录中寻找子模块代码：
    - 内联，在大括号中，当 `mod vegetables` 后方不是一个分号而是一个大括号
    - 在文件 `src/garden/vegetables.rs`
    - 在文件 `src/garden/vegetables/mod.rs`

- **模块中的代码路径**: 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的Asparagus类型可以在crate::garden::vegetables::Asparagus被找到。
  
- **私有 vs 公用**: 一个模块里的代码**默认对其父模块私有**。为了使一个模块公用，应当在声明时使用pub mod替代mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub。
  - `pub` 也可以限定公开范围。即仅对指定的module放开访问权限。包括具体的module名、super、crate等。

- **use 关键字**: 在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用crate::garden::vegetables::Asparagus的作用域，你可以通过 `use crate::garden::vegetables::Asparagus;` 创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。

注意，整个模块树都植根于名为 crate 的**隐式模块**下。

## 路径
调用一个模块，需要指明他的路径。路径有两种形式：
- 绝对路径（absolute path）是以 crate 根（root）开头的全路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于当前 crate 的代码，则以字面值 crate 开头。
- 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。

绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。

使用 use 将两个同名类型引入同一作用域这个问题还有另一个解决办法：在这个类型的路径后面，我们使用 `as` 指定一个新的本地名称或者别名。

``` rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

### 嵌套路径来消除大量的 use 行

- 使用嵌套路径将相同的项在一行中引入作用域。

``` rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

- 可以在路径的任何层级使用嵌套路径，这在组合两个共享子路径的 use 语句时非常有用。

``` rust
use std::io::{self, Write};
```

### 重导出（re-exporting）

使用 use 关键字，将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的。如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，那我们可以将 pub 和 use 合起来使用。这种技术被称为 “重导出（re-exporting）”：我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域。

``` rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

