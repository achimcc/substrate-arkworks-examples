# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [compile:groth16:default](#compile:groth16:default)
    - [compile:bls12-381-pairing:default](#compile:bls12-381-pairing:default)
    - [compile:bls12-381-msm-g1-10:default](#compile:bls12-381-msm-g1-10:default)
    - [compile:bls12-381-msm-g1-1000:default](#compile:bls12-381-msm-g1-1000:default)
    - [compile:bls12-381-msm-g2-10:default](#compile:bls12-381-msm-g2-10:default)
    - [compile:bls12-381-msm-g2-1000:default](#compile:bls12-381-msm-g2-1000:default)
    - [compile:bls12-381-mul-affine-g1:default](#compile:bls12-381-mul-affine-g1:default)
    - [compile:bls12-381-mul-affine-g2:default](#compile:bls12-381-mul-affine-g2:default)
    - [compile:bls12-381-mul-projective-g1:default](#compile:bls12-381-mul-projective-g1:default)
    - [compile:bls12-381-mul-projective-g2:default](#compile:bls12-381-mul-projective-g2:default)
    - [compile:bls12-377-pairing:default](#compile:bls12-377-pairing:default)
    - [compile:bls12-377-msm-g1-10:default](#compile:bls12-377-msm-g1-10:default)
    - [compile:bls12-377-msm-g1-1000:default](#compile:bls12-377-msm-g1-1000:default)
    - [compile:bls12-377-msm-g2-10:default](#compile:bls12-377-msm-g2-10:default)
    - [compile:bls12-377-msm-g2-1000:default](#compile:bls12-377-msm-g2-1000:default)
    - [compile:bls12-377-mul-affine-g1:default](#compile:bls12-377-mul-affine-g1:default)
    - [compile:bls12-377-mul-affine-g2:default](#compile:bls12-377-mul-affine-g2:default)
    - [compile:bls12-377-mul-projective-g1:default](#compile:bls12-377-mul-projective-g1:default)
    - [compile:bls12-377-mul-projective-g2:default](#compile:bls12-377-mul-projective-g2:default)
    - [compile:bw6-761-pairing:default](#compile:bw6-761-pairing:default)
    - [compile:bw6-761-msm-g1-10:default](#compile:bw6-761-msm-g1-10:default)
    - [compile:bw6-761-msm-g1-1000:default](#compile:bw6-761-msm-g1-1000:default)
    - [compile:bw6-761-msm-g2-10:default](#compile:bw6-761-msm-g2-10:default)
    - [compile:bw6-761-msm-g2-1000:default](#compile:bw6-761-msm-g2-1000:default)
    - [compile:bw6-761-mul-affine-g1:default](#compile:bw6-761-mul-affine-g1:default)
    - [compile:bw6-761-mul-affine-g2:default](#compile:bw6-761-mul-affine-g2:default)
    - [compile:bw6-761-mul-projective-g1:default](#compile:bw6-761-mul-projective-g1:default)
    - [compile:bw6-761-mul-projective-g2:default](#compile:bw6-761-mul-projective-g2:default)
    - [compile:ed-on-bls12-381-msm-sw-10:default](#compile:ed-on-bls12-381-msm-sw-10:default)
    - [compile:ed-on-bls12-381-msm-sw-1000:default](#compile:ed-on-bls12-381-msm-sw-1000:default)
    - [compile:ed-on-bls12-381-msm-te-10:default](#compile:ed-on-bls12-381-msm-te-10:default)
    - [compile:ed-on-bls12-381-msm-te-1000:default](#compile:ed-on-bls12-381-msm-te-1000:default)
    - [compile:ed-on-bls12-381-mul-affine-sw:default](#compile:ed-on-bls12-381-mul-affine-sw:default)
    - [compile:ed-on-bls12-381-mul-affine-te:default](#compile:ed-on-bls12-381-mul-affine-te:default)
    - [compile:ed-on-bls12-381-mul-projective-sw:default](#compile:ed-on-bls12-381-mul-projective-sw:default)
    - [compile:ed-on-bls12-381-mul-projective-te:default](#compile:ed-on-bls12-381-mul-projective-te:default)
    - [compile:ed-on-bls12-377-mul-affine:default](#compile:ed-on-bls12-377-mul-affine:default)
    - [compile:ed-on-bls12-377-mul-projective:default](#compile:ed-on-bls12-377-mul-projective:default)
    - [compile:ed-on-bls12-377-msm-10:default](#compile:ed-on-bls12-377-msm-10:default)
    - [compile:ed-on-bls12-377-msm-1000:default](#compile:ed-on-bls12-377-msm-1000:default)
    - [exec:groth16:default](#exec:groth16:default)
    - [exec:bls12-381-pairing:default](#exec:bls12-381-pairing:default)
    - [exec:bls12-381-msm-g1-10:default](#exec:bls12-381-msm-g1-10:default)
    - [exec:bls12-381-msm-g1-1000:default](#exec:bls12-381-msm-g1-1000:default)
    - [exec:bls12-381-msm-g2-10:default](#exec:bls12-381-msm-g2-10:default)
    - [exec:bls12-381-msm-g2-1000:default](#exec:bls12-381-msm-g2-1000:default)
    - [exec:bls12-381-mul-affine-g1:default](#exec:bls12-381-mul-affine-g1:default)
    - [exec:bls12-381-mul-affine-g2:default](#exec:bls12-381-mul-affine-g2:default)
    - [exec:bls12-381-mul-projective-g1:default](#exec:bls12-381-mul-projective-g1:default)
    - [exec:bls12-381-mul-projective-g2:default](#exec:bls12-381-mul-projective-g2:default)
    - [exec:bls12-377-pairing:default](#exec:bls12-377-pairing:default)
    - [exec:bls12-377-msm-g1-10:default](#exec:bls12-377-msm-g1-10:default)
    - [exec:bls12-377-msm-g1-1000:default](#exec:bls12-377-msm-g1-1000:default)
    - [exec:bls12-377-msm-g2-10:default](#exec:bls12-377-msm-g2-10:default)
    - [exec:bls12-377-msm-g2-1000:default](#exec:bls12-377-msm-g2-1000:default)
    - [exec:bls12-377-mul-affine-g1:default](#exec:bls12-377-mul-affine-g1:default)
    - [exec:bls12-377-mul-affine-g2:default](#exec:bls12-377-mul-affine-g2:default)
    - [exec:bls12-377-mul-projective-g1:default](#exec:bls12-377-mul-projective-g1:default)
    - [exec:bls12-377-mul-projective-g2:default](#exec:bls12-377-mul-projective-g2:default)
    - [exec:bw6-761-pairing:default](#exec:bw6-761-pairing:default)
    - [exec:bw6-761-msm-g1-10:default](#exec:bw6-761-msm-g1-10:default)
    - [exec:bw6-761-msm-g1-1000:default](#exec:bw6-761-msm-g1-1000:default)
    - [exec:bw6-761-msm-g2-10:default](#exec:bw6-761-msm-g2-10:default)
    - [exec:bw6-761-msm-g2-1000:default](#exec:bw6-761-msm-g2-1000:default)
    - [exec:bw6-761-mul-affine-g1:default](#exec:bw6-761-mul-affine-g1:default)
    - [exec:bw6-761-mul-affine-g2:default](#exec:bw6-761-mul-affine-g2:default)
    - [exec:bw6-761-mul-projective-g1:default](#exec:bw6-761-mul-projective-g1:default)
    - [exec:bw6-761-mul-projective-g2:default](#exec:bw6-761-mul-projective-g2:default)
    - [exec:ed-on-bls12-381-msm-sw-10:default](#exec:ed-on-bls12-381-msm-sw-10:default)
    - [exec:ed-on-bls12-381-msm-sw-1000:default](#exec:ed-on-bls12-381-msm-sw-1000:default)
    - [exec:ed-on-bls12-381-msm-te-10:default](#exec:ed-on-bls12-381-msm-te-10:default)
    - [exec:ed-on-bls12-381-msm-te-1000:default](#exec:ed-on-bls12-381-msm-te-1000:default)
    - [exec:ed-on-bls12-381-mul-affine-sw:default](#exec:ed-on-bls12-381-mul-affine-sw:default)
    - [exec:ed-on-bls12-381-mul-affine-te:default](#exec:ed-on-bls12-381-mul-affine-te:default)
    - [exec:ed-on-bls12-381-mul-projective-sw:default](#exec:ed-on-bls12-381-mul-projective-sw:default)
    - [exec:ed-on-bls12-381-mul-projective-te:default](#exec:ed-on-bls12-381-mul-projective-te:default)
    - [exec:ed-on-bls12-377-mul-affine:default](#exec:ed-on-bls12-377-mul-affine:default)
    - [exec:ed-on-bls12-377-mul-projective:default](#exec:ed-on-bls12-377-mul-projective:default)
    - [exec:ed-on-bls12-377-msm-10:default](#exec:ed-on-bls12-377-msm-10:default)
    - [exec:ed-on-bls12-377-msm-1000:default](#exec:ed-on-bls12-377-msm-1000:default)

## Benchmark Results

### compile:groth16:default

|        | `2209532 bytes`            |
|:-------|:-------------------------- |
|        | `180.28 ms` (✅ **1.00x**)  |

### compile:bls12-381-pairing:default

|        | `2115526 bytes`            |
|:-------|:-------------------------- |
|        | `116.59 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-10:default

|        | `2057684 bytes`           |
|:-------|:------------------------- |
|        | `92.71 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g1-1000:default

|        | `2057689 bytes`           |
|:-------|:------------------------- |
|        | `88.12 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-10:default

|        | `2052877 bytes`           |
|:-------|:------------------------- |
|        | `73.45 ms` (✅ **1.00x**)  |

### compile:bls12-381-msm-g2-1000:default

|        | `2052882 bytes`           |
|:-------|:------------------------- |
|        | `75.14 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g1:default

|        | `2026347 bytes`           |
|:-------|:------------------------- |
|        | `67.78 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-affine-g2:default

|        | `2024570 bytes`           |
|:-------|:------------------------- |
|        | `57.58 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g1:default

|        | `2026854 bytes`           |
|:-------|:------------------------- |
|        | `71.52 ms` (✅ **1.00x**)  |

### compile:bls12-381-mul-projective-g2:default

|        | `2024738 bytes`           |
|:-------|:------------------------- |
|        | `72.91 ms` (✅ **1.00x**)  |

### compile:bls12-377-pairing:default

|        | `2146613 bytes`            |
|:-------|:-------------------------- |
|        | `227.77 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-10:default

|        | `2057654 bytes`           |
|:-------|:------------------------- |
|        | `96.14 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g1-1000:default

|        | `2057659 bytes`           |
|:-------|:------------------------- |
|        | `94.81 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-10:default

|        | `2053801 bytes`           |
|:-------|:------------------------- |
|        | `77.64 ms` (✅ **1.00x**)  |

### compile:bls12-377-msm-g2-1000:default

|        | `2053806 bytes`           |
|:-------|:------------------------- |
|        | `73.28 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g1:default

|        | `2026248 bytes`            |
|:-------|:-------------------------- |
|        | `100.40 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-affine-g2:default

|        | `2025398 bytes`           |
|:-------|:------------------------- |
|        | `52.32 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g1:default

|        | `2026749 bytes`           |
|:-------|:------------------------- |
|        | `66.90 ms` (✅ **1.00x**)  |

### compile:bls12-377-mul-projective-g2:default

|        | `2025572 bytes`           |
|:-------|:------------------------- |
|        | `50.11 ms` (✅ **1.00x**)  |

### compile:bw6-761-pairing:default

|        | `2115159 bytes`            |
|:-------|:-------------------------- |
|        | `123.01 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-10:default

|        | `2082916 bytes`            |
|:-------|:-------------------------- |
|        | `153.42 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g1-1000:default

|        | `2082922 bytes`            |
|:-------|:-------------------------- |
|        | `137.18 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-10:default

|        | `2082920 bytes`            |
|:-------|:-------------------------- |
|        | `116.53 ms` (✅ **1.00x**)  |

### compile:bw6-761-msm-g2-1000:default

|        | `2082921 bytes`            |
|:-------|:-------------------------- |
|        | `127.08 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g1:default

|        | `2052264 bytes`            |
|:-------|:-------------------------- |
|        | `133.37 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-affine-g2:default

|        | `2052264 bytes`            |
|:-------|:-------------------------- |
|        | `114.61 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g1:default

|        | `2052467 bytes`            |
|:-------|:-------------------------- |
|        | `148.33 ms` (✅ **1.00x**)  |

### compile:bw6-761-mul-projective-g2:default

|        | `2052467 bytes`            |
|:-------|:-------------------------- |
|        | `142.45 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-10:default

|        | `2057226 bytes`           |
|:-------|:------------------------- |
|        | `65.90 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-sw-1000:default

|        | `2057231 bytes`           |
|:-------|:------------------------- |
|        | `66.54 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-10:default

|        | `2046406 bytes`           |
|:-------|:------------------------- |
|        | `90.92 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-msm-te-1000:default

|        | `2046407 bytes`            |
|:-------|:-------------------------- |
|        | `116.62 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-sw:default

|        | `2024113 bytes`           |
|:-------|:------------------------- |
|        | `84.34 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-affine-te:default

|        | `2024116 bytes`           |
|:-------|:------------------------- |
|        | `77.44 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-sw:default

|        | `2024113 bytes`           |
|:-------|:------------------------- |
|        | `98.46 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-381-mul-projective-te:default

|        | `2028300 bytes`           |
|:-------|:------------------------- |
|        | `86.25 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-affine:default

|        | `2045524 bytes`            |
|:-------|:-------------------------- |
|        | `110.92 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-mul-projective:default

|        | `2028426 bytes`           |
|:-------|:------------------------- |
|        | `68.19 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-10:default

|        | `2046587 bytes`           |
|:-------|:------------------------- |
|        | `86.00 ms` (✅ **1.00x**)  |

### compile:ed-on-bls12-377-msm-1000:default

|        | `2046592 bytes`           |
|:-------|:------------------------- |
|        | `86.11 ms` (✅ **1.00x**)  |

### exec:groth16:default

|        | `2209532 bytes`           |
|:-------|:------------------------- |
|        | `56.98 ms` (✅ **1.00x**)  |

### exec:bls12-381-pairing:default

|        | `2115526 bytes`           |
|:-------|:------------------------- |
|        | `19.04 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-10:default

|        | `2057684 bytes`            |
|:-------|:-------------------------- |
|        | `737.74 us` (✅ **1.00x**)  |

### exec:bls12-381-msm-g1-1000:default

|        | `2057689 bytes`           |
|:-------|:------------------------- |
|        | `14.88 ms` (✅ **1.00x**)  |

### exec:bls12-381-msm-g2-10:default

|        | `2052877 bytes`           |
|:-------|:------------------------- |
|        | `1.09 ms` (✅ **1.00x**)   |

### exec:bls12-381-msm-g2-1000:default

|        | `2052882 bytes`           |
|:-------|:------------------------- |
|        | `36.54 ms` (✅ **1.00x**)  |

### exec:bls12-381-mul-affine-g1:default

|        | `2026347 bytes`           |
|:-------|:------------------------- |
|        | `29.70 us` (✅ **1.00x**)  |

### exec:bls12-381-mul-affine-g2:default

|        | `2024570 bytes`           |
|:-------|:------------------------- |
|        | `37.31 us` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g1:default

|        | `2026854 bytes`            |
|:-------|:-------------------------- |
|        | `101.07 us` (✅ **1.00x**)  |

### exec:bls12-381-mul-projective-g2:default

|        | `2024738 bytes`           |
|:-------|:------------------------- |
|        | `37.74 us` (✅ **1.00x**)  |

### exec:bls12-377-pairing:default

|        | `2146613 bytes`           |
|:-------|:------------------------- |
|        | `18.66 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-10:default

|        | `2057654 bytes`            |
|:-------|:-------------------------- |
|        | `576.27 us` (✅ **1.00x**)  |

### exec:bls12-377-msm-g1-1000:default

|        | `2057659 bytes`           |
|:-------|:------------------------- |
|        | `14.52 ms` (✅ **1.00x**)  |

### exec:bls12-377-msm-g2-10:default

|        | `2053801 bytes`           |
|:-------|:------------------------- |
|        | `1.35 ms` (✅ **1.00x**)   |

### exec:bls12-377-msm-g2-1000:default

|        | `2053806 bytes`           |
|:-------|:------------------------- |
|        | `37.88 ms` (✅ **1.00x**)  |

### exec:bls12-377-mul-affine-g1:default

|        | `2026248 bytes`           |
|:-------|:------------------------- |
|        | `61.45 us` (✅ **1.00x**)  |

### exec:bls12-377-mul-affine-g2:default

|        | `2025398 bytes`           |
|:-------|:------------------------- |
|        | `37.10 us` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g1:default

|        | `2026749 bytes`           |
|:-------|:------------------------- |
|        | `31.59 us` (✅ **1.00x**)  |

### exec:bls12-377-mul-projective-g2:default

|        | `2025572 bytes`           |
|:-------|:------------------------- |
|        | `38.19 us` (✅ **1.00x**)  |

### exec:bw6-761-pairing:default

|        | `2115159 bytes`           |
|:-------|:------------------------- |
|        | `78.30 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g1-10:default

|        | `2082916 bytes`           |
|:-------|:------------------------- |
|        | `1.60 ms` (✅ **1.00x**)   |

### exec:bw6-761-msm-g1-1000:default

|        | `2082922 bytes`           |
|:-------|:------------------------- |
|        | `51.30 ms` (✅ **1.00x**)  |

### exec:bw6-761-msm-g2-10:default

|        | `2082920 bytes`           |
|:-------|:------------------------- |
|        | `1.45 ms` (✅ **1.00x**)   |

### exec:bw6-761-msm-g2-1000:default

|        | `2082921 bytes`           |
|:-------|:------------------------- |
|        | `46.25 ms` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g1:default

|        | `2052264 bytes`           |
|:-------|:------------------------- |
|        | `44.28 us` (✅ **1.00x**)  |

### exec:bw6-761-mul-affine-g2:default

|        | `2052264 bytes`           |
|:-------|:------------------------- |
|        | `44.84 us` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g1:default

|        | `2052467 bytes`           |
|:-------|:------------------------- |
|        | `44.30 us` (✅ **1.00x**)  |

### exec:bw6-761-mul-projective-g2:default

|        | `2052467 bytes`           |
|:-------|:------------------------- |
|        | `44.84 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-10:default

|        | `2057226 bytes`            |
|:-------|:-------------------------- |
|        | `345.06 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-msm-sw-1000:default

|        | `2057231 bytes`           |
|:-------|:------------------------- |
|        | `8.32 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-381-msm-te-10:default

|        | `2046406 bytes`           |
|:-------|:------------------------- |
|        | `6.54 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-381-msm-te-1000:default

|        | `2046407 bytes`           |
|:-------|:------------------------- |
|        | `72.86 ms` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-sw:default

|        | `2024113 bytes`           |
|:-------|:------------------------- |
|        | `36.63 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-affine-te:default

|        | `2024116 bytes`           |
|:-------|:------------------------- |
|        | `30.05 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-sw:default

|        | `2024113 bytes`           |
|:-------|:------------------------- |
|        | `24.89 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-381-mul-projective-te:default

|        | `2028300 bytes`           |
|:-------|:------------------------- |
|        | `27.47 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-mul-affine:default

|        | `2045524 bytes`           |
|:-------|:------------------------- |
|        | `6.04 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-377-mul-projective:default

|        | `2028426 bytes`           |
|:-------|:------------------------- |
|        | `27.30 us` (✅ **1.00x**)  |

### exec:ed-on-bls12-377-msm-10:default

|        | `2046587 bytes`           |
|:-------|:------------------------- |
|        | `6.07 ms` (✅ **1.00x**)   |

### exec:ed-on-bls12-377-msm-1000:default

|        | `2046592 bytes`           |
|:-------|:------------------------- |
|        | `65.89 ms` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

