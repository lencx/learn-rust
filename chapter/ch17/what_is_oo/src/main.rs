fn main() {
    let mut a = AveragedCollection {
        list: vec![],
        average: 0.0,
    };
    a.add(10);
    a.add(20);
    println!("a average: {:?}", a); // 15.0
    a.add(30);
    println!("a average: {:?}", a); // 20.0
    a.remove();
    println!("a average: {:?}", a); // 15.0

}

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(val) => {
                self.update_average();
                Some(val)
            },
            None => None,
        }
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
