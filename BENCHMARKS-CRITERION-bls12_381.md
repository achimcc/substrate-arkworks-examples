# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample](#sample)
    - [arithmetic](#arithmetic)
    - [serialization](#serialization)
    - [msm](#msm)
    - [squareroot](#squareroot)
    - [bitwise](#bitwise)
    - [conversions](#conversions)

## Benchmark Results

### sample

|        | `bls12_381::g1projective elements`          | `bls12_381::g2projective elements`           |
|:-------|:--------------------------------------------|:-------------------------------------------- |
|        | `206.60 us` (✅ **1.00x**)                   | `1.86 ms` (❌ *8.98x slower*)                 |

### arithmetic

|                                       | `for bls12_381::fr::bigint`          | `for bls12_381::fq::bigint`          | `for bls12_381::g1projective`          | `for bls12_381::g2projective`          | `for bls12_381::fq2`             | `for bls12_381::fq12`             | `for bls12_381::fq`               | `for bls12_381::fr`               |
|:--------------------------------------|:-------------------------------------|:-------------------------------------|:---------------------------------------|:---------------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                                | `N/A`                                | `1.26 us` (✅ **1.00x**)                | `4.16 us` (❌ *3.29x slower*)           | `23.09 ns` (🚀 **54.76x faster**) | `202.41 ns` (🚀 **6.25x faster**)  | `12.44 ns` (🚀 **101.61x faster**) | `8.81 ns` (🚀 **143.56x faster**)  |
| **`subtraction`**                     | `N/A`                                | `N/A`                                | `1.30 us` (✅ **1.00x**)                | `4.21 us` (❌ *3.24x slower*)           | `23.38 ns` (🚀 **55.54x faster**) | `185.79 ns` (🚀 **6.99x faster**)  | `13.39 ns` (🚀 **96.96x faster**)  | `9.73 ns` (🚀 **133.50x faster**)  |
| **`mixed addition`**                  | `N/A`                                | `N/A`                                | `889.18 ns` (✅ **1.00x**)              | `2.98 us` (❌ *3.35x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed subtraction`**               | `N/A`                                | `N/A`                                | `928.01 ns` (✅ **1.00x**)              | `3.02 us` (❌ *3.25x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                                | `N/A`                                | `589.25 ns` (✅ **1.00x**)              | `1.86 us` (❌ *3.15x slower*)           | `12.56 ns` (🚀 **46.91x faster**) | `152.53 ns` (🚀 **3.86x faster**)  | `7.40 ns` (🚀 **79.58x faster**)   | `5.86 ns` (🚀 **100.62x faster**)  |
| **`scalar multiplication`**           | `N/A`                                | `N/A`                                | `328.93 us` (✅ **1.00x**)              | `999.44 us` (❌ *3.04x slower*)         | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `22.43 ns` (❌ *3.57x slower*)    | `116.58 ns` (❌ *18.56x slower*)   | `18.58 ns` (❌ *2.96x slower*)     | `6.28 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `255.44 ns` (❌ *5.52x slower*)   | `6.51 us` (❌ *140.74x slower*)    | `77.84 ns` (❌ *1.68x slower*)     | `46.25 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `173.12 ns` (❌ *4.59x slower*)   | `4.57 us` (❌ *121.02x slower*)    | `66.67 ns` (❌ *1.77x slower*)     | `37.76 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `15.28 us` (❌ *2.04x slower*)    | `25.92 us` (❌ *3.47x slower*)     | `14.95 us` (❌ *2.00x slower*)     | `7.48 us` (✅ **1.00x**)           |
| **`sum of products of size 2`**       | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `550.67 ns` (❌ *6.31x slower*)   | `13.34 us` (❌ *152.85x slower*)   | `126.06 ns` (❌ *1.44x slower*)    | `87.26 ns` (✅ **1.00x**)          |
| **`naive sum of products of size 2`** | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `541.71 ns` (❌ *6.15x slower*)   | `13.20 us` (❌ *149.75x slower*)   | `163.54 ns` (❌ *1.86x slower*)    | `88.13 ns` (✅ **1.00x**)          |
| **`addition with carry`**             | `7.64 ns` (✅ **1.00x**)              | `8.74 ns` (❌ *1.14x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction with borrow`**         | `8.71 ns` (✅ **1.00x**)              | `9.98 ns` (❌ *1.15x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication by 2`**             | `4.73 ns` (✅ **1.00x**)              | `4.87 ns` (✅ **1.03x slower**)       | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division by 2`**                   | `4.80 ns` (✅ **1.00x**)              | `4.80 ns` (✅ **1.00x faster**)       | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization

|                                          | `for bls12_381::g1projective`          | `for bls12_381::g2projective`          | `for bls12_381::fr`                | `for bls12_381::fq`                 | `for bls12_381::fq2`               | `for bls12_381::fq12`             |
|:-----------------------------------------|:---------------------------------------|:---------------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize compressed`**               | `163.84 ns` (✅ **1.00x**)              | `226.00 ns` (❌ *1.38x slower*)         | `29.52 ns` (🚀 **5.55x faster**)    | `56.32 ns` (🚀 **2.91x faster**)     | `117.54 ns` (✅ **1.39x faster**)   | `694.97 ns` (❌ *4.24x slower*)    |
| **`serialize uncompressed`**             | `214.06 ns` (✅ **1.00x**)              | `324.27 ns` (❌ *1.51x slower*)         | `29.51 ns` (🚀 **7.25x faster**)    | `56.46 ns` (🚀 **3.79x faster**)     | `117.50 ns` (🚀 **1.82x faster**)   | `694.30 ns` (❌ *3.24x slower*)    |
| **`deserialize compressed`**             | `134.17 us` (✅ **1.00x**)              | `268.07 us` (❌ *2.00x slower*)         | `58.26 ns` (🚀 **2303.07x faster**) | `101.68 ns` (🚀 **1319.58x faster**) | `234.49 ns` (🚀 **572.19x faster**) | `1.39 us` (🚀 **96.42x faster**)   |
| **`deserialize compressed unchecked`**   | `39.76 us` (✅ **1.00x**)               | `135.15 us` (❌ *3.40x slower*)         | `58.25 ns` (🚀 **682.59x faster**)  | `101.67 ns` (🚀 **391.05x faster**)  | `237.49 ns` (🚀 **167.41x faster**) | `1.37 us` (🚀 **29.04x faster**)   |
| **`deserialize uncompressed`**           | `94.49 us` (✅ **1.00x**)               | `132.62 us` (❌ *1.40x slower*)         | `58.18 ns` (🚀 **1624.11x faster**) | `101.57 ns` (🚀 **930.24x faster**)  | `234.83 ns` (🚀 **402.35x faster**) | `1.38 us` (🚀 **68.50x faster**)   |
| **`deserialize uncompressed unchecked`** | `193.20 ns` (✅ **1.00x**)              | `441.37 ns` (❌ *2.28x slower*)         | `58.22 ns` (🚀 **3.32x faster**)    | `101.65 ns` (🚀 **1.90x faster**)    | `234.58 ns` (❌ *1.21x slower*)     | `1.38 us` (❌ *7.12x slower*)      |

### msm

|        | `for bls12_381::g1projective`          | `for bls12_381::g2projective`           |
|:-------|:---------------------------------------|:--------------------------------------- |
|        | `2.39 s` (✅ **1.00x**)                 | `7.35 s` (❌ *3.07x slower*)             |

### squareroot

|                          | `for bls12_381::fr`          | `for bls12_381::fq`             | `for bls12_381::fq2`              |
|:-------------------------|:-----------------------------|:--------------------------------|:--------------------------------- |
| **`square root for qr`** | `25.49 us` (✅ **1.00x**)     | `39.30 us` (❌ *1.54x slower*)   | `134.03 us` (❌ *5.26x slower*)    |
| **`legendre for qr`**    | `14.26 us` (✅ **1.00x**)     | `39.22 us` (❌ *2.75x slower*)   | `39.64 us` (❌ *2.78x slower*)     |

### bitwise

|                               | `operations for bls12_381::fr::bigint`          | `operations for bls12_381::fq::bigint`           |
|:------------------------------|:------------------------------------------------|:------------------------------------------------ |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                         | `5.03 ns` (✅ **1.04x slower**)                   |
| **`from little-endian bits`** | `138.50 ns` (✅ **1.00x**)                       | `189.44 ns` (❌ *1.37x slower*)                   |
| **`from big-endian bits`**    | `138.38 ns` (✅ **1.00x**)                       | `189.38 ns` (❌ *1.37x slower*)                   |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                         | `5.08 ns` (✅ **1.04x slower**)                   |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)                         | `5.76 ns` (✅ **1.06x slower**)                   |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                         | `5.21 ns` (✅ **1.06x slower**)                   |

### conversions

|                   | `for bls12_381::fr`          | `for bls12_381::fq`              |
|:------------------|:-----------------------------|:-------------------------------- |
| **`from bigint`** | `40.27 ns` (✅ **1.00x**)     | `76.68 ns` (❌ *1.90x slower*)    |
| **`into bigint`** | `26.54 ns` (✅ **1.00x**)     | `47.71 ns` (❌ *1.80x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

