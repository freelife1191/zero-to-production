# 제로부터 시작하는 러스트 백엔드 프로그래밍 실습


<!-- TOC -->
* [제로부터 시작하는 러스트 백엔드 프로그래밍](#제로부터-시작하는-러스트-백엔드-프로그래밍)
* [1. 시작하기](#1-시작하기)
  * [1.1 러스트 툴 체인 설치하기](#11-러스트-툴-체인-설치하기)
  * [1.2 프로젝트 셋업](#12-프로젝트-셋업)
  * [1.4 내부 개발 루프](#14-내부-개발-루프)
    * [1.4.1 빠른 링킹](#141-빠른-링킹)
    * [1.4.2 cargo-watch](#142-cargo-watch)
  * [1.5 지속적인 통합](#15-지속적인-통합)
    * [1.5.1 CI 단계](#151-ci-단계)
* [3. 신규 구독자로 등록하기](#3-신규-구독자로-등록하기)
  * [3.3 첫 번째 엔드포인트: 기본 헬스 체크](#33-첫-번째-엔드포인트-기본-헬스-체크)
    * [3.3.2 actix-web 애플리케이션 구조](#332-actix-web-애플리케이션-구조)
      * [4. 런타임: tokio](#4-런타임-tokio)
<!-- TOC -->


# 1. 시작하기

---

## 1.1 러스트 툴 체인 설치하기

---

- 예제 코드: https://github.com/LukeMathWalker/zero-to-production

[rustup](https://rustup.rs/) 설치
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

설치 후 실행
```bash
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
```


## 1.2 프로젝트 셋업

---

rust 버전 확인
```bash
rustc --version
```

cargo 버전 확인
```bash
cargo --version
```

프로젝트 생성 및 Git 연결
```bash
cargo new zero2prod
```


## 1.4 내부 개발 루프

---

### 1.4.1 빠른 링킹

```toml
# .cargo/config.toml

# On Window
# ```
# cargo install -f cargo-binutils
# rustup component add llvm-tools-preview
# ```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# On Linux:
# - Ubuntu, `sudo apt-get install lld clang`
# - Arch, `sudo pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

# On MacOS, `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```

### 1.4.2 cargo-watch

cargo-watch 설치
```bash
cargo install cargo-watch
```

코드가 변경될 때마다 cargo check  수행
```bash
cargo watch -x check
```

cargo-watch command chaining 지원 cargo check 성공 cargo test 성공 cargo run 애플리케이션 실행
```bash
cargo watch -x check -x test -x run
```


## 1.5 지속적인 통합

---

### 1.5.1 CI 단계

테스트
```shell
cargo test
```


코드 커버리지
```shell
# tarpaulin은 리눅스를 실행하는 x86_64 CPU 아키텍처만 지원한다
cargo install cargo-tarpaulin

# 테스트 함수를 제외한 애플리케이션 코드에 대해서만 코드 커버리지 계산
cargo tarpaulin --ignore-tests
```


린터(linter)는 자연스럽지 않은 코드, 과도하게 복잡한 구조, 일반적인 실수, 비효율성 등을 찾아낸다

린팅
```shell
# clippy 설치
rustup component add clippy

# clippy 실행
cargo clippy

# CI 파이프라인에서 clippy가 warning을 발생시키면 린터 체크 실패
cargo clippy -- -D warnings
```

- `#[allow(clippy::lint_name)]`:  대상 코드 블록에서 warning에 대한 체크를 비활성화
    - `clippy.toml` 설정 또는 프로젝트 레벨에서 `#![allow(clippy::lint_name)]` 지시를 사용해 린트 메세지 모두 비활성화

rustfmt 공식 러스트 포매터 설치
```shell
rustup component add rustfmt

# 프로젝트 전체 형태 다듬기
cargo fmt

# CI 파이프라인에 다음 포매팅 단계 추가
cargo fmt -- --check
```

보안 취약점
프로젝트의 의존성 트리에 존재하는 모든 크레이트와 관련해 보고된 취약점 확인 cargo-audiot 설치
```shell
cargo install cargo-audit
```



# 3. 신규 구독자로 등록하기

---

## 3.3 첫 번째 엔드포인트: 기본 헬스 체크

---

### 3.3.2 actix-web 애플리케이션 구조

#### 4. 런타임: tokio

- cargo-expand
    - 출력 결과를 컴파일러에 전달하지 않은 상태에서 코드 안의 모든 매크로를 펼친다
    - 매크로를 탐색하면서 어떤 일이 발생하는지 확인할  수 있다
    - stable 컴파일러를 사용해서 코드를 빌드하고 테스트하고 실행한다
    - nightly 컴파일러를 사용해서 매크로를 확장한다

cargo expand 설치
```shell
cargo install cargo-expand

# 사용
cargo expand

# nightly 컴ㅏ일러 설치
rustup toolchain install nightly --allow-downgrade

# 이 명령어 호출에만 nightly 툴체인을 사용한다
cargo +nightly expand
```

`--allow-downgrade` 를 사용하면 rustup은 필요한 컴포넌트들을 모두 사용할 수 있는 가장 최신 nightly 빌드를 찾아서 설치한다

`rustup default` 명령어를 사용하면 cargo가 사용하는 기본 툴체인과 rustup이 관리하는 다른 도구들을 변경할 수 있다