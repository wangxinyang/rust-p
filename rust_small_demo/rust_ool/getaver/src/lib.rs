#[derive(Debug)]
pub struct AverCollect {
    list: Vec<i32>,
    average: f64,
}

impl AverCollect {
    pub fn new() -> AverCollect {
        AverCollect {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn add(&mut self, num: i32) {
        self.list.push(num);
        self.update_average()
    }

    pub fn delete(&mut self) {
        self.list.pop();
        self.update_average()
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
