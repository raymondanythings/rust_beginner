# 반복문

## Fundamentals
- loop
  - 특정 로직을 영원히 반복 할 경우 사용
  - 다른 언어의 `while` 문과 비슷함
- while
  - 특정 조건에 따라 로직을 수행할 경우 사용

## 사용법

### loop
- 기본
```rust
fn main(){
  loop {
    println("Hello");
  }
}
```
위 코드는 프로그램이 종료되지 않고 `Hello`가 영원히 출력됩니다.

- 종료시점 설정
```rust
fn main(){
  let mut i = 3;
  loop {
    println!("{:?}",i);
    i -= 1;
    if i == 0 {
      break;
    }
  }
  println!("DONE");
}
```
`break`로 종료시점을 설정하기 위해 가변변수와 함께 사용

### while

- 기본
```rust
fn main(){
  let mut i = 1;
  while i <= 3  {
    println!("{:?}",i);
    i += 1;
  }
}
```
`while` 이후 로직이 수행되는 조건을 명시합니다.

## 정리
- `loop`는 특정 로직을 영원히 반복하기 위해 사용
- `break`를 통해 종료 시점 설정 가능
- 종료 시점 설정을 위해 가변변수와 함께 사용
- `while`은 특정 조건을 명시하여 로직 수행 여부를 결정합니다.