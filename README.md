# lau-fibonacci-growth

**Fibonacci growth patterns for agent capability development** — how skills grow in spirals, not lines.

A Rust library that wraps the mathematics of the Fibonacci sequence, the golden ratio, and phyllotaxis into composable, serializable types. Use it to model logarithmic spirals, build Fibonacci-scaled growth curves, lay out sunflower-seed patterns, or design skill trees whose unlock order follows consecutive Fibonacci indices.

---

## What This Does

Provides eight core types that turn Fibonacci number theory into practical Rust abstractions:

| Type | Purpose |
|---|---|
| `FibonacciSequence` | Compute, query, and decompose classic Fibonacci numbers |
| `GoldenSpiral` | Logarithmic spiral with φ growth per quarter turn |
| `GrowthCurve` | Fibonacci-scaled logistic (sigmoid) growth model |
| `SkillNode` | Single skill with level-gated unlock and exponential power |
| `SpiralSkillTree` | Collection of skills, balanced when no skill exceeds φ× another |
| `FibonacciRetracement` | Standard Fibonacci retracement levels (finance) |
| `Phyllotaxis` | Vogel's sunflower-spiral point generator |
| `LucasSequence` | Lucas companion sequence with identity verification |

Every type derives `Serialize`/`Deserialize` so you can persist configurations and computed state as JSON (or any serde format).

---

## Key Idea

The Fibonacci sequence doesn't just produce numbers — it encodes a *growth rhythm*. Consecutive ratios converge to φ ≈ 1.618 (the golden ratio), and that single constant shows up in spirals, skill balancing, sunflower seed placement, and financial support levels. This library makes that rhythm first-class in Rust.

---

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
lau-fibonacci-growth = "0.1"
```

Or via `cargo add`:

```sh
cargo add lau-fibonacci-growth
```

Requires Rust 2021 edition. The only runtime dependency is `serde` (with `derive`).

---

## Quick Start

```rust
use lau_fibonacci_growth::*;

// Classic Fibonacci
let mut fib = FibonacciSequence::new();
fib.compute_to(20);
println!("F(10) = {}", fib.at(10));       // 55
println!("F(20) = {}", fib.at(20));       // 6765

// Golden ratio convergence
let phi = golden_ratio();
println!("φ ≈ {phi}");                     // 1.6180339887…
println!("F(20)/F(19) ≈ {}", fib.ratio(19)); // approaches φ

// Zeckendorf representation — every integer is a sum of non-consecutive Fib numbers
let z = fib.zeckendorf(100);
// 100 = F(11) + F(6) + F(4) = 89 + 8 + 3
println!("Zeckendorf(100) indices: {z:?}");

// Golden spiral
let spiral = GoldenSpiral {
    center: (0.0, 0.0),
    direction: 0.0,
    turns: 3.0,
};
let (x, y) = spiral.point_at(0.5); // halfway around the spiral
println!("Point at t=0.5: ({x:.2}, {y:.2})");

// Growth curve — Fibonacci-scaled logistic
let curve = GrowthCurve { base: 5.0, rate: 0.4, phi_scale: 1.0 };
println!("Plateau: {}", curve.plateau());
println!("Steps to 50%: {}", curve.steps_to_reach(curve.plateau() * 0.5));

// Phyllotaxis — sunflower seed layout
let phy = Phyllotaxis { n: 500, golden_angle: 137.5077640500378 };
let (cx, cy) = phy.point(42);
let (cw, ccw) = phy.spiral_count(); // consecutive Fibonacci numbers
println!("Spiral arms: {cw} CW, {ccw} CCW");

// Fibonacci retracement
let retr = FibonacciRetracement { high: 200.0, low: 100.0 };
for (ratio, value) in retr.levels() {
    println!("{ratio:.3} → {value:.1}");
}

