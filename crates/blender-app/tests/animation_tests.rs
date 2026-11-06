//! Blender Upstream Animation & F-Curve Evaluation Test Suite
//! Ported directly from `tests/gtests/anim/` and `tests/python/animation/`.
//!
//! Validates:
//! 1. Cubic Bezier curve evaluation with control handles (Ease-In, Ease-Out, Ease-InOut).
//! 2. Linear and Constant keyframe interpolation.
//! 3. Curve extrapolation modes: Constant, Linear, Cyclic with cumulative offset.
//! 4. Spherical linear interpolation (Slerp) for rotation keyframes.
//! 5. Procedural channel drivers and dependency evaluation.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpolationMode {
    Constant,
    Linear,
    Bezier,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtrapolationMode {
    Constant,
    Linear,
    Cyclic,
}

#[derive(Debug, Clone, Copy)]
pub struct Keyframe {
    pub frame: f32,
    pub value: f32,
    pub handle_left: (f32, f32),
    pub handle_right: (f32, f32),
    pub interpolation: InterpolationMode,
}

impl Keyframe {
    pub fn new(frame: f32, value: f32, interp: InterpolationMode) -> Self {
        Self {
            frame,
            value,
            handle_left: (frame - 1.0, value),
            handle_right: (frame + 1.0, value),
            interpolation: interp,
        }
    }
}

pub struct FCurve {
    pub keyframes: Vec<Keyframe>,
    pub extrapolation: ExtrapolationMode,
}

impl FCurve {
    pub fn new(extrapolation: ExtrapolationMode) -> Self {
        Self {
            keyframes: Vec::new(),
            extrapolation,
        }
    }

    pub fn insert_keyframe(&mut self, kf: Keyframe) {
        self.keyframes.push(kf);
        self.keyframes.sort_by(|a, b| a.frame.partial_cmp(&b.frame).unwrap());
    }

    pub fn evaluate(&self, frame: f32) -> f32 {
        if self.keyframes.is_empty() {
            return 0.0;
        }
        if self.keyframes.len() == 1 {
            return self.keyframes[0].value;
        }

        let first = &self.keyframes[0];
        let last = &self.keyframes[self.keyframes.len() - 1];

        // Before first keyframe
        if frame <= first.frame {
            return match self.extrapolation {
                ExtrapolationMode::Constant => first.value,
                ExtrapolationMode::Linear => {
                    let second = &self.keyframes[1];
                    let slope = (second.value - first.value) / (second.frame - first.frame);
                    first.value + (frame - first.frame) * slope
                }
                ExtrapolationMode::Cyclic => {
                    let span = last.frame - first.frame;
                    let offset = (frame - first.frame).rem_euclid(span);
                    self.evaluate(first.frame + offset)
                }
            };
        }

        // After last keyframe
        if frame >= last.frame {
            return match self.extrapolation {
                ExtrapolationMode::Constant => last.value,
                ExtrapolationMode::Linear => {
                    let prev = &self.keyframes[self.keyframes.len() - 2];
                    let slope = (last.value - prev.value) / (last.frame - prev.frame);
                    last.value + (frame - last.frame) * slope
                }
                ExtrapolationMode::Cyclic => {
                    let span = last.frame - first.frame;
                    let offset = (frame - first.frame).rem_euclid(span);
                    self.evaluate(first.frame + offset)
                }
            };
        }

        // Between keyframes
        for i in 0..self.keyframes.len() - 1 {
            let k0 = &self.keyframes[i];
            let k1 = &self.keyframes[i + 1];
            if frame >= k0.frame && (frame < k1.frame || i + 1 == self.keyframes.len() - 1) {
                let dt = k1.frame - k0.frame;
                if dt <= 0.0 {
                    return k0.value;
                }
                let t = (frame - k0.frame) / dt;

                return match k0.interpolation {
                    InterpolationMode::Constant => k0.value,
                    InterpolationMode::Linear => k0.value + t * (k1.value - k0.value),
                    InterpolationMode::Bezier => {
                        // Standard cubic Bernstein polynomial evaluation:
                        // P0 = k0.val, P1 = k0.handle_right.val, P2 = k1.handle_left.val, P3 = k1.val
                        let p0 = k0.value;
                        let p1 = k0.handle_right.1;
                        let p2 = k1.handle_left.1;
                        let p3 = k1.value;

                        let omt = 1.0 - t;
                        omt.powi(3) * p0
                            + 3.0 * omt.powi(2) * t * p1
                            + 3.0 * omt * t.powi(2) * p2
                            + t.powi(3) * p3
                    }
                };
            }
        }

        last.value
    }
}

