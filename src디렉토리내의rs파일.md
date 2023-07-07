# 짜증나는 상황..💢
### 1. src 디렉토리에는 main.rs 파일이 기본적으로 있음.<br>
### 2. 코드 실행한다고 이름이 다른 ```.rs``` 파일을 여러 개 만들었는데,<br>수정하고 ```cargo run``` 한다고해서 해당 파일이 실행되지 않았음.
<br><br>
# 좀 찝찝하지만 그나마 해결에 준할 수 있는 방법
### 1. 파일을 모조리 다 binary 설정에 대응시키기
➡️ 비효율적임. 너무 노동력이 많이 듦. 계속 Cargo.toml 편집만 하고 있을 수 없음.<br><br>
일단 방법은,
1. 생성된 프로젝트 내의 ```Cargo.toml``` 파일을 편집기로 열기<br><br>
2. 열면 기본적으로
  ```vim
  [package]
  name = "rectangles"
  version = "0.1.0"
  edition = "2021"
  
  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
  
  [dependencies]
  ```
   <br>
   대충 이렇게 생긴 코드가 있음. 이 코드 밑에 <code>[[bin]]</code>을 사용한 코드를 추가.
<br><br>

3. 예시
  ```vim
  [package]
  name = "rectangles"
  version = "0.1.0"
  edition = "2021"
  
  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
  
  [dependencies]
  
  [[bin]]
  name = "main3"
  path = "src/main3.rs"

  [[bin]]
  name = "main4"
  path = "src/main4.rs"
  ```
<br><br>
4. ```cargo run``` 해야할 때마다 ```cargo run --bin main3``` 이런 식으로 입력하면됨
<br><br>
### 2. <code>cargo run</code>의 기본 진입경로가 <code>src/main.rs</code>인 점 이용
실행할 파일의 이름을 임시로 ```main.rs```로 바꾸기<br>
➡️ 위 방법보다 이게 낫긴한데 이것도 뭔가... 뭔가...... 이상함......<br>
1. ```$ mv 실행하고자하는파일.rs main.rs``` (물론 기존 ```main.rs```는 또 옮겨줘야함;;^^)
2. ```$ cargo run```
➕ 여기서는 ```Cargo.toml``` 건드릴 필요 ❌
