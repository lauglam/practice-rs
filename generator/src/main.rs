use std::pin::Pin;

trait Generator {
    type Item;
    /// 采用固定引用
    fn next(self: Pin<&mut Self>) -> Option<Self::Item>;
}

trait IntoGenerator {
    type Item;
    type IntoGen: Generator<Item = Self::Item>;
    fn into_gen(self) -> Self::IntoGen;
}

impl<I: Iterator> Generator for I {
    type Item = <I as Iterator>::Item;

    fn next(self: Pin<&mut Self>) -> Option<Self::Item> {
        unsafe { Iterator::next(Pin::get_unchecked_mut(self)) }
    }
}

fn main() {
    println!("Hello, world!");
}
