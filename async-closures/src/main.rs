//! 异步闭包
//! 在 trait 中拥有静态异步 fn，它们将如何工作
//! 异步转换器如何以一种令人惊讶的自然方式“正常工作”

use std::future::Future;
use std::pin::Pin;
use std::process::Output;
use std::task::Poll::Ready;
use std::task::{Context, Poll};

async fn receive_message(s: &str) -> &str {
    todo!()
}

/// 同步组合器
fn call_twice_sync(mut op: impl FnMut(&str)) {
    op("Hello");
    op("Rustaceans");
}

/// 异步组合器
// fn call_twice_async<F>(mut op: impl FnMut(&str) -> F)
// where
//     F: Future<Output = ()>,
// {
//     op("Hello");
//     op("Rustaceans");
// }
async fn call_twice_async<F>(mut op: impl AsyncFnMut<&str>)
    where
        F: Future<Output = ()>,
{
    op.call("Hello").await;
    op.call("Rustaceans").await;
}


fn main() {
    let mut buffer = String::new();
    // call_twice_sync(|s| buffer.push_str(s));

    // `await` is only allowed inside `async` functions and blocks
    // call_twice_sync(|s| buffer.push_str(receive_message(s).await));

    // 闭包试图用异步块构建一个未来。这个异步块将捕获对它需要的所有变量的引用：在本例中为 s 和 buf
    // call_twice_async(|s| async { buffer.push_str(s) });

    // 这里的关键点是闭包返回一个结构 (MyAsyncBlockType) 并且这个结构持有对 buf 和 s 的引用，以便它可以在等待时使用它们
    // 签名说它需要一个 &str——这意味着闭包在执行时可以使用该字符串，但它不能保留对该字符串的引用并在以后使用它
    // buffer 也是如此，它可以通过闭包的隐式 self 参数访问
    // 但是当闭包返回 future 时，它试图创建对 buf 和 s 的引用，这些引用比闭包本身还长！
    // call_twice_async(|s| MyAsyncBlockType { buffer: &mut buffer, s });

    println!("{}", buffer);
}

struct MyAsyncBlockType<'b> {
    buffer: &'b mut String,
    /// 此处保存的s是闭包传入的参数，这迫使s保留的引用比闭包本身还长
    s: &'b str,
}

impl Future for MyAsyncBlockType<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.s;
        Ready(self.buffer.push_str(s))
    }
}

fn push_buf<'a>(buf: &'a mut String, s: &'a str) -> impl Future<Output = ()> + 'a {
    // 返回的这个async块的生命周期与两个参数相连，必须显式标注
    async move {
        buf.push_str(s);
    }
}

// async fn test() {
//     let c = for<'a> |buffer: &'a mut String, s: &'a str| -> impl Future<Output = ()> + 'a {
//         async move { buffer.push_str(s) }
//     };
//
//     let mut buffer = String::new();
// }

trait AsyncFnMut<A> {
    type Output;
    type Call<'a>: Future<Output = Self::Output> + 'a
    where
        Self: 'a;
    // async fn call(&mut self, args: A) -> Self::Output;

    fn call<'a>(&mut self, args: A) -> Self::Call<'_>;
}

// impl<A,F> AsyncFnMut<A> for F
// where
//     F :FnMut(A),
// {
//     type Output = ();
//     type Call<'a> where Self: 'a = ();
//
//     fn call<'a>(&mut self, args: A) -> Self::Call<'_> {
//         async move {
//             self(args);
//         }
//     }
// }
