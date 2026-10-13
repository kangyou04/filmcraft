# FilmCraft 비공식 한국어판

이 버전은 **kangyou04가 배포하는 비공식 Windows x64 한국어판**입니다.
FilmCraft 제작사의 공식 배포본이나 공식 지원 제품이 아닙니다.
FilmCraft 0.7.2과 loeness의 한국어 지원 PR #963에 기반하며,
kangyou04가 편집 용어 11개를 개선했습니다. 이 별도 배포본에는 추가로 한국어 표현 19개 개선,
효과 검색 별칭과 주요 오류의 한국어 안내가 포함됩니다. 공통 그래픽 용어 38개를 익숙한 한국어 표현으로 정리했습니다.
중국어 번체 지원도 기반 PR에 포함되어 있습니다.

## 다운로드와 실행

1. 이 저장소의 Releases에서 `FilmCraft-Korean-v0.7.2-ko.1-windows-x64.zip`을 받습니다.
2. ZIP을 **전부 압축 해제**해 쓰기 가능한 폴더에 둡니다.
3. `filmcraft.exe`를 실행하면 한국어 메뉴로 시작합니다. 설치 과정은 없습니다.
4. 설정, 로그와 복구 자료는 실행 파일 옆 `data` 폴더에 저장됩니다.

Windows 10/11 x64용입니다. ARM64 및 32비트용 실행 파일이 아닙니다.
이 한국어판은 개발 프로필로 빌드했으며, 최적화된 공식 릴리스와 성능이 다를 수 있습니다.
Microsoft 코드 서명 인증서는 사용하지 않았습니다.
Windows C 런타임은 정적으로 연결하므로 별도 Visual C++ 재배포 패키지 설치가 필요하지 않습니다.
프로그램을 실행할 수 없는 경우 보안 경고를 무시하지 말고 원인과 파일 해시를 확인하세요.

한국어가 표시되지 않으면 Edit → Preferences → General에서 Interface Language를 한국어로 선택합니다.
`data/preferences.json`의 초기 한국어 설정을 제거했거나 외부 데이터 폴더를 지정하면 언어 설정이 달라질 수 있습니다.

## MCP 사용 (선택)

일반 실행에서는 MCP 제어 서버를 열지 않습니다.
MCP가 필요하면 일반 실행 창을 닫고 `MCP-시작.cmd`로 실행합니다.
제어 주소는 `127.0.0.1:9877`이며, 같은 포트를 다른 앱이 쓰고 있으면 바꿔야 합니다.

Codex 등 MCP 클라이언트에서는 ZIP에 포함된 CLI의 절대 경로를 지정합니다.

```toml
[mcp_servers.filmcraft]
command = 'C:\압축을푼폴더\filmcraft-cli.exe'
args = ['mcp', '--bridge', '127.0.0.1:9877']
```

설정을 바꾼 뒤 MCP 클라이언트를 다시 시작합니다. 기존 개인 설정 파일, API 키 및 영상 프로젝트는 이 배포에 포함하지 않습니다.

## 출처와 변경 사항

- 원 프로젝트: https://github.com/storytold/filmcraft
- 한국어/중국어 번체 지원: loeness, https://github.com/storytold/filmcraft/pull/963
- 한국어 용어 개선: kangyou04, 커밋 `d08a34b0962226d160316f43183a41796666be57`
- 추가 한국어 개선: 빈 → 미디어 폴더 등 19개 표현, 블러·쉐도우·테두리·외곽선 검색 별칭.
- 그래픽 공통 용어: 획, 칠, 그림자 효과 등 38개 표현 정리. 기존 자간·행간·커닝·혼합 모드·불투명도는 유지.
- 용어 출처: https://helpx.adobe.com/kr/photoshop/desktop/create-manage-layers/apply-layer-effects/layer-style-effects-and-options-overview.html 및 https://helpx.adobe.com/kr/photoshop/using/line-character-spacing.html
- Adobe 이미지, 아이콘, 글꼴이나 프로그램 코드를 사용하지 않습니다.
- 한국어 오류 안내: 시퀀스/항목 미선택, 미디어 누락, 형식/설정 미지원, 디코딩/인코딩, 파일 읽기·쓰기와 취소.
- 오류 상세 원문과 MCP/CLI의 오류 반환은 유지합니다. 모든 영어 오류를 번역한 것은 아닙니다.
- 편집 결과나 엔진의 오류 판정은 변경하지 않았습니다.
- 별도 배포 조건에 따라 ArtCraft 상표 로고 및 이를 표시하는 코드를 제거했습니다.
- 정보 창에 비공식 미리보기와 원 작업자의 출처를 표시합니다.
- 번역 개선 PR은 유지되며, 이 배포용 수정은 별도 `release-korean-v0.7.2` 브랜치에 있습니다.

MIT 또는 Apache-2.0 라이선스로 배포합니다. 원 제작진과 기존 번역자의 저작권 및 기여를 유지합니다.
ZIP의 `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `ATTRIBUTION.md`와 폰트 라이선스를 참고하세요.
`SHA256SUMS.txt`로 내려받은 ZIP의 SHA-256을 확인할 수 있습니다.

## 같은 소스에서 빌드하기

`release-korean-v0.7.2` 브랜치에서 Windows MSVC용 Rust와 Microsoft 빌드 도구를 사용합니다.
PowerShell에서 아래 환경 변수를 설정한 뒤 빌드합니다. 새 Rust 패키지는 추가하지 않습니다.

```powershell
$env:CARGO_PROFILE_DEV_DEBUG = '0'
$env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS = '-C target-feature=+crt-static'
$env:FILMCRAFT_REQUIRE_WINRES = '1'
cargo build --locked -p filmcraft -p filmcraft-cli --target x86_64-pc-windows-msvc
```

실행 파일은 `target/x86_64-pc-windows-msvc/debug`에 생성됩니다.
배포 ZIP은 `portable.txt`와 `data/preferences.json` (`{"version":2,"general":{"interfaceLanguage":"ko"}}`)을 함께 넣어 만듭니다.

## 검증 범위

한국어 카탈로그 테스트, 배포용 빌드, 자산/의존성 계층 검사와 압축 해제한 실행 파일의 한국어 상태 및 MCP 응답을 확인합니다.
원본 코드에는 Windows에서 전체 워크스페이스 검사를 막는 기존 `std::os::unix` 테스트와 clippy 경고가 있습니다.
모든 영상 형식과 모든 PC의 호환성을 검증한 정식 릴리스는 아닙니다. 중요한 원본 프로젝트는 복사본으로 먼저 확인하세요.
