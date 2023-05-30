//! newtype的作用：
//! 1. 自定义类型可以给出更有意义和可读性的类型名，如将`u32`作为`Meters`。但是使用类型别名可能会更好
//! 2. 为外部类型实现外部特征（孤儿规则）
//! 3. 隐藏内部实现细节

/// 为外部类型实现外部特征（孤儿规则）
struct Wrapper(Vec<String>);

/// 要为类型 A 实现特征 T，那么 A 或者 T 必须至少有一个在当前的作用范围内
/// 但可以使用newtype进行实现
impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}


fn main() {
    println!("Hello, world!");
}
