use regex::Regex

fn main() {
    let re = regex::Regex::new(r"(?x)  # 修饰符x
    (?<year>\d{4})-  # 尖括号捕获参数
    (?<month>\d{2})- 
    (?<day>\d{2})
    ").unwrap();

    let matches = re.captures("2018-01-02").unwrap();
    assert_eq!(&matches["year"], 2018);
    assert_eq!(&matches["month"], 01);
    assert_eq!(&matches["day"], 02);

    let replaced = re.replace_all("2018-01-02", "$month/$day/$year");  // 先捕获再替换
    assert_eq!(replaced, "02/01/2018");
}