# FilmCraft 0.9.0 한국어 커뮤니티 빌드 (ko.1)

[Windows x64 다운로드](https://github.com/kangyou04/filmcraft/releases/tag/v0.9.0-ko.1) · [원본 v0.9.0 변경 내역](https://github.com/storytold/filmcraft/releases/tag/v0.9.0)

공식 FilmCraft v0.9.0의 한국어 카탈로그를 바탕으로 Windows x64용으로 다시 빌드한 비공식 커뮤니티 배포본입니다. 한국어 카탈로그 원작성은 **CokeRoach**의 [PR #508](https://github.com/storytold/filmcraft/pull/508)에 있습니다. **kangyou04**가 편집 용어 11개를 더 분명하게 다듬었습니다.

## 설치 및 MCP

ZIP을 새 폴더에 풀고 `FilmCraft-Korean.cmd`를 실행하세요. 한글 설정은 동봉한 portable `data/preferences.json`에 저장됩니다. 이전 휴대용 설정을 유지하려면 앱을 닫고 기존 `data` 폴더를 새 패키지에 복사하세요. 기존 프로젝트는 건드리지 않습니다.

동봉한 `filmcraft-cli.exe`와 GUI는 모두 v0.9.0에서 빌드했습니다. 기존 MCP 서버 구성을 그대로 사용할 수 있도록 GUI 실행 파일 이름과 제어 포트 9877을 유지합니다. `MCP-시작.cmd`는 같은 패키지 안에서 해당 포트를 열어 시작합니다. 기존 MCP 설정 파일은 수정하지 않았습니다.

## 포함 내용

- 최신 공식 v0.9.0 편집 기능과 수정 사항
- 공식 한국어 카탈로그와 kangyou04의 11개 편집 용어 개선
- ArtCraft 로고 자산을 제거한 커뮤니티 패키지
- 라이선스·출처 안내, 실행 정보와 SHA-256

이 배포본은 FilmCraft 제작팀의 공식 배포가 아니며 Windows 코드 서명이 없습니다. 한국어 지원 원작성자와 upstream 기여자 표시는 유지했습니다.
