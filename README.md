# hyperish/opale

하이퍼리쉬(Hyperish) 프레임워크 패밀리의 **고성능 크로스플랫폼 그래픽 프레임워크** 레포입니다.

```rust
use opale::prelude::*;

let viewport = ViewportDesc::new(1920, 1080)
    .with_backend(GraphicsBackend::Metal)
    .with_clear_color(Color::rgba(0.05, 0.06, 0.08, 1.0));
```

상위 범용 명세와 문서 상태 규칙(`확정` / `제안` / `미정`)은 [`hyperish/docs`](https://github.com/hyperish/docs)를 따릅니다.

## 라이선스

Licensed under the [MIT License](LICENSE).
