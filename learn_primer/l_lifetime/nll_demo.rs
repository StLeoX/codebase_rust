fn capitalize(data: &mut [char]) {
    // do something
}

// 可变借用的规则：在同一作用域内，变量只能串行地出借可变借用。

// 引入NLL之前，必须采用花括号，在词法作用域中对先前的可变借用进行销毁。
fn main() {
    let mut data = vec!['a', 'b', 'c', 'd'];
    {
        let slice = &mut data[..];  // 创建可变借用
        capitalize(slice);  // 消费可变借用slice
    }
    data.push('e');  // 再次创建data的可变借用并消费
}


// 引入NLL后允许写这种线性代码
fn main() {
    let mut data = vec!['a', 'b', 'c', 'd'];
    let slice = &mut data[..];  // 创建可变借用
    capitalize(slice);  // 消费可变借用slice
    data.push('e');  // 再次创建data的可变借用并消费

    // capitalize(slice); // 此时NLL已经销毁借用slice了，再访问会报错。
}


////
// NLL demo02
struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}

fn to_refs<T>(mut list: &mut List<T>) -> Vec<&mut T> {
    let mut ret = vec![];
    loop {
        ret.push(&mut list.value);
        if let Some(node) = list.next.as_mut() {
            list = node;
        } else {
            return ret;
        }
    }
}
