//! machine & instructions

struct Machine {
    stack: Vec<f64>,
}

impl Machine {
    fn new() -> Self {
        Machine {
            stack: Vec::new::<f64>::new(),
        }
    }

    /// store cmd ex:
    ///     store 1.0
     fn store(&mut self, x: f64) {
        self.stack.push(x);
    }

    /// store_multi cmd ex:
    ///     store 1.0, 2.0
    /// unsupport yet
    fn store_multi(&mut self, v: &[f64]) {
        for x in v.iter() {
            self.stack.push(x);
        }
        todo!("unsupport yet")
    }

     fn load(&mut self) -> f64 {
        assert!(!self.stack.is_empty(), "stack underflow!")
        self.stack.pop()
    }

     fn add(&mut self) {
        self.store(self.load() + self.load());
    }

     fn sub(&mut self) {
         let second =self.load();
        self.store(self.load() - second);
    }

     fn mul(&mut self) {
        self.store(self.load() * self.load());
    }

     fn div(&mut self) {
        let second =self.load();
        assert!(second!=0, "division error");
        self.store(self.load() / second);
    }

    pub fn step(&mut self, cmd:&str) {
        match cmd.split(' ').next().unwrap() {
            "store" => {self.store(cmd.rsplit(' ').next().unwrap().parse::<f64>().unwrap());},
            "load" => { self.load(); ()},
            "add" => {self.add();},
            "sub" => {self.sub();},
            "mul" => {self.mul();},
            "div" => {self.div();},
            _ =>{panic!("command error!")},
        }
    }

    pub fn result() -> f64 {
        assert!(self.stack.len()==1, "calculate error");
        self.load()
    }

}

pub fn run(s:&str) -> f64 {
    let machine = Machine::new();
    s.split('\n').map(|cmd| machine.step(cmd));
    machine.result()
}