# lau-fibonacci-growth

Fibonacci growth patterns for agent capability development — how skills grow in spirals, not lines.

## Modules

- **FibonacciSequence** — the golden sequence (compute, ratio, Zeckendorf representation)
- **GoldenSpiral** — logarithmic spiral with φ growth per quarter turn
- **GrowthCurve** — Fibonacci-scaled logistic growth model
- **SpiralSkillTree** — skill development as a spiral unlock pattern
- **SkillNode** — individual skill with Fibonacci-indexed unlock
- **FibonacciRetracement** — standard retracement levels (financial analysis)
- **Phyllotaxis** — sunflower spiral patterns (Vogel's formula)
- **LucasSequence** — companion to Fibonacci, verifying Lucas identity

## Theorems Verified

All 12 theorems are verified by the test suite:

1. F(n+1)/F(n) → φ (golden ratio convergence)
2. F(n) = F(n-1) + F(n-2) (recurrence relation)
3. Σ F(i) = F(n+2) - 1 (sum formula)
4. Zeckendorf representation uniqueness
5. Golden spiral radius grows by φ per quarter turn
6. Fibonacci retracement levels (0.236, 0.382, 0.618)
7. Phyllotaxis spiral counts are consecutive Fibonacci numbers
8. Lucas identity: L(n) = F(n-1) + F(n+1)
9. Skill tree total power follows growth curve
10. Balanced skills: no skill exceeds φ× another
11. Growth curve reaches asymptotic plateau
12. Fibonacci initial values: F(0)=0, F(1)=1, F(2)=1, F(3)=2, F(4)=3, F(5)=5

## Usage

```rust
use lau_fibonacci_growth::*;

let mut fib = FibonacciSequence::new();
fib.compute_to(20);
assert_eq!(fib.at(10), 55);

let spiral = GoldenSpiral { center: (0.0, 0.0), direction: 0.0, turns: 2.0 };
let points = spiral.points(100);

let retracement = FibonacciRetracement { high: 100.0, low: 50.0 };
let level_618 = retracement.support_at(0.618);
```
