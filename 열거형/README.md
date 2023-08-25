# Enum

## Fundamentals
- 여러 가능한 값중 하나인 데이터 조각
  - 각각의 가능한 값을 열거값이라고 부름
- 컴파일러에 프로그램에 관한 정보를 제공함



## 사용법

- 기본
```rust
enum Direction {
  Up,
  Down,
  Left,
  Right
}
```
열거형을 생성하기 위해 콤마로 구분되는 열거값을 작성합니다.
열거형은 `match` 표현식과 결합 시 프로그램을 안정적으로 만들 수 있습니다.

- 사용법
```rust
enum Direction {
  Up,
  Down,
  Left,
  Right
}

fn which_way(go: Direction){
  match go {
    Direction::Up => "up",
    Direction::Down => "down",
    Direction::Left => "left",
    Direction::Right => "right",
  }
}
```
열거형에 정의된 모든 열거값을 평가해야 하기 때문에, 개발자의 실수로 인한 오류를 방지할 수 있습니다.

## 정리
- 열거형은 한번에 하나의 열거값만 될 수 있습니다.
- `match` 표현식과 함께 사용하면 프로그램을 안정적으로 만들 수 있습니다.
- 코드의 가독성을 높혀줍니다.