# SCM2 clean UI patch

변경 내용
- 화면 상단의 SCM2 TRACE / PANIC 로그 표시 제거
- 진단용 Java/WIPI/ClassLoader TRACE 출력 제거
- 게임 화면 크기를 디버그용 58vw에서 기본 82vw(최대 360px)로 확대
- 방향키/OK/CLR을 3행 그리드로 재배치
  - 1행: ▲
  - 2행: ◀ / OK / ▶
  - 3행: ▼ / CLR
- 숫자키는 별도 3x4 그리드로 정리
- 짧은 화면에서는 자동으로 화면/버튼 크기를 조금 줄여 세로 공간 확보
- SCM2 null Clip 오디오 크래시 수정은 건드리지 않음

적용 후에도 세부 위치/크기는 CSS 숫자만 바꾸면 쉽게 조정 가능합니다.
