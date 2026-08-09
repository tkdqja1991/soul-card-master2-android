# 소울카드마스터2 WIE Android 전용 빌드

이 소스는 dlunch/wie의 Android/Tauri 프론트엔드에 소울카드마스터2 WIPI 패키지를 내장하도록 수정한 테스트 빌드입니다.

## 변경점
- `wie_web/public/scm2.wipi.zip`에 게임 패키지 내장
- 파일 선택 화면 없이 앱 시작 시 자동 실행
- 240x320 게임 화면 + 터치 키패드 UI
- WIE WebFilesystem/IndexedDB 기반 저장 기능 유지
- 앱 이름/식별자를 소울카드마스터2 전용으로 변경

## 가장 쉬운 APK 빌드 방법
GitHub 저장소에 이 폴더 전체를 올린 다음 Actions > `Build SCM2 Android APK` > Run workflow를 실행합니다.
성공하면 실행 결과의 Artifacts에 `SoulCardMaster2-Android-APK`가 생성됩니다.

## 로컬 빌드 핵심 명령
Rust, wasm-pack, Node.js, Java 21, Android SDK/NDK, Tauri CLI가 준비되어 있어야 합니다.

```bash
npm ci
npm run build:prod
cd wie_app
cargo tauri android init --ci
cargo tauri android build --target aarch64 --apk --ci --config '{"build":{"beforeBuildCommand":""}}'
```

## 주의
이 수정본은 소스 구조에 맞춰 제작한 포팅 테스트 버전입니다. 실제 Android 기기에서의 게임 부팅/사운드/세이브 여부는 APK를 만든 뒤 확인해야 합니다.
