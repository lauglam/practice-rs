fn main() {
    let num = Number { begin: 0, end: 5 };
    // Rust 通过 for 语法糖，自动把实现了`IntoIterator`特征的数组类型转换为迭代器
    for i in num {
        println!("{i}");
    }

    // let num_iter = num.into_iter(); // `into_iter()`会消耗掉`num`

    let num_iter = NumberIterator { begin: 0, end: 5 };
    for i in num_iter {
        println!("{i}");
    }

    // 迭代器内的方法分为 消费者适配器 和 迭代器适配器
    //   1. 消费者适配器：消费掉迭代器，产生一个最终的结果。如：collect, sum（它们内部都调用了next方法）
    //   2. 迭代器适配器：返回一个新的迭代器，用于对原有的迭代器内的元素进行调整。如：map, filter（他们可以链式链接，最后都需要一个消费者适配器的到结果）
}

/// `Number` 本身不是一个迭代器
struct Number {
    begin: usize,
    end: usize,
}

/// `Number`实现了`IntoIterator`，可以生成一个迭代器
/// Rust 通过 for 语法糖，自动把实现了该特征的数组类型转换为迭代器
impl IntoIterator for Number {
    type Item = usize;
    type IntoIter = NumberIterator;

    fn into_iter(self) -> Self::IntoIter {
        NumberIterator { begin: self.begin, end: self.end }
    }
}

/// 属于`Number`的迭代器
struct NumberIterator {
    begin: usize,
    end: usize,
}

/// 迭代器都必须实现`Iterator`
impl Iterator for NumberIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin < self.end {
            let res = self.begin;
            self.begin += 1;
            Some(res)
        } else {
            None
        }
    }
}
