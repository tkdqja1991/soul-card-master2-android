# SCM2 PC runtime mimic patch

업로드된 PC 실행본과 Android 소스를 비교해 확인한 차이를 반영합니다.

1. PC 런처는 `--program-name 010100D2`를 사용합니다.
   - SCM2(AID 010261FB)에 한해서 `MC_knlGetProgramName`이 `010100D2`를 반환하도록 맞춥니다.
2. PC 실행본의 저장소에는 `/certify`, `/mix`가 루트 경로로 존재합니다.
   - SCM2 패키지는 이 파일들을 `p/certify`, `p/mix`로 넣고 있는데 기존 Android 소스는 대문자 `P/`만 제거합니다.
   - `p/`와 `P/` 둘 다 게스트 루트로 노출하도록 수정합니다.

기존 SCM2 version-gate/FakeSocket 패치 파일은 건드리지 않습니다.
