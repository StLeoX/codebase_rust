// 01
// 针对trait间的关系，加号表示取交集，冒号表示等价
trait tr1 : tr2 + tr3 ;
// 在集合论中的表示：
// tr1 = tr2 ∩ tr3

// 02
// trait的实现impl，本质是trait的组合
impl tr1 for T; impl tr2 for T;... ;impl trn for T; 
// 在集合论中的表示：
// T = tr1 ∪ tr2 ∪... trn

// 03
// where 关键字的使用
impl <T: A + B> tr1 for T;
// 等价于写：
impl tr1 for T where T: A + B
// 在集合论中的表示：
// tr1 ⊂ T, T = A ∩ B

// 04
// where关键字解决泛型嵌套的形式
fn foo<T:A,K:B+C,R:D>(a:T, k:K, r:R) {...}
// 等价于
fn foo<T,K,R>(a:T, k:K, r:R) where T:A,K:B+C,R:D {...}