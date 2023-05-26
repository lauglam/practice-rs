//！ https://course.rs/advance/circle-self-ref/self-referential.html

// struct SelfRef<'a> {
//     value: String,
//
//     // 该引用指向上面的value
//     pointer_to_value: &'a str,
// }

// fn main() {
//     let s = "aaa".to_string();
//     let v = SelfRef{
//         value: s,
//         pointer_to_value:   &s
//     };
// }

// =================================================================================================

// #[derive(Debug)]
// struct WhatAboutThis<'a> {
//     name: String,
//     nickname: Option<&'a str>,
// }
//
// /// 下面这段代码无法使用在方法中作为返回值返回，因为返回时所有权的转移可能会涉及值在内存中的移动
// /// 此处的 'a 是凭空出现的，并没有一个参数是'a
// fn creator<'a>() -> WhatAboutThis<'a> {
//     let mut tricky = WhatAboutThis { name: "Annablelle".to_string(), nickname: None };
//     tricky.nickname = Some(&tricky.name[..4]); // tricky.name在此处被借用，所以转移所有权可能会造成移动
//
//     tricky // 返回可能会造成值的移动
// }
//
// fn main() {
//     // 下面这段代码可以使用，但无法作为方法返回，具体看creator方法
//     let mut tricky = WhatAboutThis { name: "Annablelle".to_string(), nickname: None };
//     tricky.nickname = Some(&tricky.name[..4]);
//     println!("{tricky:?}");
//     // 虽然上述的代码可以通过，但下面的代码会出错，原因是tricky的所有权被转移，而转移所有权有可能造成移动
//     let s = tricky;
// }

// =================================================================================================

// /// unsafe实现
// #[derive(Debug)]
// struct SelfRef {
//     value: String,
//     // 接存储裸指针，而不是 Rust 的引用，
//     // 因此不再受到 Rust 借用规则和生命周期的限制，而且实现起来非常清晰、简洁。
//     // 但是缺点就是，通过指针获取值时需要使用 unsafe 代码。
//     pointer_to_value: *const String,
// }
//
// impl SelfRef {
//     fn new(txt: &str) -> Self {
//         Self { value: String::from(txt), pointer_to_value: std::ptr::null() }
//     }
//
//     fn init(&mut self) {
//         let self_ref: *const String = &self.value;
//         self.pointer_to_value = self_ref;
//     }
//
//     fn value(&self) -> &str {
//         &self.value
//     }
//
//     fn pointer_to_value(&self) -> &String {
//         assert!(
//             !self.pointer_to_value.is_null(),
//             "Test::b called without Test::init being called first"
//         );
//         unsafe { &*(self.pointer_to_value) }
//     }
// }

// fn main() {
//     let mut t = SelfRef::new("hello");
//     t.init();
//     // 打印值和指针地址
//     println!("{}, {:p}", t.value(), t.pointer_to_value());
// }

// =================================================================================================

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

/// 无法移动的Pin
/// 下面是一个自引用数据结构体，因为 slice 字段是一个指针，指向了 data 字段
/// 我们无法使用普通引用来实现，因为违背了 Rust 的编译规则
/// 因此，这里我们使用了一个裸指针，通过 NonNull 来确保它不会为 null
struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        // 为了确保函数返回时数据的所有权不会被转移，我们将它放在堆上，唯一的访问方式就是通过指针
        let res = Unmovable {
            data,
            // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        // 这里其实安全的，因为修改一个字段不会转移整个结构体的所有权
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

fn main() {
    let unmoved = Unmovable::new(String::from("hello"));
    // 只要结构体没有被转移，那指针就应该指向正确的位置，而我们可以随意移动指针
    let still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

    // 因为我们的类型没有实现 `Unpin` 特征，下面这段代码将无法编译
    // let new_unmoved = Unmovable::new(String::from("world"));
    // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
}
