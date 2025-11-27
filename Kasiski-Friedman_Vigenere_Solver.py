import re
from collections import Counter
import math

# =========================
# Input
# =========================
CIPHERTEXT = """ENTER CIPHERTEXT HERE"""
KNOWN_KEY = None  # Set to a string like "FRIEND" when you know the key; otherwise leave as None.

# Keep only uppercase A–Z (strip spaces, punctuation, digits)
cipher = re.sub(r'[^A-Z]', '', CIPHERTEXT.upper())

# =========================
# English frequency model
# =========================
# Relative English letter frequencies (in %). Converted to probabilities ENGp for scoring.
ENG = {
    'A':8.167,'B':1.492,'C':2.782,'D':4.253,'E':12.702,'F':2.228,'G':2.015,'H':6.094,'I':6.966,'J':0.153,'K':0.772,
    'L':4.025,'M':2.406,'N':6.749,'O':7.507,'P':1.929,'Q':0.095,'R':5.987,'S':6.327,'T':9.056,'U':2.758,'V':0.978,
    'W':2.360,'X':0.150,'Y':1.974,'Z':0.074
}
ENGp = {k: v/100 for k, v in ENG.items()}

# =========================
# Core statistics
# =========================
def index_of_coincidence(text: str) -> float:
    """Compute IC = sum c_i(c_i-1) / (N(N-1)); higher (~0.066) suggests monoalphabetic."""
    N = len(text)
    if N < 2:
        return 0.0
    counts = Counter(text)
    return sum(c * (c - 1) for c in counts.values()) / (N * (N - 1))

def friedman_estimate(N: int, ic: float) -> float:
    """Friedman key length estimate for Vigenère."""
    return (0.027 * N) / ((0.065 - ic) + N * (ic - 0.038))

def kasiski_distances(text: str, n: int = 3):
    """Find distances between repeated n-grams (for Kasiski examination)."""
    positions = {}
    for i in range(len(text) - n + 1):
        chunk = text[i:i+n]
        positions.setdefault(chunk, []).append(i)
    dists = []
    for pos in positions.values():
        for a, b in zip(pos, pos[1:]):
            dists.append(b - a)
    return dists

def factorize(n: int):
    """Return non-trivial factors of n (for spotting common key-length factors)."""
    f = set()
    for i in range(2, int(math.sqrt(n)) + 1):
        if n % i == 0:
            f.add(i); f.add(n // i)
    return f

# =========================
# Vigenère helpers
# =========================
def vigenere_decrypt(ct: str, key: str) -> str:
    """Classic Vigenère decryption: P = C - K (mod 26)."""
    key = key.upper()
    out = []
    for i, c in enumerate(ct):
        k = ord(key[i % len(key)]) - 65
        p = (ord(c) - 65 - k) % 26
        out.append(chr(p + 65))
    return ''.join(out)

def best_caesar_shift(column: str):
    """Find the Caesar shift that best matches English (chi-squared). Returns (shift, score)."""
    N = len(column)
    best_shift, best_score = None, float('inf')
    for s in range(26):
        # Shift back by s to 'decrypt' this column
        dec = [(ord(c) - 65 - s) % 26 for c in column]
        counts = Counter(chr(x + 65) for x in dec)
        # Chi-squared against expected English frequencies
        chi = 0.0
        for ch in ENGp:
            expected = ENGp[ch] * N
            observed = counts.get(ch, 0)
            if expected > 0:
                chi += ((observed - expected) ** 2) / expected
        if chi < best_score:
            best_shift, best_score = s, chi
    return best_shift, best_score

def score_key_length(ct: str, m: int):
    """Split ciphertext into m columns; compute best Caesar shift and total chi-squared."""
    columns = [ct[i::m] for i in range(m)]
    shifts, total_chi = [], 0.0
    for col in columns:
        s, chi = best_caesar_shift(col)
        shifts.append(s); total_chi += chi
    return shifts, total_chi

def shifts_to_key(shifts):
    """Convert numeric shifts (0=A,..,25=Z) to a Vigenère key string."""
    return ''.join(chr(65 + s) for s in shifts)

# =========================
# Analysis & candidate recovery
# =========================
N = len(cipher)
ic = index_of_coincidence(cipher)
fried = friedman_estimate(N, ic)
print(f"Length: {N}, IC: {ic:.5f}, Friedman keylen ~ {fried:.2f}")

# Kasiski: gather distances between repeated trigrams and factor frequencies
dists = kasiski_distances(cipher, n=3)
factor_counts = Counter(f for d in dists for f in factorize(d))
print("Kasiski factor candidates (top 10):", factor_counts.most_common(10))

# Try key lengths 3..20 and rank by total chi-squared score
candidates = []
for m in range(3, 21):
    shifts, total = score_key_length(cipher, m)
    candidates.append((m, total, shifts))
candidates.sort(key=lambda x: x[1])

print("\nTop key-length candidates:")
for m, total, shifts in candidates[:5]:
    print(f"m={m:2d}  chi={total:.1f}  shifts={shifts}  key='{shifts_to_key(shifts)}'")

# Preview plaintext for top 3 candidates
print("\nPlaintext previews:")
for m, total, shifts in candidates[:3]:
    key = shifts_to_key(shifts)
    pt = vigenere_decrypt(cipher, key)
    print(f"\nKeylen {m}, key {key}")
    print(pt[:300])

# =========================
# Final decryption (when key known)
# =========================
if KNOWN_KEY:
    plaintext = vigenere_decrypt(cipher, KNOWN_KEY)
    print("\n--- Final decryption ---")
    print(f"Key: {KNOWN_KEY}")
    print(plaintext)
else:
    print("\nNo KNOWN_KEY provided. Set KNOWN_KEY = 'YOURKEY' to decrypt fully.")