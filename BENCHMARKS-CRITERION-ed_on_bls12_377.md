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
|        | `59.49 us` (✅ **1.00x**)              |

### arithmetic

|                                       | `for edonbls12_377::fr::bigint`          | `for edonbls12_377::fq::bigint`          | `for edonbls12_377::g`          | `for edonbls12_377::fq`          | `for edonbls12_377::fr`           |
|:--------------------------------------|:-----------------------------------------|:-----------------------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                                    | `N/A`                                    | `436.36 ns` (✅ **1.00x**)       | `8.81 ns` (🚀 **49.54x faster**)  | `8.82 ns` (🚀 **49.47x faster**)   |
| **`subtraction`**                     | `N/A`                                    | `N/A`                                    | `451.33 ns` (✅ **1.00x**)       | `9.02 ns` (🚀 **50.04x faster**)  | `9.01 ns` (🚀 **50.12x faster**)   |
| **`mixed addition`**                  | `N/A`                                    | `N/A`                                    | `429.92 ns` (✅ **1.00x**)       | `N/A`                            | `N/A`                             |
| **`mixed subtraction`**               | `N/A`                                    | `N/A`                                    | `436.60 ns` (✅ **1.00x**)       | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                                    | `N/A`                                    | `322.06 ns` (✅ **1.00x**)       | `5.82 ns` (🚀 **55.34x faster**)  | `5.86 ns` (🚀 **54.98x faster**)   |
| **`scalar multiplication`**           | `N/A`                                    | `N/A`                                    | `143.99 us` (✅ **1.00x**)       | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                                    | `N/A`                                    | `N/A`                           | `6.24 ns` (✅ **1.01x faster**)   | `6.28 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                                    | `N/A`                                    | `N/A`                           | `43.40 ns` (✅ **1.01x slower**)  | `42.88 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                                    | `N/A`                                    | `N/A`                           | `36.13 ns` (✅ **1.03x slower**)  | `34.92 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                                    | `N/A`                                    | `N/A`                           | `7.31 us` (✅ **1.01x slower**)   | `7.27 us` (✅ **1.00x**)           |
| **`sum of products of size 2`**       | `N/A`                                    | `N/A`                                    | `N/A`                           | `61.66 ns` (✅ **1.02x slower**)  | `60.35 ns` (✅ **1.00x**)          |
| **`naive sum of products of size 2`** | `N/A`                                    | `N/A`                                    | `N/A`                           | `89.83 ns` (✅ **1.01x slower**)  | `88.99 ns` (✅ **1.00x**)          |
| **`addition with carry`**             | `7.61 ns` (✅ **1.00x**)                  | `7.61 ns` (✅ **1.00x faster**)           | `N/A`                           | `N/A`                            | `N/A`                             |
| **`subtraction with borrow`**         | `8.75 ns` (✅ **1.00x**)                  | `8.74 ns` (✅ **1.00x faster**)           | `N/A`                           | `N/A`                            | `N/A`                             |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)                  | `4.69 ns` (✅ **1.00x slower**)           | `N/A`                           | `N/A`                            | `N/A`                             |
| **`division by 2`**                   | `4.58 ns` (✅ **1.00x**)                  | `4.56 ns` (✅ **1.01x faster**)           | `N/A`                           | `N/A`                            | `N/A`                             |

### serialization

|                                          | `for edonbls12_377::g`          | `for edonbls12_377::fr`            | `for edonbls12_377::fq`             |
|:-----------------------------------------|:--------------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize compressed`**               | `85.30 ns` (✅ **1.00x**)        | `28.43 ns` (🚀 **3.00x faster**)    | `28.59 ns` (🚀 **2.98x faster**)     |
| **`serialize uncompressed`**             | `108.82 ns` (✅ **1.00x**)       | `28.46 ns` (🚀 **3.82x faster**)    | `28.60 ns` (🚀 **3.80x faster**)     |
| **`deserialize compressed`**             | `188.69 us` (✅ **1.00x**)       | `57.63 ns` (🚀 **3274.15x faster**) | `56.10 ns` (🚀 **3363.50x faster**)  |
| **`deserialize compressed unchecked`**   | `39.01 us` (✅ **1.00x**)        | `57.63 ns` (🚀 **676.95x faster**)  | `56.11 ns` (🚀 **695.30x faster**)   |
| **`deserialize uncompressed`**           | `149.53 us` (✅ **1.00x**)       | `57.60 ns` (🚀 **2595.77x faster**) | `56.09 ns` (🚀 **2665.67x faster**)  |
| **`deserialize uncompressed unchecked`** | `169.62 ns` (✅ **1.00x**)       | `57.61 ns` (🚀 **2.94x faster**)    | `56.12 ns` (🚀 **3.02x faster**)     |

### msm

|        | `for edonbls12_377::g`           |
|:-------|:-------------------------------- |
|        | `1.36 s` (✅ **1.00x**)           |

### squareroot

|                          | `for edonbls12_377::fr`          | `for edonbls12_377::fq`           |
|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square root for qr`** | `12.10 us` (✅ **1.00x**)         | `31.22 us` (❌ *2.58x slower*)     |
| **`legendre for qr`**    | `12.22 us` (✅ **1.00x**)         | `11.02 us` (✅ **1.11x faster**)   |

### bitwise

|                               | `operations for edonbls12_377::fr::bigint`          | `operations for edonbls12_377::fq::bigint`           |
|:------------------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                             | `4.85 ns` (✅ **1.00x faster**)                       |
| **`from little-endian bits`** | `130.88 ns` (✅ **1.00x**)                           | `130.97 ns` (✅ **1.00x slower**)                     |
| **`from big-endian bits`**    | `130.92 ns` (✅ **1.00x**)                           | `130.87 ns` (✅ **1.00x faster**)                     |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                             | `4.91 ns` (✅ **1.00x slower**)                       |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)                             | `5.42 ns` (✅ **1.00x faster**)                       |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                             | `4.91 ns` (✅ **1.00x slower**)                       |

### conversions

|                   | `for edonbls12_377::fr`          | `for edonbls12_377::fq`           |
|:------------------|:---------------------------------|:--------------------------------- |
| **`from bigint`** | `40.26 ns` (✅ **1.00x**)         | `40.66 ns` (✅ **1.01x slower**)   |
| **`into bigint`** | `25.83 ns` (✅ **1.00x**)         | `26.53 ns` (✅ **1.03x slower**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

