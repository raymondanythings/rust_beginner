// 컴파일 오류 => 스트럭트 생성 후 소유권 드롭 불가 -> 데이터에 대한 소유권이 없기 때문.
// struct Employee {
//     name: &str,
// }

struct Employee {
    name: String,
}

fn main() {
    let emp_name = "Jayson".to_owned();
    let emp_name = String::from("Jayson");
    let emp = Employee { name: emp_name };
    println!("{}", emp.name)
}
