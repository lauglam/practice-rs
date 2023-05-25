use std::time::Duration;
use tokio::runtime::Runtime;

fn main() {
    println!("Hello, world!");
}

trait Sequencer {
    fn generate(&self) -> Vec<i32>;
}

struct PlainSequencer {
    bound: i32,
}

impl PlainSequencer {
    async fn generate_async(&self) -> Vec<i32> {
        let mut res = vec![];

        for i in 0..self.bound {
            res.push(i);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        res
    }
}

impl Sequencer for PlainSequencer {
    fn generate(&self) -> Vec<i32> {
        let rt = Runtime::new().unwrap();
        // rt.block_on(async { self.generate_async().await })

        let bound = self.bound;
        rt.block_on(async move {
            let rt = Runtime::new().unwrap();
            rt.spawn(async move {
                let mut res = vec![];
                for i in 0..bound {
                    res.push(i);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                res
            })
            .await
            .unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sync_method() {
        let sequencer = PlainSequencer { bound: 3 };
        let vec = sequencer.generate();
        print!("vec: {:?}", vec);
    }
}
