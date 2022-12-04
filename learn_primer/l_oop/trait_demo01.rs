struct Duck;  //单元结构体
struct Pig;

trait Fly {  // trait的声明
    fn fly(&self) -> bool;
}

impl Fly for Duck {  // trait的实现
    fn fly(&self) -> bool {true}
}

impl Fly for Pig {
    fn fly(&self) -> bool {false}
}

// 基于静态类型的委托
fn fly_static<T: Fly>(obj: T) -> bool {
    obj.fly()
}

// 基于动态类型的委托
fn fly_dynamic(obj: &dyn Fly) -> bool {  // 注意使用dyn修饰引用
    obj.fly()
}

fn main() {
    
    let pig1 = Pig;  // 单元结构体的构造
    // 调用静态类型委托的方式，静态分发
    assert_eq!(fly_static::<Pig>(pig1), false);  // ::<Pig>表示泛型T实例化为类型Pig

    // 注意fly_static的调用必须紧跟let，不能交换该语句与上一条语句的顺序。？原因不理解
    let duck1 = Duck;  
    assert_eq!(fly_static::<Duck>(duck1), true);

    // 调用动态类型委托的方式，动态分发
    // assert_eq!(fly_dynamic(&pig1), false);
    // assert_eq!(fly_dynamic(&duck1), true);

    // 注意上面两种写法是错误的；针对单元结构体的实参，必须取结构体的引用
    assert_eq!(fly_dynamic(&Pig), false);
    assert_eq!(fly_dynamic(&Duck), false);

}