// Lucas sequence
let mut lucas = LucasSequence::new();
lucas.compute_to(10);
assert!(lucas.verify_lucas_identity(5)); // L(n) = F(n-1) + F(n+1)
```

---

## API Reference

### Constants

| Name | Value | Description |
|---|---|---|
| `PHI` | ≈ 1.618 | Golden ratio (computed constant) |
| `GOLDEN_ANGLE_DEG` | ≈ 137.508° | Golden angle in degrees |
| `golden_ratio()` | (1 + √5) / 2 | Precise φ at runtime |

### `FibonacciSequence`

| Method | Signature | Returns |
|---|---|---|
| `new` | `() → Self` | Starts with `[0, 1]` |
| `compute_to` | `(&mut self, n: usize)` | Extend sequence to index `n` |
| `at` | `(&self, n: usize) → u64` | F(n) — lazy-computes if needed |
| `ratio` | `(&self, n: usize) → f64` | F(n+1)/F(n) → approaches φ |
| `golden_ratio_approximation` | `(&self) → f64` | Last computed ratio |
| `is_fibonacci` | `(&self, n: u64) → bool` | Test via 5n²±4 perfect-square check |
| `index_of` | `(&self, n: u64) → Option<usize>` | Which index produces `n` |
| `sum_to` | `(&self, n: usize) → u64` | Σ F(0..=n) = F(n+2) − 1 |
| `zeckendorf` | `(&self, n: u64) → Vec<usize>` | Non-consecutive Fibonacci decomposition |

### `GoldenSpiral`

| Method | Signature | Returns |
|---|---|---|
| `point_at` | `(&self, t: f64) → (f64, f64)` | (x, y) at parameter t ∈ [0, 1] |
| `radius_at` | `(&self, t: f64) → f64` | r = φ^(θ/90°) |
| `arc_length` | `(&self, t_start, t_end) → f64` | Numerical (Simpson's rule) |
| `points` | `(&self, n: usize) → Vec<(f64, f64)>` | n evenly-spaced points |

Fields: `center: (f64, f64)`, `direction: f64` (degrees), `turns: f64`.

### `GrowthCurve`

A logistic (sigmoid) curve scaled by φ:

```
value(step) = plateau / (1 + e^(−rate × (step − base × phi_scale)))
plateau     = base × φ × phi_scale × 10
```

| Method | Signature | Returns |
|---|---|---|
| `value_at` | `(&self, step: usize) → f64` | Growth value at step |
| `plateau` | `(&self) → f64` | Asymptotic maximum |
| `steps_to_reach` | `(&self, target: f64) → usize` | Steps to hit target (or `usize::MAX`) |
| `growth_rate_at` | `(&self, step: usize) → f64` | Derivative (inflection peak) |

Fields: `base: f64`, `rate: f64`, `phi_scale: f64`.

### `SkillNode`

| Method | Signature | Returns |
|---|---|---|
| `power_at` | `(&self, level: usize) → f64` | base_power × growth_rate^(level − unlocks_at), or 0 if locked |
| `is_unlocked` | `(&self, level: usize) → bool` | level ≥ unlocks_at |

Fields: `name: String`, `base_power: f64`, `growth_rate: f64`, `dependencies: Vec<String>`, `unlocks_at: usize`.

### `SpiralSkillTree`

| Method | Signature | Returns |
|---|---|---|
| `unlock_order` | `(&self) → Vec<String>` | Skills sorted by unlock level |
| `skill_at_level` | `(&self, name, level) → f64` | Power of one skill at level |
| `total_power` | `(&self, level: usize) → f64` | Sum of all unlocked skill powers |
| `is_balanced` | `(&self, level: usize) → bool` | No skill exceeds φ × min skill |

### `FibonacciRetracement`

| Method | Signature | Returns |
|---|---|---|
| `levels` | `(&self) → Vec<(f64, f64)>` | (ratio, value) for 7 standard levels |
| `support_at` | `(&self, ratio: f64) → f64` | low + ratio × (high − low) |
| `resistance_at` | `(&self, ratio: f64) → f64` | Alias for support (symmetric) |

Standard ratios: 0.0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0.

### `Phyllotaxis`

| Method | Signature | Returns |
|---|---|---|
| `point` | `(&self, index: usize) → (f64, f64)` | Vogel's formula: r = √index, θ = index × golden_angle |
| `points` | `(&self) → Vec<(f64, f64)>` | All n seed positions |
| `spiral_count` | `(&self) → (usize, usize)` | (CW, CCW) spiral arm counts — consecutive Fibonacci numbers |

### `LucasSequence`

| Method | Signature | Returns |
|---|---|---|
| `new` | `() → Self` | Starts with `[2, 1]` |
| `compute_to` | `(&mut self, n: usize)` | Extend to index `n` |
| `at` | `(&self, n: usize) → u64` | L(n) |
| `verify_lucas_identity` | `(&self, n: usize) → bool` | L(n) = F(n−1) + F(n+1) |

---

## How It Works

### Lazy computation

`FibonacciSequence` and `LucasSequence` store computed values in a `Vec`. Calling `compute_to(n)` extends the vector only as far as needed. `at(n)` can also compute on-the-fly in a fresh temporary, so you can query without mutable access.

### Perfect-square Fibonacci test

`is_fibonacci(n)` doesn't build a table — it uses the number-theoretic test: n is a Fibonacci number if and only if one of 5n²+4 or 5n²−4 is a perfect square. This works for arbitrarily large `u64` values without precomputation.

### Zeckendorf decomposition

The greedy algorithm works because Fibonacci numbers form a complete sequence: starting from the largest F ≤ n and working down guarantees the non-consecutive property.

### Golden spiral

The spiral uses the polar equation r = φ^(θ/90°). Each quarter turn (90°) multiplies the radius by φ, which is exactly how the Fibonacci spiral grows when you tile quarter-circle arcs through consecutive Fibonacci squares.

### Growth curve

A standard logistic function `L / (1 + e^(−k(x−x₀)))` with the inflection point and plateau scaled by φ × phi_scale. The derivative `dV/ds` peaks at the inflection point — this is where skill growth is fastest.

### Spiral skill tree balancing

`is_balanced` checks the φ-constraint: the most powerful skill must not exceed φ × the least powerful. This mirrors how Fibonacci spirals distribute energy — no single arm dominates.

### Phyllotaxis

Uses Vogel's model: `r = c√n`, `θ = n × golden_angle`. The golden angle (≈ 137.508°) is the irrational angle most effective at distributing points evenly. The resulting spiral arm counts are always consecutive Fibonacci numbers.

---

## The Math

### Theorem 1 — Golden Ratio Convergence

F(n+1) / F(n) → φ = (1 + √5) / 2 ≈ 1.6180339887…

### Theorem 2 — Fibonacci Recurrence

F(n) = F(n−1) + F(n−2), with F(0) = 0, F(1) = 1.

### Theorem 3 — Sum Formula

Σ F(i) for i = 0..n = F(n+2) − 1

### Theorem 4 — Zeckendorf's Theorem

Every positive integer has a unique representation as a sum of non-consecutive Fibonacci numbers (excluding F(0) and F(1)).

### Theorem 5 — Spiral Growth Factor

The golden spiral grows by factor φ per quarter turn: r(θ) = φ^(θ/90°).

### Theorem 6 — Fibonacci Retracement Ratios

The standard ratios derive from φ:

| Ratio | Derivation |
|---|---|
| 0.618 | 1/φ |
| 0.382 | 1 − 1/φ = 1/φ² |
| 0.236 | (1/φ)² |
| 0.786 | √(1/φ) |

### Theorem 7 — Phyllotaxis Spiral Arms

For n seeds placed with the golden angle, the number of visible CW and CCW spirals are consecutive Fibonacci numbers F(k) and F(k+1) where F(k) ≈ √n.

### Theorem 8 — Lucas Identity

L(n) = F(n−1) + F(n+1), connecting the Lucas sequence (2, 1, 3, 4, 7, 11, …) to Fibonacci.

### Theorem 9 — Total Power Monotonicity

Total skill power is monotonically non-decreasing with level (each skill's power function is non-negative and grows).

### Theorem 10 — φ-Balance Constraint

A skill tree is balanced when max_power ≤ φ × min_power across all unlocked skills.

### Theorem 11 — Logistic Plateau

The φ-scaled logistic curve approaches plateau = base × φ × phi_scale × 10 asymptotically — values can never exceed it.

### Theorem 12 — Base Cases

F(0) = 0, F(1) = 1, establishing the foundation for all derived computations.

---

## Testing

63 tests covering:

- Fibonacci recurrence, base cases, and large-value computation
- Golden ratio convergence (error bounds at F(10) and F(20))
- Zeckendorf decomposition for every integer 1–100, plus specific cases
- Sum formula verification
- Spiral point generation, radius growth, and arc length
- Growth curve monotonicity, plateau, and steps-to-reach
- Skill tree unlock order, power, balance, and imbalance detection
- Fibonacci membership test and index lookup
- Retracement level values and derived ratio relationships
- Phyllotaxis point generation and spiral arm counting
- Lucas sequence computation and identity verification
- Serde round-trip for all types

Run them:

```sh
cargo test
```

---

## License

MIT
