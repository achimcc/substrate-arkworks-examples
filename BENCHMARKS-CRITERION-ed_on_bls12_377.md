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

|        | `edonbls12_377::g elements`           |
|:-------|:------------------------------------- |
|        | `59.54 us` (✅ **1.00x**)              |

### arithmetic

|                                       | `for edonbls12_377::fr::bigint`          | `for edonbls12_377::fq::bigint`          | `for edonbls12_377::g`          | `for edonbls12_377::fq`          | `for edonbls12_377::fr`           |
|:--------------------------------------|:-----------------------------------------|:-----------------------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                                    | `N/A`                                    | `435.43 ns` (✅ **1.00x**)       | `8.81 ns` (🚀 **49.42x faster**)  | `8.82 ns` (🚀 **49.36x faster**)   |
| **`subtraction`**                     | `N/A`                                    | `N/A`                                    | `451.70 ns` (✅ **1.00x**)       | `9.02 ns` (🚀 **50.09x faster**)  | `9.04 ns` (🚀 **49.99x faster**)   |
| **`mixed addition`**                  | `N/A`                                    | `N/A`                                    | `433.03 ns` (✅ **1.00x**)       | `N/A`                            | `N/A`                             |
| **`mixed subtraction`**               | `N/A`                                    | `N/A`                                    | `437.02 ns` (✅ **1.00x**)       | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                                    | `N/A`                                    | `322.57 ns` (✅ **1.00x**)       | `5.82 ns` (🚀 **55.42x faster**)  | `5.87 ns` (🚀 **54.95x faster**)   |
| **`scalar multiplication`**           | `N/A`                                    | `N/A`                                    | `144.03 us` (✅ **1.00x**)       | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                                    | `N/A`                                    | `N/A`                           | `6.26 ns` (✅ **1.00x faster**)   | `6.28 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                                    | `N/A`                                    | `N/A`                           | `43.27 ns` (✅ **1.01x slower**)  | `42.84 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                                    | `N/A`                                    | `N/A`                           | `36.12 ns` (✅ **1.04x slower**)  | `34.88 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                                    | `N/A`                                    | `N/A`                           | `7.32 us` (✅ **1.01x slower**)   | `7.28 us` (✅ **1.00x**)           |
| **`sum of products of size 2`**       | `N/A`                                    | `N/A`                                    | `N/A`                           | `61.60 ns` (✅ **1.02x slower**)  | `60.53 ns` (✅ **1.00x**)          |
| **`naive sum of products of size 2`** | `N/A`                                    | `N/A`                                    | `N/A`                           | `89.84 ns` (✅ **1.01x slower**)  | `89.04 ns` (✅ **1.00x**)          |
| **`addition with carry`**             | `7.61 ns` (✅ **1.00x**)                  | `7.61 ns` (✅ **1.00x slower**)           | `N/A`                           | `N/A`                            | `N/A`                             |
| **`subtraction with borrow`**         | `8.74 ns` (✅ **1.00x**)                  | `8.74 ns` (✅ **1.00x faster**)           | `N/A`                           | `N/A`                            | `N/A`                             |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)                  | `4.69 ns` (✅ **1.00x faster**)           | `N/A`                           | `N/A`                            | `N/A`                             |
| **`division by 2`**                   | `4.56 ns` (✅ **1.00x**)                  | `4.57 ns` (✅ **1.00x slower**)           | `N/A`                           | `N/A`                            | `N/A`                             |

### serialization

|                                          | `for edonbls12_377::g`          | `for edonbls12_377::fr`            | `for edonbls12_377::fq`             |
|:-----------------------------------------|:--------------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize compressed`**               | `85.37 ns` (✅ **1.00x**)        | `28.41 ns` (🚀 **3.01x faster**)    | `28.62 ns` (🚀 **2.98x faster**)     |
| **`serialize uncompressed`**             | `108.94 ns` (✅ **1.00x**)       | `28.41 ns` (🚀 **3.83x faster**)    | `28.64 ns` (🚀 **3.80x faster**)     |
| **`deserialize compressed`**             | `189.15 us` (✅ **1.00x**)       | `57.75 ns` (🚀 **3275.55x faster**) | `56.25 ns` (🚀 **3362.57x faster**)  |
| **`deserialize compressed unchecked`**   | `39.02 us` (✅ **1.00x**)        | `57.75 ns` (🚀 **675.71x faster**)  | `56.28 ns` (🚀 **693.38x faster**)   |
| **`deserialize uncompressed`**           | `150.08 us` (✅ **1.00x**)       | `57.70 ns` (🚀 **2600.88x faster**) | `56.33 ns` (🚀 **2664.05x faster**)  |
| **`deserialize uncompressed unchecked`** | `169.60 ns` (✅ **1.00x**)       | `57.72 ns` (🚀 **2.94x faster**)    | `56.26 ns` (🚀 **3.01x faster**)     |

### msm

|        | `for edonbls12_377::g`           |
|:-------|:-------------------------------- |
|        | `1.36 s` (✅ **1.00x**)           |

### squareroot

|                          | `for edonbls12_377::fr`          | `for edonbls12_377::fq`           |
|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square root for qr`** | `12.11 us` (✅ **1.00x**)         | `31.24 us` (❌ *2.58x slower*)     |
| **`legendre for qr`**    | `12.23 us` (✅ **1.00x**)         | `11.03 us` (✅ **1.11x faster**)   |

### bitwise

|                               | `operations for edonbls12_377::fr::bigint`          | `operations for edonbls12_377::fq::bigint`           |
|:------------------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                             | `4.85 ns` (✅ **1.00x faster**)                       |
| **`from little-endian bits`** | `131.14 ns` (✅ **1.00x**)                           | `131.06 ns` (✅ **1.00x faster**)                     |
| **`from big-endian bits`**    | `131.14 ns` (✅ **1.00x**)                           | `131.54 ns` (✅ **1.00x slower**)                     |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                             | `4.91 ns` (✅ **1.00x faster**)                       |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)                             | `5.42 ns` (✅ **1.00x faster**)                       |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                             | `4.90 ns` (✅ **1.00x slower**)                       |

### conversions

|                   | `for edonbls12_377::fr`          | `for edonbls12_377::fq`           |
|:------------------|:---------------------------------|:--------------------------------- |
| **`from bigint`** | `40.30 ns` (✅ **1.00x**)         | `40.65 ns` (✅ **1.01x slower**)   |
| **`into bigint`** | `26.05 ns` (✅ **1.00x**)         | `26.30 ns` (✅ **1.01x slower**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

