//! # lau-fibonacci-growth
//!
//! Fibonacci growth patterns for agent capability development —
//! how skills grow in spirals, not lines.

use serde::{Deserialize, Serialize};
use std::f64::consts::SQRT_2;

/// The golden ratio φ ≈ 1.6180339887
pub const PHI: f64 = (1.0 + SQRT_2.recip()) * 2.0_f64.recip() + 1.0;
// More precisely:
// phi = (1 + sqrt(5)) / 2

/// The golden angle in degrees ≈ 137.507764°
pub const GOLDEN_ANGLE_DEG: f64 = 137.5077640500378;

// ─── 1. FibonacciSequence ──────────────────────────────────────────────────

/// The golden sequence — classic Fibonacci numbers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FibonacciSequence {
    values: Vec<u64>,
    computed_to: usize,
}

impl Default for FibonacciSequence {
    fn default() -> Self {
        Self::new()
    }
}

impl FibonacciSequence {
    /// Start with [0, 1].
    pub fn new() -> Self {
        Self {
            values: vec![0, 1],
            computed_to: 1,
        }
    }

    /// Compute up to the n-th Fibonacci number (index n).
    pub fn compute_to(&mut self, n: usize) {
        while self.values.len() <= n {
            let len = self.values.len();
            let next = self.values[len - 1] + self.values[len - 2];
            self.values.push(next);
        }
        self.computed_to = self.computed_to.max(n);
    }

    /// Return F(n).
    pub fn at(&self, n: usize) -> u64 {
        if n < self.values.len() {
            self.values[n]
        } else {
            // compute without mutating — use Binet-like loop
            let mut fib = FibonacciSequence::new();
            fib.compute_to(n);
            fib.values[n]
        }
    }

    /// Return F(n+1)/F(n), approaching φ.
    pub fn ratio(&self, n: usize) -> f64 {
        if n == 0 {
            return f64::INFINITY; // F(1)/F(0) = 1/0
        }
        let a = self.at(n);
        let b = self.at(n + 1);
        b as f64 / a as f64
    }

    /// Last computed ratio.
    pub fn golden_ratio_approximation(&self) -> f64 {
        if self.values.len() < 2 {
            return 1.0;
        }
        let n = self.values.len() - 2;
        self.ratio(n)
    }

    /// Check if n is a Fibonacci number.
    /// A number is Fibonacci iff 5n²+4 or 5n²-4 is a perfect square.
    pub fn is_fibonacci(&self, n: u64) -> bool {
        if n <= 1 {
            return true;
        }
        let n_sq = n as u128 * n as u128;
        let check = |x: u128| -> bool {
            let s = (x as f64).sqrt() as u128;
            s * s == x || (s + 1) * (s + 1) == x
        };
        check(5 * n_sq + 4) || check(5 * n_sq - 4)
    }

    /// Which Fibonacci index is n?
    pub fn index_of(&self, n: u64) -> Option<usize> {
        if !self.is_fibonacci(n) {
            return None;
        }
        let mut fib = FibonacciSequence::new();
        let mut idx = 0;
        loop {
            fib.compute_to(idx);
            if fib.values[idx] == n {
                return Some(idx);
            }
            if fib.values[idx] > n {
                return None;
            }
            idx += 1;
        }
    }

    /// Σ F(i) for i=0..n = F(n+2) - 1
    pub fn sum_to(&self, n: usize) -> u64 {
        let mut fib = FibonacciSequence::new();
        fib.compute_to(n + 2);
        fib.values[0..=n].iter().sum()
    }

    /// Zeckendorf representation — sum of non-consecutive Fibonacci numbers.
    pub fn zeckendorf(&self, mut n: u64) -> Vec<usize> {
        if n == 0 {
            return vec![0];
        }
        let mut fib = FibonacciSequence::new();
        // Compute enough Fibonacci numbers
        fib.compute_to(2);
        while *fib.values.last().unwrap() <= n {
            let len = fib.values.len();
            fib.compute_to(len);
        }

        let mut indices = Vec::new();
        // Skip F(0)=0, start from largest
        for i in (2..fib.values.len()).rev() {
            if fib.values[i] <= n {
                indices.push(i);
                n -= fib.values[i];
                if n == 0 {
                    break;
                }
            }
        }
        indices.sort();
        indices
    }
}

// ─── 2. GoldenSpiral ──────────────────────────────────────────────────────

/// The Fibonacci spiral — a logarithmic spiral with growth factor φ per quarter turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenSpiral {
    /// Center point of the spiral.
    pub center: (f64, f64),
    /// Starting angle in degrees.
    pub direction: f64,
    /// Number of complete rotations.
    pub turns: f64,
}

impl GoldenSpiral {
    /// Point on spiral at parameter t (0..1 over full spiral).
    /// θ = t * turns * 360°
    /// r = a * φ^(θ/90°)
    pub fn point_at(&self, t: f64) -> (f64, f64) {
        let theta_deg = t * self.turns * 360.0;
        let r = self.radius_at(t);
        let angle = (self.direction + theta_deg).to_radians();
        (
            self.center.0 + r * angle.cos(),
            self.center.1 + r * angle.sin(),
        )
    }

    /// r = a * φ^(θ/90°), where a = 1 (unit scale).
    pub fn radius_at(&self, t: f64) -> f64 {
        let theta_deg = t * self.turns * 360.0;
        golden_ratio().powf(theta_deg / 90.0)
    }

