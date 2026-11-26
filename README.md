# hyperish/opale

하이퍼리쉬(Hyperish) 프레임워크 패밀리의 **고성능 크로스플랫폼 그래픽 프레임워크** 레포입니다.

```rust
use opale::prelude::*;

let viewport = ViewportDesc::new(1920, 1080)
    .with_backend(GraphicsBackend::Metal)
    .with_clear_color(Color::rgba(0.05, 0.06, 0.08, 1.0));
```

상위 범용 명세와 문서 상태 규칙(`확정` / `제안` / `미정`)은 [`hyperish/docs`](https://github.com/hyperish/docs)를 따릅니다.

앱이나 에디터가 제공하는 렌더 타깃에 3D 장면을 그리는 라이브러리입니다. 현재 크레이트는 위 예시의 색·크기·백엔드·뷰포트 설정을 제공합니다. 첫 렌더러의 범위와 완료 기준은 아래 문서에 있습니다.

| 문서 | 내용 |
|---|---|
| [제품 범위](docs/00-overview.md) | 사용자, 첫 장면, 다른 레포와의 경계 |
| [사용 API](docs/01-dx-and-dsl.md) | 현재 설정 API와 프레임 호출 순서 |
| [아키텍처](docs/02-architecture.md) | 리소스 소유권, GPU 의존성과 CPU 경로 |
| [호스트 어댑터](docs/03-adapters-and-pipeline.md) | 네이티브 합성, 웹 캔버스, 입력 |
| [결정 기록](docs/04-open-questions.md) | 제품 범위와 선정 근거 |
| [구현 계획](docs/05-implementation-plan.md) | 구현 상태, 단계별 인수 기준 |

## 라이선스

Licensed under the [MIT License](LICENSE).
