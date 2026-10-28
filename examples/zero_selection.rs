//! Signed-zero selection: plain Rust, glam, then positive-zero controls
//! Keep the comparison bounds constant so the optimizer can see them
//! Matching symptoms do not establish one compiler root cause

use glam::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec3A, Vec4};
use glam_determinism_repros::{show, show_f64, workarounds::{
    FloatSafeExt, VectorFloatSafeExt, canonicalize_f32_zero, canonicalize_f64_zero,
}};
use std::hint::black_box;

macro_rules! scalar {
    ($ty:ty, $show:ident, $canonicalize:ident) => {{
        let n = black_box(-0.0 as $ty);
        let p = black_box(0.0 as $ty);
        let selected = if n < 0.0 { n } else { 0.0 };
        for (name, raw, safe) in [
            ("select", selected, $canonicalize(selected)),
            ("min", n.min(0.0), n.min_safe(0.0)),
            ("max", p.max(-0.0), p.max_safe(-0.0)),
            ("clamp", n.clamp(-1.0, 0.0), n.clamp_safe(-1.0, 0.0)),
        ] {
            $show(&format!("rust_{}_{name}_raw", stringify!($ty)), raw);
            $show(&format!("rust_{}_{name}_safe", stringify!($ty)), safe);
            assert_eq!(safe.to_bits(), 0, "{} {name} must return positive zero", stringify!($ty));
        }
    }};
}

macro_rules! vector {
    ($ty:ty, $scalar:ty, $show:ident, $canonicalize:ident) => {{
        // A separate function preserves the reduction witness in this combined example
        #[inline(never)]
        fn min_with_zero(x: $scalar) -> $scalar {
            let mut v = <$ty>::ZERO;
            v.x = x;
            v.min_element()
        }

        let n = black_box(<$ty>::splat(-0.0));
        let p = black_box(<$ty>::ZERO);
        let mut min_input = <$ty>::ZERO;
        min_input.x = black_box(-0.0);
        let mut max_input = <$ty>::splat(-0.0);
        max_input.x = black_box(0.0);
        let selected = <$ty>::select(n.cmplt(<$ty>::ZERO), n, <$ty>::ZERO).x;
        let smooth = n.smoothstep(<$ty>::ZERO, <$ty>::ONE).x;
        for (name, raw, safe) in [
            ("min", n.min(<$ty>::ZERO).x, n.min_safe(<$ty>::ZERO).x),
            ("max", p.max(<$ty>::splat(-0.0)).x, p.max_safe(<$ty>::splat(-0.0)).x),
            ("clamp_upper", n.clamp(-<$ty>::ONE, <$ty>::ZERO).x, n.clamp_safe(-<$ty>::ONE, <$ty>::ZERO).x),
            ("clamp_lower", p.clamp(<$ty>::splat(-0.0), <$ty>::ONE).x, p.clamp_safe(<$ty>::splat(-0.0), <$ty>::ONE).x),
            ("min_element", min_with_zero(black_box(-0.0)), min_input.min_element_safe()),
            ("max_element", max_input.max_element(), max_input.max_element_safe()),
            ("select", selected, $canonicalize(selected)),
            ("saturate", n.saturate().x, n.clamp_safe(<$ty>::ZERO, <$ty>::ONE).x),
            ("smoothstep", smooth, $canonicalize(smooth)),
        ] {
            $show(&format!("{}_{name}_raw", stringify!($ty)), raw);
            $show(&format!("{}_{name}_safe", stringify!($ty)), safe);
            assert_eq!(safe.to_bits(), 0, "{} {name} must return positive zero", stringify!($ty));
        }

        let finite = black_box(<$ty>::splat(-3.5));
        assert_eq!(finite.min_safe(<$ty>::ONE), finite, "{} must retain nonzero minima", stringify!($ty));
        assert_eq!(finite.clamp_safe(-<$ty>::ONE, <$ty>::ONE), -<$ty>::ONE, "{} must retain nonzero clamp results", stringify!($ty));
    }};
}

fn main() {
    scalar!(f32, show, canonicalize_f32_zero);
    scalar!(f64, show_f64, canonicalize_f64_zero);

    vector!(Vec2, f32, show, canonicalize_f32_zero);
    vector!(Vec3, f32, show, canonicalize_f32_zero);
    vector!(Vec3A, f32, show, canonicalize_f32_zero);
    vector!(Vec4, f32, show, canonicalize_f32_zero);
    vector!(DVec2, f64, show_f64, canonicalize_f64_zero);
    vector!(DVec3, f64, show_f64, canonicalize_f64_zero);
    vector!(DVec4, f64, show_f64, canonicalize_f64_zero);
}
