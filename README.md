# 러스트 정리 _ 여름 방학 ! ☀️☀️
[*KR*](https://rinthel.github.io/rust-lang-book-ko/)<br>
[*ENG*](https://doc.rust-lang.org/book/)<br>

---

### 08장 관련
스네이크 케이스를 비롯한 [표기법들](https://velog.io/@leyuri/%ED%91%9C%EA%B8%B0%EB%B2%95-%EC%8A%A4%EB%84%A4%EC%9D%B4%ED%81%AC-%EC%BC%80%EC%9D%B4%EC%8A%A4-%ED%8C%8C%EC%8A%A4%EC%B9%BC-%EC%BC%80%EC%9D%B4%EC%8A%A4-%EC%B9%B4%EB%A9%9C-%EC%BC%80%EC%9D%B4%EC%8A%A4)<br>
|이름|설명|타입|
|:---:|:---:|:---:|
|문자 리터럴|단일 문자|char|
|스트링 리터럴|하나 이상의 문자|&str|
<br>

#### 문자열을 나타내는 타입
* ```String``` 타입
    * 크기 조정, 타입 변경이 가능하고, 소유권을 가지며 UTF-8로 인코딩된 스트링 타입
    * 힙에 할당됨
* ```&str``` 타입
    * 문자열 슬라이스를 나타내는 타입
    * ```immutable```하며, 문자열의 일부를 참조함
    * 문자열 리터럴이나 String 일부분 참조할 때 주로 사용
    * String 타입과 달리 크기 고정
<details>
<summary>사용 예시</summary>
<div markdown="1">

```rust
fn main() {
    let mut s = String::new(); // 빈 String 생성
    s.push_str("Hello"); // 문자열을 String에 추가
    println!("{}", s); // 출력: Hello
}
```
```rust
fn main() {
    let s: &str = "Hello"; // 문자열 리터럴로부터 &str 변수 생성
    println!("{}", s); // 출력: Hello
}
```
</details>

---
