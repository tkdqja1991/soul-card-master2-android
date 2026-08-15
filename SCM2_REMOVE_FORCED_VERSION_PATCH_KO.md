# SCM2 강제 버전분기 제거 테스트

유지:
- `p/`/`P/` 파일을 게스트 루트로 노출하는 PC 런타임 호환 수정
- SCM2의 프로그램 이름을 PC 런처와 동일한 `010100D2`로 반환하는 수정

제거:
- `data/binary_patches.toml`에 추가했던 0x12db46 / 0x12dbfe 강제 분기 패치
- `register_scm2_fake_socket()`에서 런타임에 게스트 ARM 코드를 덮어쓰는 패치

이유:
PC용 정상 실행 엔진에는 해당 SCM2 전용 패치 문자열이 없고, Android에서 PC 런타임 환경을 맞춘 뒤 버전 확인은 이미 사라졌습니다. 그 상태에서 상태값 준비 없이 성공 분기로 강제 점프하면 이후 코드가 전제하는 데이터가 없어 WASM/Rust trap(`RuntimeError: unreachable`)을 만들 가능성이 큽니다.
