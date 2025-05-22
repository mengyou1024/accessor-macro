## 介绍

accessor-macro 是一个 Rust 的派生宏（derive macro）库，用于自动生成结构体字段的 getter 和 setter 方法
该宏支持为字段生成简单的访问器方法，同时也支持在 setter 中添加范围检查逻辑，以确保设置的值在指定范围内

### 核心功能

#### 1. 自动生成 Getter 和 Setter

使用 #[derive(Accessor)] 宏来自动生成字段的 getter 和 setter
字段上使用 #[accessor(get, set)] 属性来标记需要生成 getter 和 setter 的字段


#### 2. 范围检查

在 setter 方法中可以添加范围检查逻辑，当设置的值超出指定范围时，setter 返回 false
使用 range=[min, max] 来定义字段的取值范围

## 使用方法

### 1.  定义结构体并添加 `#[derive(Accessor)]`

在结构体定义前添加 #[derive(Accessor)] ，并在需要生成访问器和修改器的字段上添加 #[accessor] 属性。

```rust
use accessor_macro::Accessor;
use uintx::u24;

#[derive(Accessor, Debug)]
struct Person {
    //  生成getter和setter
    #[accessor(get, set)]
    name: String,
    //  生成getter和setter，同时添加范围检查
    #[accessor(get, set, range=[0, 200])]
    age: i32,
    //  生成getter和setter，同时添加范围检查, 范围使用表达式生成
    #[accessor(get, set, range=[u24::from(0), u24::from(100)])]
    number: u24,
}
```

### 2.  属性说明

- get : 为字段生成 getter 方法。
- set : 为字段生成 setter 方法。
- range=[min, max] : 在 setter 方法中添加范围检查，当设置的值超出范围时，返回 false 。

### 3.  示例代码

```rust
fn main() {
    let mut person = Person {
        name: "Alice".to_string(),
        age: 25,
        number: u24::from(50),
    };
    println!("person: {:?}", person);

    // 尝试设置超出范围的值
    println!("set age to -1:");
    assert!(!person.set_age(-1));
    println!("person: {:?}", person);

    // 设置合法的值
    println!("set age to 18:");
    assert!(person.set_age(18));
    println!("person: {:?}", person);
}
```

## 运行示例

你可以使用以下命令运行示例代码：

```rust
cargo run --example basic_usage
```

## 小声BB

这个项目是AI生成的，如果您有任何问题请咨询AI

如果您不幸联系了我，那么不好意思，我并没有能力做任何修改😊

当然如果能给该项目提交一个PR那就再好不过了👍

![OIP](./assets/rust.png)
