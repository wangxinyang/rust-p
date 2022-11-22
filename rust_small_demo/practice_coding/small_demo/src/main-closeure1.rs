struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculate: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
// 返回闭包
where
    T: Fn(u32) -> u32,
{
    fn new(calculate: T) -> Self {
        Self {
            calculate,
            value: None,
        }
    }

    fn value(&self, arg: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                let v = (self.calculate)(arg);
                v
            }
        }
    }
}

fn main() {
    let cacher = Cacher::new(|x| x + 1);
    println!("{}", cacher.value(1));
}
