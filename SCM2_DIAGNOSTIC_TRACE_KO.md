# SCM2 진단 TRACE 패치

게임 동작을 우회하는 패치가 아니라, `RuntimeError: unreachable` 직전에 호출된 Java/WIPIC API와 Rust panic 메시지를 화면 상단 상태창에 남기기 위한 진단 패치입니다.

변경 파일:
- wie_ktf/src/runtime/wipi_c.rs
- wie_ktf/src/runtime/java/jvm_support/method.rs
- wie_web/src/ts/index.ts

오류가 나면 상단 상태창 맨 아래의 `--- 최근 SCM2 TRACE ---` 부분을 캡처하면 마지막 호출을 특정할 수 있습니다.
