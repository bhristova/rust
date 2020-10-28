fn base_fizzbuzz(n: usize, k1: u8, k2: u8, labels: &[String; 3]) -> Vec<String> {
    if k1 == 0 || k2 == 0 {
        panic!("You can't devide by zero!")
    }

    if k1 == 1 || k2 == 1 {
        panic!("What is the point of dividing by one?!")
    }
    let k1Usize = k1 as usize;
    let k2Usize = k2 as usize;
    let mut result = Vec::<String>::new();
    for x in 1..n + 1 {
        match x as usize {
            x if x % k1Usize == 0 && x % k2Usize == 0 => result.push(String::from(&labels[2])),
            x if x % k1Usize == 0 => result.push(String::from(&labels[0])),
            x if x % k2Usize == 0 => result.push(String::from(&labels[1])),
            _ => result.push(x.to_string())
        }
    }
    result
}

pub fn fizzbuzz(n: usize) -> Vec<String> {
    let labels = ["Fizz".to_string(), "Buzz".to_string(), "Fizzbuzz".to_string()];
    base_fizzbuzz(n, 3, 5, &labels)
}

pub fn custom_buzz(n: usize, k1: u8, k2: u8) -> Vec<String> {
    let labels = ["Fizz".to_string(), "Buzz".to_string(), "Fizzbuzz".to_string()];
    base_fizzbuzz(n, k1, k2, &labels)
}

pub struct FizzBuzzer {
    pub k1: u8,
    pub k2: u8,
    pub labels: [String; 3],
}

impl FizzBuzzer {
    pub fn take(&self, n: usize) -> Vec<String> {
        base_fizzbuzz(n, self.k1, self.k2, &self.labels)
    }
    
    pub fn change_label(&mut self, index: usize, value: &String) {
        self.labels[index] = value.to_string();
    }
    
    pub fn print(&self) {
        print!("{:?}", self.labels);
    }
}
