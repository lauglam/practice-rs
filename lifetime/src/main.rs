//！ https://course.rs/advance/lifetime/advance.html
//! 生命周期消除：
//! 1. 每个引用参数都会获得独自的生命周期
//! 2. 若只有一个输入生命周期（函数参数只有一个引用类型），那么该生命周期会被赋给所有的输出生命周期
//! 3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self
//! 的生命周期被赋给所有的输出生命周期

// =================================================例子1============================================

// #[derive(Debug)]
// struct Foo;
//
// impl Foo {
//     /// 将可变引用`&mut self`转为不可变引用`&self`
//     fn mutate_and_share(& mut self) -> &Self {
//         &*self
//     }
//
//     fn share(&self) {}
// }
//
// fn main() {
//     let mut foo = Foo;
//     // 此处虽然输入的是可变引用，但其实用到的是不可变引用
//     // 理论上应该可以通过编译（因为可以存在多个不可变引用）
//     let loan = foo.mutate_and_share();
//
//     // 编译器的提示在这里其实有些难以理解
//     // 因为可变借用仅在 mutate_and_share 方法内部有效，出了该方法后，就只有返回的不可变借用
//     // 因此，按理来说可变借用不应该在 main 的作用范围内存在
//     foo.share();
//
//     println!("{loan:?}");
// }

// =================================================例子2============================================

// #[allow(unused)]
// fn main() {
//     use std::{collections::HashMap, hash::Hash};
//
//     fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
//     where
//         K: Clone + Eq + Hash,
//         V: Default,
//     {
//         match map.get_mut(&key) {
//             // 存在则返回
//             Some(value) => value,
//             // 不存在，插入后返回
//             None => {
//                 // 分析代码可知在 match map.get_mut(&key) 方法调用完成后，对 map 的可变借用就可以结束了。
//                 // 但从报错看来，编译器不太聪明，它认为该借用会持续到整个 match 语句块的结束(第 49 行处)，这便造成了后续借用的失败。
//                 map.insert(key.clone(), V::default());
//                 map.get_mut(&key).unwrap()
//             }
//         }
//     }
// }

// ==============================================无界生命周期=========================================

// /// 无界生命周期
// /// 不安全代码(unsafe)经常会凭空产生引用或生命周期，这些生命周期被称为是 无界(unbound) 的
// /// 无界生命周期往往是在解引用一个裸指针(裸指针 raw
// pointer)时产生的，换句话说，它是凭空产生的，因为输入参数根本就没有这个生命周期 #[allow(unused)]
// fn main() {
//     // 参数 x 是一个裸指针，它并没有任何生命周期，然后通过 unsafe
// 操作后，它被进行了解引用，变成了一个 Rust 的标准引用类型，该类型必须要有生命周期，也就是 'a
// // 可以看出 'a 是凭空产生的，因此它是无界生命周期     // 这种生命周期由于没有受到任何约束，
// 因此它想要多大就多大，这实际上比 'static 要强大（因为它可大可小）     // 我们在实际应用中，
// 要尽量避免这种无界生命周期。=     //
// 最简单的避免无界生命周期的方式就是在函数声明中运用生命周期消除规则。 若一个输出生命周期被消除了，
// 那么必定因为有一个输入生命周期与之对应     fn f<'a, T>(x: *const T) -> &'a T {         //
// 只有unsafe才可以这样做，因为在safe中无法返回一个对当前函数拥有的数据的引用         unsafe { &*x }
//     }
// }

// ===========================================生命周期约束
// HRTB=======================================

