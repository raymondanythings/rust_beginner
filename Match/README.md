# Match

## Fundamentals
- 로직을 추가하는데에 사용
- if..else 문과 비슷함
- 차이점
  - 모든 표현식이 일치해야 함

## 사용법

- 기본
```rust
fn main(){
  let some_bool = true;
  match some_bool {
    true => println!("its true!"),
    false => println!("its false"),
  }
}
```

위 코드는 `some_bool` match 표현식이며, `some_bool`은 `true` 또는 `false` 2가지 값을 가질 수 있습니다.
따라서, match 표현식에선 2가지의 표현식 평가를 구현해야 합니다.

- 다양한 가능성
```rust
fn main(){
  let some_int = 3;
  match some_int {
    1 => println!("its 1"),
    2 => println!("its 2"),
    3 => println!("its 3"),
    _ => println!("its something else"),
  }
}
```
`int`와 같이 다양한 가능성을 평가해야 하는 match 표현식일 경우, `_`를 통해 다른 모든 가능성을 평가할 수 있습니다.

## `match`와 `else..if` 의 차이점
- `match` 는 컴파일러가 모든 가능성을 확인합니다.
  - 만약 확인하지 않은 가능성이 있거나, 새로운 가능성이 추가 될 경우, 컴파일러가 이를 알려줍니다.


## 정리
- 단일 변수로 작업할 경우, `else..if` 보다 `match`가 좋습니다.
- `match`는 모든 가능성을 고려하기 떄문에, 확실한 코드를 작성할 수 있습니다.
- 다양한 가능성 중 몇가지만 처리할 경우, `_` 를 통해 이외의 모든 가능성을 확인 할 수 있습니다.