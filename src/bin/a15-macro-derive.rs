#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

// derive macro
/**
 * Copy, Clone
 * 호출 시 소유권을 넘기지 않음.
 *
 * - 대체로 구조체 내의 엔트리가 4~5개 일때 사용
 * - 이상일 경우 사용 X
 * - 메모리 용량점유 증가
 */
#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    //  derive 매크로 없을 시
    // match me.position {
    //     Position::Manager => println!("manager"),
    //     Position::Supervisor => println!("supervisor"),
    //     Position::Worker => println!("worker"),
    // }

    println!("{:?}", me)
}