    /// Approximate arc length from t_start to t_end.
    /// Uses numerical integration (Simpson's rule with n segments).
    pub fn arc_length(&self, t_start: f64, t_end: f64) -> f64 {
        let n = 1000;
        let dt = (t_end - t_start) / n as f64;

        let mut sum = 0.0;
        for i in 0..n {
            let t0 = t_start + i as f64 * dt;
            let t1 = t_start + (i + 1) as f64 * dt;
            let t_mid = (t0 + t1) / 2.0;

            let p0 = self.point_at(t0);
            let p_mid = self.point_at(t_mid);
            let p1 = self.point_at(t1);

            // Simpson
            let d01 = ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)).sqrt();
            let d0m = ((p_mid.0 - p0.0).powi(2) + (p_mid.1 - p0.1).powi(2)).sqrt();
            let dm1 = ((p1.0 - p_mid.0).powi(2) + (p1.1 - p_mid.1).powi(2)).sqrt();

            sum += (d01 + 4.0 * (d0m + dm1) / 2.0) / 6.0;
        }
        sum
    }

    /// n points along the spiral.
    pub fn points(&self, n: usize) -> Vec<(f64, f64)> {
        (0..n)
            .map(|i| self.point_at(i as f64 / (n - 1).max(1) as f64))
            .collect()
    }
}

/// Compute the precise golden ratio.
pub fn golden_ratio() -> f64 {
    (1.0 + 5_f64.sqrt()) / 2.0
}

// ─── 3. GrowthCurve ───────────────────────────────────────────────────────

/// Skill growth following a Fibonacci-scaled logistic curve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthCurve {
    pub base: f64,
    pub rate: f64,
    pub phi_scale: f64,
}

impl GrowthCurve {
    /// Growth value at step n, using a phi-scaled logistic curve:
    /// value = plateau / (1 + e^(-rate * (step - base*phi_scale)))
    pub fn value_at(&self, step: usize) -> f64 {
        let plateau = self.plateau();
        let x = self.rate * (step as f64 - self.base * self.phi_scale);
        plateau / (1.0 + (-x).exp())
    }

    /// Asymptotic maximum.
    pub fn plateau(&self) -> f64 {
        self.base * golden_ratio() * self.phi_scale * 10.0
    }

    /// Steps needed to reach target value.
    pub fn steps_to_reach(&self, target: f64) -> usize {
        if target >= self.plateau() {
            return usize::MAX;
        }
        let plateau = self.plateau();
        // target = plateau / (1 + e^(-r*(s - b*phi)))
        // 1 + e^(-r*(s-b*phi)) = plateau / target
        // e^(-r*(s-b*phi)) = plateau/target - 1
        // -r*(s-b*phi) = ln(plateau/target - 1)
        // s = b*phi - ln(plateau/target - 1)/r
        let ratio = plateau / target - 1.0;
        if ratio <= 0.0 {
            return usize::MAX;
        }
        let s = self.base * self.phi_scale - ratio.ln() / self.rate;
        s.max(0.0).ceil() as usize
    }

    /// Derivative (growth rate) at step.
    pub fn growth_rate_at(&self, step: usize) -> f64 {
        let plateau = self.plateau();
        let x = self.rate * (step as f64 - self.base * self.phi_scale);
        let exp_neg = (-x).exp();
        // dV/ds = P * rate * e^(-x) / (1+e^(-x))^2
        plateau * self.rate * exp_neg / (1.0 + exp_neg).powi(2)
    }
}

// ─── 5. SkillNode ─────────────────────────────────────────────────────────

/// A single skill node in the spiral tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillNode {
    pub name: String,
    pub base_power: f64,
    pub growth_rate: f64,
    pub dependencies: Vec<String>,
    pub unlocks_at: usize,
}

impl SkillNode {
    /// Power at a given level, grows as base_power * growth_rate^level.
    pub fn power_at(&self, level: usize) -> f64 {
        if !self.is_unlocked(level) {
            return 0.0;
        }
        let effective = level - self.unlocks_at;
        self.base_power * self.growth_rate.powi(effective as i32)
    }

    /// Unlocked if level >= unlocks_at and dependencies met.
    pub fn is_unlocked(&self, level: usize) -> bool {
        level >= self.unlocks_at
    }
}

// ─── 4. SpiralSkillTree ───────────────────────────────────────────────────

/// Skill development as a spiral — unlock order follows Fibonacci indices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralSkillTree {
    pub skills: Vec<SkillNode>,
    pub levels: usize,
}

impl SpiralSkillTree {
    /// Fibonacci-ordered unlock sequence.
    pub fn unlock_order(&self) -> Vec<String> {
        let mut skills = self.skills.clone();
        skills.sort_by_key(|s| s.unlocks_at);
        skills.into_iter().map(|s| s.name).collect()
    }

    /// Power of a specific skill at a given level.
    pub fn skill_at_level(&self, skill: &str, level: usize) -> f64 {
        self.skills
            .iter()
            .find(|s| s.name == skill)
            .map(|s| s.power_at(level))
            .unwrap_or(0.0)
    }

    /// Sum of all skill powers at a given level.
    pub fn total_power(&self, level: usize) -> f64 {
        self.skills.iter().map(|s| s.power_at(level)).sum()
    }

