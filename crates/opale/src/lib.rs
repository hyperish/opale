//! `opale` — High-performance cross-platform graphics framework for the Hyperish family.
//!
//! This initial pre-release establishes core graphics primitives for viewport,
//! linear RGBA color, and backend descriptors.

/// Target graphics backend API supported by `opale`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicsBackend {
    /// Apple Metal (macOS / iOS).
    Metal,
    /// Khronos Vulkan (Windows / Linux / Android).
    Vulkan,
    /// DirectX 12 (Windows).
    Dx12,
    /// WebGPU (Browser / Cross-platform).
    WebGpu,
    /// Software CPU fallback renderer.
    Cpu,
}

/// Linear-space RGBA color with normalized floating-point channels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red channel (`0.0..=1.0` in SDR, unbounded in HDR).
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel (`0.0..=1.0`).
    pub a: f32,
}

impl Color {
    /// Opaque black (`0.0, 0.0, 0.0, 1.0`).
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);

    /// Creates a linear RGBA color.
    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

/// Two-dimensional pixel extent for surfaces and viewports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extent2D {
    /// Width in physical pixels.
    pub width: u32,
    /// Height in physical pixels.
    pub height: u32,
}

/// Configuration descriptor for an `opale` rendering viewport.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportDesc {
    extent: Extent2D,
    backend: GraphicsBackend,
    clear_color: Color,
}

impl ViewportDesc {
    /// Creates a viewport descriptor with the given pixel dimensions.
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            extent: Extent2D { width, height },
            backend: GraphicsBackend::WebGpu,
            clear_color: Color::BLACK,
        }
    }

    /// Selects the preferred [`GraphicsBackend`].
    #[must_use]
    pub const fn with_backend(mut self, backend: GraphicsBackend) -> Self {
        self.backend = backend;
        self
    }

    /// Sets the viewport clear color.
    #[must_use]
    pub const fn with_clear_color(mut self, color: Color) -> Self {
        self.clear_color = color;
        self
    }

    /// Returns the viewport dimensions.
    #[must_use]
    pub const fn extent(&self) -> Extent2D {
        self.extent
    }

    /// Returns the selected graphics backend.
    #[must_use]
    pub const fn backend(&self) -> GraphicsBackend {
        self.backend
    }

    /// Returns the clear color.
    #[must_use]
    pub const fn clear_color(&self) -> Color {
        self.clear_color
    }
}

/// Commonly used types re-exported for ergonomic imports.
pub mod prelude {
    pub use crate::{Color, Extent2D, GraphicsBackend, ViewportDesc};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn viewport_builder_sets_extent_backend_and_clear_color() {
        let vp = ViewportDesc::new(1920, 1080)
            .with_backend(GraphicsBackend::Metal)
            .with_clear_color(Color::rgba(0.1, 0.2, 0.3, 1.0));

        assert_eq!(
            vp.extent(),
            Extent2D {
                width: 1920,
                height: 1080
            }
        );
        assert_eq!(vp.backend(), GraphicsBackend::Metal);
        assert_eq!(vp.clear_color(), Color::rgba(0.1, 0.2, 0.3, 1.0));
    }
}