// #![allow(unused)]
//
// /// 生命周期约束 HRTB
// /// 生命周期约束跟特征约束类似，都是通过形如 'a: 'b 的语法，来说明两个生命周期的长短关系
// ///
// /// 'b 必须活得比 'a 久（至少一样久），也就是结构体中的 s 字段引用的值必须要比 r
// 字段引用的值活得要久 struct DoubleRef<'a, 'b: 'a, T> {
//     r: &'a T,
//     s: &'b T,
// }
//
// /// 表示类型 T 必须比 'a 活得要久(被引用者的生命周期必须要比引用长)
// /// 在 Rust 1.30 版本之前，该写法是必须的
// // struct Ref<'a, T: 'a> {
// //     r: &'a T,
// // }
//
// /// 但是从 1.31 版本开始，编译器可以自动推导 T: 'a 类型的约束，因此我们只需这样写即可：
// struct Ref<'a, T> {
//     r: &'a T,
// }
//
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// impl<'a, 'b: 'a> ImportantExcerpt<'b> {
//     /// 只有在 'b >= 'a 的情况下，'b 才能转换成 'a
//     fn announce_and_return_part(&'b self, announcement: &'a str) -> &'a str {
//         println!("Attention please: {announcement}");
//         self.part
//     }
// }
//
// fn main() {
//     let (a, b) = (5, 6);
//     let dou = DoubleRef { r: &a, s: &b };
// }

// ==========================================闭包函数的消除规则========================================

// #![allow(unused)]
//
// /// 闭包函数的消除规则
// fn main() {
//     fn fn_elision(x: &i32) -> &i32 {
//         x
//     }
//
//     // 错误原因是编译器无法推测返回的引用和传入的引用谁活得更久
//     // 对于函数的生命周期而言，
//     // 它的消除规则之所以能生效是因为它的生命周期完全体现在签名的引用类型上，在函数体中无需任何体现
//     // 可是闭包，并没有函数那么简单，
//     // 它的生命周期分散在参数和闭包函数体中(主要是它没有确切的返回值签名)
//     let closure_slision = |x: &'a i32| -> &'a i32 { x };
// }

// ========================================NLL (Non-Lexical Lifetime)===============================

// /// NLL (Non-Lexical Lifetime)
// /// 引用的生命周期正常来说应该从借用开始一直持续到作用域结束
// /// 但是这种规则会让多引用共存的情况变得更复杂
// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s;
//     let r2 = &s;
//     println!("{r1} and {r2}");
//     // 新编译器中，r1,r2作用域在这里结束
//
//     // 按照上述规则，这段代码将会报错，因为 r1 和 r2 的不可变引用将持续到 main 函数结束
//     // 而在此范围内，我们又借用了 r3的可变引用，这违反了借用的规则：要么多个不可变借用，要么一个可变借用
//     // 但该规则从 1.31版本引入 NLL 后，就变成了：引用的生命周期从借用处开始，一直持续到最后一次使用的地方。
//     let r3 = &mut s;
//     println!("{r3}");
// }

// ==============================================Reborrow 再借用=====================================

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl Point {
//     fn move_to(&mut self, x: i32, y: i32) {
//         self.x = x;
//         self.y = y;
//     }
// }
//
// fn main() {
//     let mut p = Point { x: 0, y: 0 };
//     let r = &mut p;
//     // rr 是对 r 的再借用
//     let rr: &Point = &*r;
//
//     // 但不能这样
//     // let rr1: &Point = &p;
//
//     println!("{rr:?}");
//     r.move_to(10, 10);
//     println!("{r:?}");
// }

// ==============================================&'static and T:'static=====================================

#![allow(unused)]
use std::fmt::Debug;

/// T:'static这种语法属于特征约束
/// 对于类型T的约束T:'static意味着T不包含任何非'static引用；对于接收方来说可以安全地持有T直到自己将其drop
fn main() {
    fn print_it<T: Debug + 'static>(input: T) {
        println!("符合 T:'static is: {input:?}");
    }

    // 显式标注生命周期
    fn print_it_1<'a, T: Debug + 'static>(input: &'a T) {
        println!("符合 T:'static is: {input:?}");
    }

    let s1 = "你好".to_string();
    // &String符合'a约束，Ok
    // String不包含非'static引用，Ok
    print_it_1(&s1);
    // s1的所有权转移到print_it，print_it可以安全的持有它直至函数结束，这种非引用的转移默认是满足'static约束的
    print_it(s1);

    #[derive(Debug)]
    struct Bad<'a>(&'a String);
    let s1 = String::from("1");
    let s2 = Bad(&s1);
    // print_it_1(&s2); // Bad
    // print_it(s2);    // Bad
}