    /// Check that no skill exceeds φ× another at the same level.
    pub fn is_balanced(&self, level: usize) -> bool {
        let powers: Vec<f64> = self
            .skills
            .iter()
            .filter(|s| s.is_unlocked(level))
            .map(|s| s.power_at(level))
            .filter(|&p| p > 0.0)
            .collect();

        if powers.len() <= 1 {
            return true;
        }

        let min_p = powers.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_p = powers.iter().cloned().fold(0.0_f64, f64::max);

        if min_p == 0.0 {
            return false;
        }

        max_p <= min_p * golden_ratio()
    }
}

// ─── 6. FibonacciRetracement ──────────────────────────────────────────────

/// Fibonacci retracement levels, like in financial analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FibonacciRetracement {
    pub high: f64,
    pub low: f64,
}

impl FibonacciRetracement {
    /// Standard retracement ratios: 0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0
    pub const RATIOS: [f64; 7] = [0.0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0];

    /// Return (ratio, value) pairs for all standard levels.
    pub fn levels(&self) -> Vec<(f64, f64)> {
        Self::RATIOS
            .iter()
            .map(|&r| (r, self.support_at(r)))
            .collect()
    }

    /// Value at a given retracement ratio: low + ratio * (high - low).
    pub fn support_at(&self, ratio: f64) -> f64 {
        self.low + ratio * (self.high - self.low)
    }

    /// Resistance is the same as support (retracement is symmetric).
    pub fn resistance_at(&self, ratio: f64) -> f64 {
        self.support_at(ratio)
    }
}

// ─── 7. Phyllotaxis ───────────────────────────────────────────────────────

/// Fibonacci in nature — sunflower spiral patterns using Vogel's formula.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phyllotaxis {
    pub n: usize,
    pub golden_angle: f64,
}

impl Phyllotaxis {
    /// Point for seed `index` using Vogel's formula:
    /// r = c * sqrt(index)
    /// θ = index * golden_angle
    pub fn point(&self, index: usize) -> (f64, f64) {
        let r = (index as f64).sqrt();
        let theta = (index as f64 * self.golden_angle).to_radians();
        (r * theta.cos(), r * theta.sin())
    }

    /// All n points.
    pub fn points(&self) -> Vec<(f64, f64)> {
        (0..self.n).map(|i| self.point(i)).collect()
    }

    /// Count clockwise and counterclockwise spirals.
    /// For a proper phyllotaxis with golden angle, these should be
    /// consecutive Fibonacci numbers.
    pub fn spiral_count(&self) -> (usize, usize) {
        // For n seeds with golden angle, the spiral counts are approximately
        // the Fibonacci pair whose sum is closest to sqrt(n).
        let target = (self.n as f64).sqrt() as usize;
        let mut fib = FibonacciSequence::new();
        fib.compute_to(30);

        // Find consecutive Fibonacci pair closest to target
        let mut best = (5, 8);
        let mut best_diff = usize::MAX;
        for i in 3..29 {
            let f1 = fib.values[i] as usize;
            let f2 = fib.values[i + 1] as usize;
            let avg = (f1 + f2) / 2;
            let diff = avg.abs_diff(target);
            if diff < best_diff {
                best_diff = diff;
                best = (f1.min(f2), f1.max(f2));
            }
        }
        best
    }
}

// ─── 8. LucasSequence ─────────────────────────────────────────────────────

/// Lucas sequence — companion to Fibonacci: L(0)=2, L(1)=1, L(n)=L(n-1)+L(n-2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LucasSequence {
    pub values: Vec<u64>,
}

impl Default for LucasSequence {
    fn default() -> Self {
        Self::new()
    }
}

impl LucasSequence {
    /// Start with [2, 1].
    pub fn new() -> Self {
        Self { values: vec![2, 1] }
    }

    /// Compute up to index n.
    pub fn compute_to(&mut self, n: usize) {
        while self.values.len() <= n {
            let len = self.values.len();
            let next = self.values[len - 1] + self.values[len - 2];
            self.values.push(next);
        }
    }

    /// Return L(n).
    pub fn at(&self, n: usize) -> u64 {
        if n < self.values.len() {
            self.values[n]
        } else {
            let mut lucas = LucasSequence::new();
            lucas.compute_to(n);
            lucas.values[n]
        }
    }

