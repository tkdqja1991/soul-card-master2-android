# SCM2 class-load name trace

이 패치는 현재 `RuntimeError: unreachable` 직전에 요청되는 Java 클래스 이름을 확인하기 위한 진단용 패치입니다.

추가 로그 예시:

- `SCM2 TRACE: KTF loadClass name=...`
- `SCM2 TRACE: KTF loadClass return=CLASS`
- `SCM2 TRACE: KTF loadClass error=...`

기존 게임 동작을 우회하지 않고 `net/wie/KtfClassLoader.loadClass`에서 부모 구현을 호출하면서 이름과 결과만 기록합니다.
