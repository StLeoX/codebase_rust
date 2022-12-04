// 实现向量的Any方法：将闭包作为函数
// 具体逻辑是：闭包是一个谓词表达式，判断向量中是否存在任意元素满足该谓词表达式

// 方式一：trait限定
trait Any {
    fn any<F>(&self, f: F) -> bool where
    Self: Sized,
    F: Fn(i32) -> bool;  // 注意到Fn Trait是fundamental，不需要use
}

impl Any for Vec<i32> {
    fn any<F>(&self, f: F) -> bool where
    Self: Sized,
    F: Fn(i32) -> bool {
        for &i in self.iter() {
            if f(i) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let v = vec![1,2,3];
    let result = v.any(|x| x==3);
    println!("{}", result);
}


////////////////////////////////
// 方式二：trait对象
trait Any {
    fn any(&self, f: &(dyn Fn(i32) -> bool)) -> bool;
}

impl Any for Vec<i32> {
    fn any(&self, f: &(dyn Fn(i32) -> bool)) -> bool {
        for &i in self.iter() {
            if f(i) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let v = vec![1,2,3];
    let result = v.any(&|x| x==3);  // 调用方式也发生变化了
    println!("{}", result);
}
