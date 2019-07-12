struct MinStack1 {
    data: Vec<i64>,
    min: i64
}

impl MinStack1 {
    fn new() -> Self {
        MinStack1 {
            data: vec![],
            min: 0
        }        
    }

    fn push(&mut self, x: i32) {
        let x = x as i64;
        if self.data.len() == 0 {
            self.data.push(0);
            self.min = x;
        } else {
            self.data.push(x-self.min);
            if x < self.min { 
                self.min = x;
            }
        }
    }

    fn pop(&mut self) {
        if self.data.len() > 0 {
            if let Some(x) = self.data.pop() {
                if x < 0 {
                    self.min = self.min - x;
                }
            }
        }
    }

    fn top(&self) -> i32 {
        match self.data.last() {
            Some(&x) => {
                if x < 0 {
                    self.min as i32
                } else {
                    (x + self.min) as i32
                }
            }
            _ => 0
        }
    }

    fn get_min(&self) -> i32 {
        self.min as i32
    }
}

struct MinStack2 {
    data: Vec<i32>,
    min: i32
}

impl MinStack2 {
    fn new() -> Self {
        MinStack2 {
            data: vec![],
            min: i32::max_value()
        }        
    }

    fn push(&mut self, x: i32) {
        if x <= self.min {
            self.data.push(self.min);
            self.min = x;
        }
        self.data.push(x)
    }

    fn pop(&mut self) {
        if let Some(x) = self.data.pop() {
            if x == self.min {
                self.min = self.data.pop().unwrap();
            }
        }
    }

    fn top(&self) -> i32 {
        match self.data.last() {
            Some(&x) => x,
            _ => 0
        }
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_test() {
        {
            let mut sp = MinStack1::new();
            sp.push(-2);
            sp.push(0);
            sp.push(-3);
            assert_eq!(sp.get_min(), -3);
            sp.pop();
            assert_eq!(sp.top(), 0);
            assert_eq!(sp.get_min(), -2);
            sp.push(-2);
            assert_eq!(sp.top(), -2);
            sp.pop();
            assert_eq!(sp.get_min(), -2);
            sp.pop();
            assert_eq!(sp.get_min(), -2);
        }
        {
            let mut sp = MinStack1::new();
            sp.push(-90);
            sp.push(-2147483648);
            assert_eq!(sp.top(), -2147483648);
            assert_eq!(sp.get_min(), -2147483648);
            sp.pop();
            assert_eq!(sp.top(), -90);
            assert_eq!(sp.get_min(), -90);
        }
        {
            let mut sp = MinStack2::new();
            sp.push(-2);
            sp.push(0);
            sp.push(-3);
            assert_eq!(sp.get_min(), -3);
            sp.pop();
            assert_eq!(sp.top(), 0);
            assert_eq!(sp.get_min(), -2);
            sp.push(-2);
            assert_eq!(sp.top(), -2);
            sp.pop();
            assert_eq!(sp.get_min(), -2);
            sp.pop();
            assert_eq!(sp.get_min(), -2);
        }
        {
            let mut sp = MinStack2::new();
            sp.push(-90);
            sp.push(-2147483648);
            assert_eq!(sp.top(), -2147483648);
            assert_eq!(sp.get_min(), -2147483648);
            sp.pop();
            assert_eq!(sp.top(), -90);
            assert_eq!(sp.get_min(), -90);
        }
    }
}