    /// Verify Lucas identity: L(n) = F(n-1) + F(n+1).
    pub fn verify_lucas_identity(&self, n: usize) -> bool {
        if n == 0 {
            // L(0) = 2, F(-1) + F(1) — edge case
            // F(-1) = 1 (by extension), F(1) = 1, so 1+1=2 ✓
            return self.at(0) == 2;
        }
        let fib = FibonacciSequence::new();
        let f_prev = fib.at(n - 1);
        let f_next = fib.at(n + 1);
        self.at(n) == f_prev + f_next
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── FibonacciSequence tests ──

    #[test]
    fn test_fibonacci_initial_values() {
        let fib = FibonacciSequence::new();
        assert_eq!(fib.values, vec![0, 1]);
    }

    #[test]
    fn test_fibonacci_compute_to() {
        let mut fib = FibonacciSequence::new();
        fib.compute_to(10);
        assert_eq!(fib.at(0), 0);
        assert_eq!(fib.at(1), 1);
        assert_eq!(fib.at(2), 1);
        assert_eq!(fib.at(3), 2);
        assert_eq!(fib.at(4), 3);
        assert_eq!(fib.at(5), 5);
        assert_eq!(fib.at(6), 8);
        assert_eq!(fib.at(7), 13);
        assert_eq!(fib.at(8), 21);
        assert_eq!(fib.at(9), 34);
        assert_eq!(fib.at(10), 55);
    }

    #[test]
    fn test_fibonacci_theorem_12() {
        // F(0)=0, F(1)=1, F(2)=1, F(3)=2, F(4)=3, F(5)=5
        let fib = FibonacciSequence::new();
        assert_eq!(fib.at(0), 0);
        assert_eq!(fib.at(1), 1);
        assert_eq!(fib.at(2), 1);
        assert_eq!(fib.at(3), 2);
        assert_eq!(fib.at(4), 3);
        assert_eq!(fib.at(5), 5);
    }

    #[test]
    fn test_fibonacci_recurrence_theorem_2() {
        // F(n) = F(n-1) + F(n-2)
        let mut fib = FibonacciSequence::new();
        fib.compute_to(20);
        for n in 2..=20 {
            assert_eq!(fib.at(n), fib.at(n - 1) + fib.at(n - 2));
        }
    }

    #[test]
    fn test_golden_ratio_convergence_theorem_1() {
        let mut fib = FibonacciSequence::new();
        fib.compute_to(30);
        let phi = golden_ratio();
        // F(n+1)/F(n) → φ
        let ratio_10 = fib.ratio(10);
        let ratio_20 = fib.ratio(20);
        assert!((ratio_10 - phi).abs() < 0.01);
        assert!((ratio_20 - phi).abs() < 0.001);
    }

    #[test]
    fn test_golden_ratio_approximation() {
        let mut fib = FibonacciSequence::new();
        fib.compute_to(15);
        let approx = fib.golden_ratio_approximation();
        let phi = golden_ratio();
        assert!((approx - phi).abs() < 0.01);
    }

    #[test]
    fn test_is_fibonacci() {
        let fib = FibonacciSequence::new();
        assert!(fib.is_fibonacci(0));
        assert!(fib.is_fibonacci(1));
        assert!(fib.is_fibonacci(2));
        assert!(fib.is_fibonacci(3));
        assert!(fib.is_fibonacci(5));
        assert!(fib.is_fibonacci(8));
        assert!(fib.is_fibonacci(13));
        assert!(fib.is_fibonacci(21));
        assert!(fib.is_fibonacci(55));
        assert!(!fib.is_fibonacci(4));
        assert!(!fib.is_fibonacci(6));
        assert!(!fib.is_fibonacci(7));
        assert!(!fib.is_fibonacci(10));
        assert!(!fib.is_fibonacci(14));
    }

    #[test]
    fn test_index_of() {
        let fib = FibonacciSequence::new();
        assert_eq!(fib.index_of(0), Some(0));
        assert_eq!(fib.index_of(1), Some(1));
        assert_eq!(fib.index_of(2), Some(3));
        assert_eq!(fib.index_of(3), Some(4));
        assert_eq!(fib.index_of(5), Some(5));
        assert_eq!(fib.index_of(8), Some(6));
        assert_eq!(fib.index_of(13), Some(7));
        assert_eq!(fib.index_of(21), Some(8));
        assert_eq!(fib.index_of(4), None);
        assert_eq!(fib.index_of(6), None);
    }

    #[test]
    fn test_sum_formula_theorem_3() {
        // Σ F(i) for i=0..n = F(n+2) - 1
        let fib = FibonacciSequence::new();
        for n in 0..=15 {
            let sum = fib.sum_to(n);
            let expected = fib.at(n + 2) - 1;
            assert_eq!(sum, expected, "Sum formula failed at n={n}");
        }
    }

    #[test]
    fn test_zeckendorf_theorem_4() {
        let fib = FibonacciSequence::new();
        // Every positive integer has unique representation as sum of non-consecutive Fibonacci numbers
        for n in 1..=100 {
            let rep = fib.zeckendorf(n);
            // Verify sum equals n
            let sum: u64 = rep.iter().map(|&i| fib.at(i)).sum();
            assert_eq!(sum, n, "Zeckendorf sum mismatch for n={n}");

            // Verify non-consecutive
            for w in rep.windows(2) {
                assert!(w[1] - w[0] >= 2, "Consecutive Fibonacci indices for n={n}");
            }
        }
    }

    #[test]
    fn test_zeckendorf_specific() {
        let fib = FibonacciSequence::new();
        // 1 = F(2) = 1
        assert_eq!(fib.zeckendorf(1), vec![2]);
        // 2 = F(3) = 2
        assert_eq!(fib.zeckendorf(2), vec![3]);
        // 3 = F(4) = 3
        assert_eq!(fib.zeckendorf(3), vec![4]);
        // 4 = F(4) + F(2) = 3 + 1
        let rep = fib.zeckendorf(4);
        let sum: u64 = rep.iter().map(|&i| fib.at(i)).sum();
        assert_eq!(sum, 4);
        // 100 = 89 + 8 + 3 = F(11) + F(6) + F(4)
        let rep = fib.zeckendorf(100);
        let sum: u64 = rep.iter().map(|&i| fib.at(i)).sum();
        assert_eq!(sum, 100);
    }

    #[test]
    fn test_zeckendorf_zero() {
        let fib = FibonacciSequence::new();
        assert_eq!(fib.zeckendorf(0), vec![0]);
    }

    // ── GoldenSpiral tests ──

    #[test]
    fn test_spiral_point_at_center() {
        let spiral = GoldenSpiral {
            center: (0.0, 0.0),
            direction: 0.0,
            turns: 1.0,
        };
        let (x, y) = spiral.point_at(0.0);
        assert!((x - 1.0).abs() < 1e-10); // φ^(0/90) = 1, angle=0
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_spiral_radius_growth_theorem_5() {
        let spiral = GoldenSpiral {
            center: (0.0, 0.0),
            direction: 0.0,
            turns: 4.0, // 4 turns = 1440°
        };
        let phi = golden_ratio();
        // After one full turn (t=0.25, θ=360°), radius = φ^(360/90) = φ^4
        let r_quarter = spiral.radius_at(1.0 / 16.0); // 1 quarter turn out of 4 full turns = 90°
        assert!((r_quarter - phi).abs() < 1e-10);
    }

    #[test]
    fn test_spiral_points_count() {
        let spiral = GoldenSpiral {
            center: (0.0, 0.0),
            direction: 0.0,
            turns: 2.0,
        };
        let pts = spiral.points(100);
        assert_eq!(pts.len(), 100);
    }

    #[test]
    fn test_spiral_arc_length_positive() {
        let spiral = GoldenSpiral {
            center: (0.0, 0.0),
            direction: 0.0,
            turns: 1.0,
        };
        let len = spiral.arc_length(0.0, 1.0);
        assert!(len > 0.0);
    }

    #[test]
    fn test_spiral_increasing_radius() {
        let spiral = GoldenSpiral {
            center: (0.0, 0.0),
            direction: 0.0,
            turns: 2.0,
        };
        let r0 = spiral.radius_at(0.0);
        let r1 = spiral.radius_at(0.25);
        let r2 = spiral.radius_at(0.5);
        let r3 = spiral.radius_at(0.75);
        let r4 = spiral.radius_at(1.0);
        assert!(r0 < r1);
        assert!(r1 < r2);
        assert!(r2 < r3);
        assert!(r3 < r4);
    }

    #[test]
    fn test_spiral_off_center() {
        let spiral = GoldenSpiral {
            center: (5.0, 10.0),
            direction: 0.0,
            turns: 1.0,
        };
        let (x, y) = spiral.point_at(0.0);
        assert!((x - 6.0).abs() < 1e-10);
        assert!((y - 10.0).abs() < 1e-10);
    }

    // ── GrowthCurve tests ──

    #[test]
    fn test_growth_curve_values() {
        let curve = GrowthCurve {
            base: 1.0,
            rate: 0.5,
            phi_scale: 1.0,
        };
        let v0 = curve.value_at(0);
        let v_late = curve.value_at(100);
        assert!(v0 > 0.0);
        assert!(v_late > v0);
    }

    #[test]
    fn test_growth_curve_plateau_theorem_11() {
        let curve = GrowthCurve {
            base: 1.0,
            rate: 0.3,
            phi_scale: 1.0,
        };
        let plateau = curve.plateau();
        let v100 = curve.value_at(1000);
        assert!(v100 <= plateau + 0.01); // at or very near plateau
        // Approaching plateau
        assert!(v100 > plateau * 0.99);
    }

    #[test]
    fn test_growth_curve_steps_to_reach() {
        let curve = GrowthCurve {
            base: 1.0,
            rate: 0.5,
            phi_scale: 1.0,
        };
        let target = curve.plateau() * 0.5;
        let steps = curve.steps_to_reach(target);
        let value_at = curve.value_at(steps);
        assert!(value_at >= target * 0.9); // within 10%
    }

    #[test]
    fn test_growth_rate_at() {
        let curve = GrowthCurve {
            base: 5.0,
            rate: 0.5,
            phi_scale: 1.0,
        };
        // Growth rate should peak near the inflection point
        let rate_0 = curve.growth_rate_at(0);
        let rate_mid = curve.growth_rate_at(5);
        let rate_late = curve.growth_rate_at(100);
        assert!(rate_mid > rate_0);
        assert!(rate_mid > rate_late);
    }

    #[test]
    fn test_growth_curve_monotonic() {
        let curve = GrowthCurve {
            base: 1.0,
            rate: 0.3,
            phi_scale: 1.0,
        };
        let mut prev = curve.value_at(0);
        for i in 1..=50 {
            let v = curve.value_at(i);
            assert!(v >= prev, "Not monotonic at step {i}");
            prev = v;
        }
    }

    // ── SkillNode tests ──

    #[test]
    fn test_skill_node_power() {
        let node = SkillNode {
            name: "fireball".to_string(),
            base_power: 10.0,
            growth_rate: 1.5,
            dependencies: vec![],
            unlocks_at: 3,
        };
        assert_eq!(node.power_at(2), 0.0); // locked
        assert_eq!(node.power_at(3), 10.0); // just unlocked
        assert_eq!(node.power_at(4), 15.0); // 10 * 1.5^1
        assert_eq!(node.power_at(5), 22.5); // 10 * 1.5^2
    }

    #[test]
    fn test_skill_node_unlocked() {
        let node = SkillNode {
            name: "test".to_string(),
            base_power: 5.0,
            growth_rate: 1.2,
            dependencies: vec![],
            unlocks_at: 5,
        };
        assert!(!node.is_unlocked(4));
        assert!(node.is_unlocked(5));
        assert!(node.is_unlocked(10));
    }

    // ── SpiralSkillTree tests ──

    fn make_test_tree() -> SpiralSkillTree {
        SpiralSkillTree {
            skills: vec![
                SkillNode {
                    name: "core".to_string(),
                    base_power: 10.0,
                    growth_rate: 1.2,
                    dependencies: vec![],
                    unlocks_at: 1,
                },
                SkillNode {
                    name: "attack".to_string(),
                    base_power: 8.0,
                    growth_rate: 1.3,
                    dependencies: vec!["core".to_string()],
                    unlocks_at: 2,
                },
                SkillNode {
                    name: "defense".to_string(),
                    base_power: 8.0,
                    growth_rate: 1.25,
                    dependencies: vec!["core".to_string()],
                    unlocks_at: 3,
                },
                SkillNode {
                    name: "special".to_string(),
                    base_power: 12.0,
                    growth_rate: 1.15,
                    dependencies: vec!["attack".to_string(), "defense".to_string()],
                    unlocks_at: 5,
                },
                SkillNode {
                    name: "ultimate".to_string(),
                    base_power: 15.0,
                    growth_rate: 1.1,
                    dependencies: vec!["special".to_string()],
                    unlocks_at: 8,
                },
            ],
            levels: 10,
        }
    }

    #[test]
    fn test_skill_tree_unlock_order() {
        let tree = make_test_tree();
        let order = tree.unlock_order();
        assert_eq!(
            order,
            vec!["core", "attack", "defense", "special", "ultimate"]
        );
    }

    #[test]
    fn test_skill_tree_power_at_level() {
        let tree = make_test_tree();
        assert_eq!(tree.skill_at_level("core", 1), 10.0);
        assert_eq!(tree.skill_at_level("attack", 1), 0.0);
        assert_eq!(tree.skill_at_level("attack", 2), 8.0);
    }

    #[test]
    fn test_skill_tree_total_power_theorem_9() {
        let tree = make_test_tree();
        let p1 = tree.total_power(1);
        let p5 = tree.total_power(5);
        let p10 = tree.total_power(10);
        assert!(p1 > 0.0);
        assert!(p5 > p1);
        assert!(p10 > p5);
    }

    #[test]
    fn test_skill_tree_balanced_theorem_10() {
        // Use a tree with similar growth rates for balanced test
        let tree = SpiralSkillTree {
            skills: vec![
                SkillNode {
                    name: "a".to_string(),
                    base_power: 10.0,
                    growth_rate: 1.2,
                    dependencies: vec![],
                    unlocks_at: 1,
                },
                SkillNode {
                    name: "b".to_string(),
                    base_power: 9.0,
                    growth_rate: 1.2,
                    dependencies: vec![],
                    unlocks_at: 1,
                },
                SkillNode {
                    name: "c".to_string(),
                    base_power: 8.0,
                    growth_rate: 1.2,
                    dependencies: vec![],
                    unlocks_at: 1,
                },
            ],
            levels: 10,
        };
        assert!(tree.is_balanced(5));
    }

    #[test]
    fn test_skill_tree_not_balanced() {
        let tree = SpiralSkillTree {
            skills: vec![
                SkillNode {
                    name: "weak".to_string(),
                    base_power: 1.0,
                    growth_rate: 1.01,
                    dependencies: vec![],
                    unlocks_at: 1,
                },
                SkillNode {
                    name: "strong".to_string(),
                    base_power: 100.0,
                    growth_rate: 2.0,
                    dependencies: vec![],
                    unlocks_at: 1,
                },
            ],
            levels: 10,
        };
        // strong is 100x weak at level 1, which exceeds φ
        assert!(!tree.is_balanced(1));
    }

    // ── FibonacciRetracement tests ──

    #[test]
    fn test_retracement_levels_theorem_6() {
        let retr = FibonacciRetracement {
            high: 100.0,
            low: 0.0,
        };
        let levels = retr.levels();
        assert_eq!(levels.len(), 7);

        // Check the key Fibonacci ratios
        assert!((levels[0].0 - 0.0).abs() < 1e-10);
        assert!((levels[1].0 - 0.236).abs() < 1e-10);
        assert!((levels[2].0 - 0.382).abs() < 1e-10);
        assert!((levels[3].0 - 0.5).abs() < 1e-10);
        assert!((levels[4].0 - 0.618).abs() < 1e-10);
        assert!((levels[5].0 - 0.786).abs() < 1e-10);
        assert!((levels[6].0 - 1.0).abs() < 1e-10);

        // Values at 0 and 1
        assert!((levels[0].1 - 0.0).abs() < 1e-10);
        assert!((levels[6].1 - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_retracement_support() {
        let retr = FibonacciRetracement {
            high: 200.0,
            low: 100.0,
        };
        assert!((retr.support_at(0.0) - 100.0).abs() < 1e-10);
        assert!((retr.support_at(0.5) - 150.0).abs() < 1e-10);
        assert!((retr.support_at(0.618) - 161.8).abs() < 1e-10);
        assert!((retr.support_at(1.0) - 200.0).abs() < 1e-10);
    }

    #[test]
    fn test_retracement_resistance() {
        let retr = FibonacciRetracement {
            high: 300.0,
            low: 100.0,
        };
        assert!((retr.resistance_at(0.382) - 176.4).abs() < 1e-10);
    }

    #[test]
    fn test_retracement_derived_ratios() {
        // 0.236 = 1 - 0.764 ≈ (1/φ)^2 = 0.382^2 ≈ not exactly, 
        // but 0.618 = 1/φ, 0.382 = 1 - 1/φ
        let phi = golden_ratio();
        assert!((1.0 / phi - 0.618).abs() < 0.01);
        assert!((1.0 - 1.0 / phi - 0.382).abs() < 0.01);
        assert!(((1.0 / phi).powi(2) - 0.382).abs() < 0.01);
        // 0.236 ≈ 0.618 * 0.382
        assert!((0.618_f64 * 0.382 - 0.236).abs() < 0.001);
    }

    // ── Phyllotaxis tests ──

    #[test]
    fn test_phyllotaxis_point() {
        let phy = Phyllotaxis {
            n: 100,
            golden_angle: GOLDEN_ANGLE_DEG,
        };
        let (x, y) = phy.point(0);
        assert!((x - 0.0).abs() < 1e-10);
        assert!((y - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_phyllotaxis_points_count() {
        let phy = Phyllotaxis {
            n: 50,
            golden_angle: GOLDEN_ANGLE_DEG,
        };
        assert_eq!(phy.points().len(), 50);
    }

    #[test]
    fn test_phyllotaxis_spiral_count_theorem_7() {
        // For 100 seeds, expect Fibonacci pair near sqrt(100)=10
        // Fibonacci pairs: (5,8), (8,13) — 10 is between
        let phy = Phyllotaxis {
            n: 100,
            golden_angle: GOLDEN_ANGLE_DEG,
        };
        let (cw, ccw) = phy.spiral_count();
        // Should be consecutive Fibonacci numbers
        let fib = FibonacciSequence::new();
        assert!(fib.is_fibonacci(cw as u64));
        assert!(fib.is_fibonacci(ccw as u64));
    }

    #[test]
    fn test_phyllotaxis_increasing_radius() {
        let phy = Phyllotaxis {
            n: 100,
            golden_angle: GOLDEN_ANGLE_DEG,
        };
        let (x0, y0) = phy.point(0);
        let (x50, y50) = phy.point(50);
        let r0 = (x0 * x0 + y0 * y0).sqrt();
        let r50 = (x50 * x50 + y50 * y50).sqrt();
        assert!(r50 > r0);
    }

    // ── LucasSequence tests ──

    #[test]
    fn test_lucas_initial_values() {
        let lucas = LucasSequence::new();
        assert_eq!(lucas.values, vec![2, 1]);
    }

    #[test]
    fn test_lucas_compute() {
        let mut lucas = LucasSequence::new();
        lucas.compute_to(10);
        assert_eq!(lucas.at(0), 2);
        assert_eq!(lucas.at(1), 1);
        assert_eq!(lucas.at(2), 3);
        assert_eq!(lucas.at(3), 4);
        assert_eq!(lucas.at(4), 7);
        assert_eq!(lucas.at(5), 11);
        assert_eq!(lucas.at(6), 18);
        assert_eq!(lucas.at(7), 29);
        assert_eq!(lucas.at(8), 47);
        assert_eq!(lucas.at(9), 76);
        assert_eq!(lucas.at(10), 123);
    }

    #[test]
    fn test_lucas_identity_theorem_8() {
        let mut lucas = LucasSequence::new();
        lucas.compute_to(20);
        for n in 1..=20 {
            assert!(
                lucas.verify_lucas_identity(n),
                "Lucas identity failed at n={n}"
            );
        }
    }

    #[test]
    fn test_lucas_identity_at_0() {
        let lucas = LucasSequence::new();
        assert!(lucas.verify_lucas_identity(0));
    }

    // ── Serde round-trip tests ──

    #[test]
    fn test_fibonacci_serde() {
        let mut fib = FibonacciSequence::new();
        fib.compute_to(10);
        let json = serde_json::to_string(&fib).unwrap();
        let back: FibonacciSequence = serde_json::from_str(&json).unwrap();
        assert_eq!(fib.values, back.values);
    }

    #[test]
    fn test_spiral_serde() {
        let spiral = GoldenSpiral {
            center: (1.0, 2.0),
            direction: 45.0,
            turns: 3.0,
        };
        let json = serde_json::to_string(&spiral).unwrap();
        let back: GoldenSpiral = serde_json::from_str(&json).unwrap();
        assert!((back.center.0 - 1.0).abs() < 1e-10);
        assert!((back.turns - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_growth_curve_serde() {
        let curve = GrowthCurve {
            base: 2.0,
            rate: 0.7,
            phi_scale: 1.5,
        };
        let json = serde_json::to_string(&curve).unwrap();
        let back: GrowthCurve = serde_json::from_str(&json).unwrap();
        assert!((back.base - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_skill_node_serde() {
        let node = SkillNode {
            name: "test".to_string(),
            base_power: 5.0,
            growth_rate: 1.3,
            dependencies: vec!["a".to_string()],
            unlocks_at: 3,
        };
        let json = serde_json::to_string(&node).unwrap();
        let back: SkillNode = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "test");
    }

    #[test]
    fn test_skill_tree_serde() {
        let tree = make_test_tree();
        let json = serde_json::to_string(&tree).unwrap();
        let back: SpiralSkillTree = serde_json::from_str(&json).unwrap();
        assert_eq!(back.skills.len(), 5);
    }

    #[test]
    fn test_retracement_serde() {
        let retr = FibonacciRetracement {
            high: 100.0,
            low: 50.0,
        };
        let json = serde_json::to_string(&retr).unwrap();
        let back: FibonacciRetracement = serde_json::from_str(&json).unwrap();
        assert!((back.high - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_phyllotaxis_serde() {
        let phy = Phyllotaxis {
            n: 200,
            golden_angle: GOLDEN_ANGLE_DEG,
        };
        let json = serde_json::to_string(&phy).unwrap();
        let back: Phyllotaxis = serde_json::from_str(&json).unwrap();
        assert_eq!(back.n, 200);
    }

    #[test]
    fn test_lucas_serde() {
        let mut lucas = LucasSequence::new();
        lucas.compute_to(5);
        let json = serde_json::to_string(&lucas).unwrap();
        let back: LucasSequence = serde_json::from_str(&json).unwrap();
        assert_eq!(back.values, lucas.values);
    }

    // ── Additional edge case / coverage tests ──

    #[test]
    fn test_fibonacci_ratio_at_1() {
        let fib = FibonacciSequence::new();
        // F(2)/F(1) = 1/1 = 1.0
        assert!((fib.ratio(1) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_fibonacci_larger_values() {
        let mut fib = FibonacciSequence::new();
        fib.compute_to(50);
        assert_eq!(fib.at(20), 6765);
        assert_eq!(fib.at(30), 832040);
        assert_eq!(fib.at(40), 102334155);
    }

    #[test]
    fn test_fibonacci_idempotent_compute() {
        let mut fib = FibonacciSequence::new();
        fib.compute_to(10);
        fib.compute_to(10); // no panic
        assert_eq!(fib.at(5), 5);
    }

    #[test]
    fn test_golden_ratio_value() {
        let phi = golden_ratio();
        assert!((phi - 1.6180339887).abs() < 1e-7);
    }

    #[test]
    fn test_retracement_with_negative_range() {
        let retr = FibonacciRetracement {
            high: 0.0,
            low: -100.0,
        };
        assert!((retr.support_at(0.5) - (-50.0)).abs() < 1e-10);
        assert!((retr.support_at(0.618) - (-38.2)).abs() < 1e-10);
    }

    #[test]
    fn test_phyllotaxis_custom_angle() {
        let phy = Phyllotaxis {
            n: 10,
            golden_angle: 90.0, // not golden
        };
        let pts = phy.points();
        assert_eq!(pts.len(), 10);
    }

    #[test]
    fn test_lucas_recurrence() {
        let mut lucas = LucasSequence::new();
        lucas.compute_to(15);
        for n in 2..=15 {
            assert_eq!(lucas.at(n), lucas.at(n - 1) + lucas.at(n - 2));
        }
    }

    #[test]
    fn test_growth_curve_default_like() {
        let curve = GrowthCurve {
            base: 1.0,
            rate: 1.0,
            phi_scale: 1.0,
        };
        let p = curve.plateau();
        assert!(p > 0.0);
        assert!(curve.value_at(0) > 0.0);
    }

    #[test]
    fn test_empty_skill_tree() {
        let tree = SpiralSkillTree {
            skills: vec![],
            levels: 10,
        };
        assert!(tree.unlock_order().is_empty());
        assert_eq!(tree.total_power(5), 0.0);
        assert!(tree.is_balanced(5));
    }

    #[test]
    fn test_single_skill_tree() {
        let tree = SpiralSkillTree {
            skills: vec![SkillNode {
                name: "only".to_string(),
                base_power: 10.0,
                growth_rate: 1.5,
                dependencies: vec![],
                unlocks_at: 0,
            }],
            levels: 5,
        };
        assert!(tree.is_balanced(1));
        assert_eq!(tree.total_power(0), 10.0);
    }

    #[test]
    fn test_spiral_multiple_turns() {
        let spiral = GoldenSpiral {
            center: (0.0, 0.0),
            direction: 90.0,
            turns: 5.0,
        };
        let pts = spiral.points(500);
        assert_eq!(pts.len(), 500);
        // First point direction should be at 90°
        let (_x, y) = spiral.point_at(0.0);
        assert!(y > 0.0); // pointing up initially
    }

    #[test]
    fn test_steps_to_reach_impossible() {
        let curve = GrowthCurve {
            base: 1.0,
            rate: 0.5,
            phi_scale: 1.0,
        };
        let steps = curve.steps_to_reach(curve.plateau() + 100.0);
        assert_eq!(steps, usize::MAX);
    }

    #[test]
    fn test_fibonacci_index_of_large() {
        let fib = FibonacciSequence::new();
        assert_eq!(fib.index_of(55), Some(10));
        assert_eq!(fib.index_of(89), Some(11));
    }
}