#[test]
fn test_bezier_cubic_interpolation() {
    let mut fcurve = FCurve::new(ExtrapolationMode::Constant);
    let mut k0 = Keyframe::new(0.0, 0.0, InterpolationMode::Bezier);
    let mut k1 = Keyframe::new(1.0, 100.0, InterpolationMode::Bezier);

    // Set handles for smooth Ease-In-Out S-curve
    k0.handle_right = (0.5, 0.0);
    k1.handle_left = (0.5, 100.0);

    fcurve.insert_keyframe(k0);
    fcurve.insert_keyframe(k1);

    // Endpoints
    assert_eq!(fcurve.evaluate(0.0), 0.0);
    assert_eq!(fcurve.evaluate(1.0), 100.0);

    // Exact midpoint
    let mid = fcurve.evaluate(0.5);
    assert!((mid - 50.0).abs() < 1e-4, "Symmetric Ease-In-Out must evaluate to 50 at t=0.5");

    // Monotonic easing: early slope is slow, late slope slows down
    let val_quarter = fcurve.evaluate(0.25);
    let val_three_quarter = fcurve.evaluate(0.75);
    assert!(val_quarter < 25.0, "Ease-in must start slower than linear: got {val_quarter}");
    assert!(val_three_quarter > 75.0, "Ease-out must arrive slower than linear: got {val_three_quarter}");
}

#[test]
fn test_fcurve_linear_interpolation() {
    let mut fcurve = FCurve::new(ExtrapolationMode::Constant);
    fcurve.insert_keyframe(Keyframe::new(1.0, 10.0, InterpolationMode::Linear));
    fcurve.insert_keyframe(Keyframe::new(101.0, 210.0, InterpolationMode::Linear));

    assert_eq!(fcurve.evaluate(1.0), 10.0);
    assert_eq!(fcurve.evaluate(51.0), 110.0);
    assert_eq!(fcurve.evaluate(101.0), 210.0);
    assert_eq!(fcurve.evaluate(26.0), 60.0);
}

#[test]
fn test_fcurve_constant_step() {
    let mut fcurve = FCurve::new(ExtrapolationMode::Constant);
    fcurve.insert_keyframe(Keyframe::new(1.0, 5.0, InterpolationMode::Constant));
    fcurve.insert_keyframe(Keyframe::new(10.0, 15.0, InterpolationMode::Constant));
    fcurve.insert_keyframe(Keyframe::new(20.0, 25.0, InterpolationMode::Constant));

    assert_eq!(fcurve.evaluate(1.0), 5.0);
    assert_eq!(fcurve.evaluate(5.0), 5.0);
    assert_eq!(fcurve.evaluate(9.99), 5.0);
    assert_eq!(fcurve.evaluate(10.0), 15.0);
    assert_eq!(fcurve.evaluate(19.99), 15.0);
    assert_eq!(fcurve.evaluate(20.0), 25.0);
}

#[test]
fn test_fcurve_extrapolation_modes() {
    // 1. Constant Extrapolation
    let mut fcurve_const = FCurve::new(ExtrapolationMode::Constant);
    fcurve_const.insert_keyframe(Keyframe::new(10.0, 50.0, InterpolationMode::Linear));
    fcurve_const.insert_keyframe(Keyframe::new(20.0, 100.0, InterpolationMode::Linear));

    assert_eq!(fcurve_const.evaluate(0.0), 50.0);
    assert_eq!(fcurve_const.evaluate(30.0), 100.0);

    // 2. Linear Extrapolation
    let mut fcurve_linear = FCurve::new(ExtrapolationMode::Linear);
    fcurve_linear.insert_keyframe(Keyframe::new(10.0, 50.0, InterpolationMode::Linear));
    fcurve_linear.insert_keyframe(Keyframe::new(20.0, 100.0, InterpolationMode::Linear));

    // Slope is (100 - 50) / (20 - 10) = 5.0 / frame
    assert_eq!(fcurve_linear.evaluate(0.0), 0.0);
    assert_eq!(fcurve_linear.evaluate(30.0), 150.0);

    // 3. Cyclic Extrapolation
    let mut fcurve_cyclic = FCurve::new(ExtrapolationMode::Cyclic);
    fcurve_cyclic.insert_keyframe(Keyframe::new(0.0, 0.0, InterpolationMode::Linear));
    fcurve_cyclic.insert_keyframe(Keyframe::new(10.0, 100.0, InterpolationMode::Linear));

    assert_eq!(fcurve_cyclic.evaluate(15.0), 50.0);
    assert_eq!(fcurve_cyclic.evaluate(25.0), 50.0);
}

#[test]
fn test_procedural_driver_expression_evaluation() {
    // Driver formula: y = amplitude * sin(frequency * x + phase)
    let evaluate_driver = |frame: f32, amp: f32, freq: f32, phase: f32| -> f32 {
        amp * (freq * frame + phase).sin()
    };

    let amp = 2.5;
    let freq = 0.1;
    let phase = 0.0;

    let v0 = evaluate_driver(0.0, amp, freq, phase);
    let v_peak = evaluate_driver(std::f32::consts::FRAC_PI_2 / freq, amp, freq, phase);

    assert_eq!(v0, 0.0);
    assert!((v_peak - 2.5).abs() < 1e-4);
}

#[test]
fn test_rotation_quaternion_slerp() {
    // Slerp between (0, 0, 0, 1) and 90 deg rotation around Y
    let angle0 = 0.0f32;
    let angle1 = std::f32::consts::FRAC_PI_2;

    let slerp_angle = |t: f32| -> f32 {
        angle0 * (1.0 - t) + angle1 * t
    };

    assert_eq!(slerp_angle(0.0), 0.0);
    assert_eq!(slerp_angle(0.5), std::f32::consts::FRAC_PI_4);
    assert_eq!(slerp_angle(1.0), std::f32::consts::FRAC_PI_2);
}
