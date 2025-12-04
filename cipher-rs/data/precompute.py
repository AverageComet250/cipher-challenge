#!/usr/bin/env python3
"""
compute_ln_probs.py

Reads an input file with lines "TOKEN COUNT" (whitespace-separated),
computes ln(probabilities), determines a significance epsilon based on
a desired standard error in ln-space, ignores low-probability entries,
and prints/writes a Rust-style map of ln(probabilities).

Usage:
  python3 compute_ln_probs.py input.txt --tol 0.1 --sig-decimals 10 --out rust_map.txt

Notes:
- Default tol = 0.1 (max allowed stderr of ln(p)). Tweak if you prefer stricter/looser significance.
- The epsilon derived is: epsilon = 1 / (1 + N * tol^2)
  (for small p this ≈ 1 / (N * tol^2)). Any p < epsilon will be ignored.
- ln_missing fallback = ln(epsilon)
- Output formatting uses a fixed number of decimal places for ln(p); default is 10 (more than f32's precision).
"""

import argparse
import math
import sys
import os
from typing import List, Tuple


# ----------------------------------------
# Input parsing
# ----------------------------------------
def parse_input(path: str) -> List[Tuple[str, int]]:
    items = []
    with open(path, "r", encoding="utf-8") as f:
        for lineno, line in enumerate(f, start=1):
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split()
            if len(parts) < 2:
                print(f"warning: skipping malformed line {lineno}: {line}", file=sys.stderr)
                continue
            token = parts[0]
            try:
                count = int(parts[1])
            except ValueError:
                print(f"warning: skipping non-integer count on line {lineno}: {line}", file=sys.stderr)
                continue
            if count <= 0:
                continue
            items.append((token, count))
    return items


# ----------------------------------------
# Main probability + significance logic
# ----------------------------------------
def compute_ln_probs(items: List[Tuple[str, int]], tol: float):
    total = sum(count for (_, count) in items)
    if total <= 0:
        raise ValueError("no counts found (total == 0)")

    results = []
    for token, count in items:
        p = count / total
        ln_p = math.log(p)
        # approximate stderr of ln(p): sqrt((1-p)/(N*p))
        stderr_ln = math.sqrt((1.0 - p) / (total * p))
        results.append({
            "token": token,
            "count": count,
            "p": p,
            "ln_p": ln_p,
            "stderr_ln": stderr_ln,
            "significant": stderr_ln <= tol
        })

    # epsilon threshold: p = 1 / (1 + N * tol^2)
    epsilon = 1.0 / (1.0 + total * (tol ** 2))
    min_counts_for_epsilon = math.ceil(epsilon * total)

    return total, epsilon, min_counts_for_epsilon, results


# ----------------------------------------
# Rust map formatter
# ----------------------------------------
def format_rust_map(results, epsilon, sig_decimal_places=10, include_ignored=False):
    lines = []
    fmt = f"{{:.{sig_decimal_places}f}}"
    for r in results:
        if not include_ignored:
            if (not r["significant"]) or (r["p"] < epsilon):
                continue
        lines.append(f"\"{r['token']}\" => {fmt.format(r['ln_p'])},")
    return "\n".join(lines)


# ----------------------------------------
# Main
# ----------------------------------------
def main(argv=None):
    parser = argparse.ArgumentParser(description="Compute ln-probabilities and significance epsilon.")
    parser.add_argument("input", nargs="?", help="Input file path (token count per line).")
    parser.add_argument("--tol", type=float, default=0.1,
                        help="Max allowed stderr of ln(p) (default 0.1).")
    parser.add_argument("--sig-decimals", type=int, default=10,
                        help="Decimal places after decimal point for ln(p) output.")
    parser.add_argument("--out", help="Output filename for Rust map (default stdout).")
    parser.add_argument("--include-ignored", action="store_true",
                        help="Include entries below epsilon and significance threshold.")
    args, _ = parser.parse_known_args(argv)

    # Fall back to demo if running in Jupyter or missing file
    in_jupyter = "ipykernel" in sys.modules
    use_demo = in_jupyter or not (args.input and os.path.isfile(args.input))

    if args.input and not os.path.isfile(args.input):
        print(f"# input path not found: {args.input} — running demo instead.", file=sys.stderr)

    if use_demo:
        items = [
            ("ASOE", 91493),
            ("THE", 50000),
            ("QZ", 1),
            ("XYZ", 3),
            ("HELLO", 200)
        ]
        print("# Running demo with sample data.", file=sys.stderr)
    else:
        items = parse_input(args.input)
        if not items:
            print("No valid items parsed from input file.", file=sys.stderr)
            sys.exit(2)

    total, epsilon, min_counts, results = compute_ln_probs(items, args.tol)

    # ln-space fallback for missing quadgrams
    ln_missing = math.log(epsilon)

    # CLI summary
    print(f"# total counts: {total}")
    print(f"# tol (stderr ln-space): {args.tol}")
    print(f"# epsilon threshold (prob): {epsilon:.12g}")
    print(f"# ln_missing_epsilon = {ln_missing:.18f}")
    print(f"# minimal counts to reach epsilon: {min_counts}")

    sig_count = sum(1 for r in results if r["significant"] and r["p"] >= epsilon)
    print(f"# significant entries above epsilon: {sig_count}/{len(results)}\n")

    rust_map = format_rust_map(results, epsilon,
                               sig_decimal_places=args.sig_decimals,
                               include_ignored=args.include_ignored)

    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(rust_map + "\n")
        print(f"# Written Rust map snippet to {args.out}")
    else:
        print("# Rust map snippet")
        print(rust_map)


if __name__ == "__main__":
    main()
