
struct Circle {
    x: f32,
    y: f32,  // center
    r: f32,  // radius
}

struct CircleBuilder {
    x: f32,
    y: f32,
    r: f32,
}

impl Circle {
    // new不是返回circle，而是返回circleBuilder；这是一种委托
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            r: 0.0,
        }
    }

    // 平凡的方法
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.r * self.r
    }
}


impl CircleBuilder {
    // builder 本质上就是由一组setter构成的
    fn set_x(&mut self, x: f32) -> &mut CircleBuilder {
        self.x = x;
        self
    }

    fn set_y(&mut self, y: f32) -> &mut CircleBuilder {
        self.y = y;
        self
    }

    fn set_r(&mut self, r: f32) -> &mut CircleBuilder {
        self.r = r;
        self
    }

    fn build(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            r: self.r,
        }
    }
}

fn main() {
    let c = Circle::new()  // 不变的部分
    .set_x(1.0).set_y(1.0).set_r(1.0)  // 可变的部分
    .build();
    println!("{}", c.area());
}