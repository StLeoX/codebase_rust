// trait的继承
trait Page {
    fn page_set(self){}
}

trait PerPage {
    fn per_page_set(self){}
}

// 声明继承的trait
trait Paginate : Page + PerPage {
    fn paginate(self){}
}

// 实现继承的trait，静态分发的方式
impl <T: Page + PerPage> Paginate for T {}
