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

|        | `edonbls12_381_optimized::goptimized elements`           |
|:-------|:-------------------------------------------------------- |
|        | `146.17 us` (✅ **1.00x**)                                |

### arithmetic

|                                       | `for edonbls12_381_optimized::froptimized::bigint`          | `for edonbls12_381_optimized::fqoptimized::bigint`          | `for edonbls12_381_optimized::goptimized`          | `for edonbls12_381_optimized::fqoptimized`          | `for edonbls12_381_optimized::froptimized`           |
|:--------------------------------------|:------------------------------------------------------------|:------------------------------------------------------------|:---------------------------------------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`addition`**                        | `N/A`                                                       | `N/A`                                                       | `425.94 ns` (✅ **1.00x**)                          | `8.80 ns` (🚀 **48.40x faster**)                     | `8.81 ns` (🚀 **48.37x faster**)                      |
| **`subtraction`**                     | `N/A`                                                       | `N/A`                                                       | `442.75 ns` (✅ **1.00x**)                          | `9.01 ns` (🚀 **49.14x faster**)                     | `9.01 ns` (🚀 **49.16x faster**)                      |
| **`mixed addition`**                  | `N/A`                                                       | `N/A`                                                       | `434.39 ns` (✅ **1.00x**)                          | `N/A`                                               | `N/A`                                                |
| **`mixed subtraction`**               | `N/A`                                                       | `N/A`                                                       | `433.74 ns` (✅ **1.00x**)                          | `N/A`                                               | `N/A`                                                |
| **`double`**                          | `N/A`                                                       | `N/A`                                                       | `324.72 ns` (✅ **1.00x**)                          | `5.84 ns` (🚀 **55.65x faster**)                     | `5.84 ns` (🚀 **55.64x faster**)                      |
| **`scalar multiplication`**           | `N/A`                                                       | `N/A`                                                       | `222.05 us` (✅ **1.00x**)                          | `N/A`                                               | `N/A`                                                |
| **`negation`**                        | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `6.29 ns` (✅ **1.00x faster**)                      | `6.29 ns` (✅ **1.00x**)                              |
| **`multiplication`**                  | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `43.92 ns` (✅ **1.01x slower**)                     | `43.30 ns` (✅ **1.00x**)                             |
| **`square`**                          | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `36.97 ns` (✅ **1.05x slower**)                     | `35.17 ns` (✅ **1.00x**)                             |
| **`inverse`**                         | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `7.35 us` (✅ **1.00x slower**)                      | `7.31 us` (✅ **1.00x**)                              |
| **`sum of products of size 2`**       | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `61.68 ns` (✅ **1.03x slower**)                     | `60.14 ns` (✅ **1.00x**)                             |
| **`naive sum of products of size 2`** | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `89.80 ns` (✅ **1.01x slower**)                     | `89.01 ns` (✅ **1.00x**)                             |
| **`addition with carry`**             | `7.61 ns` (✅ **1.00x**)                                     | `7.61 ns` (✅ **1.00x faster**)                              | `N/A`                                              | `N/A`                                               | `N/A`                                                |
| **`subtraction with borrow`**         | `8.75 ns` (✅ **1.00x**)                                     | `8.73 ns` (✅ **1.00x faster**)                              | `N/A`                                              | `N/A`                                               | `N/A`                                                |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)                                     | `4.69 ns` (✅ **1.00x faster**)                              | `N/A`                                              | `N/A`                                               | `N/A`                                                |
| **`division by 2`**                   | `4.65 ns` (✅ **1.00x**)                                     | `4.56 ns` (✅ **1.02x faster**)                              | `N/A`                                              | `N/A`                                               | `N/A`                                                |

### serialization

|                                          | `for edonbls12_381_optimized::goptimized`          | `for edonbls12_381_optimized::froptimized`          | `for edonbls12_381_optimized::fqoptimized`           |
|:-----------------------------------------|:---------------------------------------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`serialize compressed`**               | `83.69 ns` (✅ **1.00x**)                           | `29.68 ns` (🚀 **2.82x faster**)                     | `28.76 ns` (🚀 **2.91x faster**)                      |
| **`serialize uncompressed`**             | `102.89 ns` (✅ **1.00x**)                          | `29.19 ns` (🚀 **3.53x faster**)                     | `28.75 ns` (🚀 **3.58x faster**)                      |
| **`deserialize compressed`**             | `224.84 us` (✅ **1.00x**)                          | `58.61 ns` (🚀 **3836.12x faster**)                  | `57.01 ns` (🚀 **3943.58x faster**)                   |
| **`deserialize compressed unchecked`**   | `39.09 us` (✅ **1.00x**)                           | `58.59 ns` (🚀 **667.21x faster**)                   | `57.06 ns` (🚀 **685.05x faster**)                    |
| **`deserialize uncompressed`**           | `185.69 us` (✅ **1.00x**)                          | `58.56 ns` (🚀 **3170.95x faster**)                  | `56.99 ns` (🚀 **3258.07x faster**)                   |
| **`deserialize uncompressed unchecked`** | `168.83 ns` (✅ **1.00x**)                          | `58.57 ns` (🚀 **2.88x faster**)                     | `57.01 ns` (🚀 **2.96x faster**)                      |

### msm

|        | `for edonbls12_381_optimized::goptimized`           |
|:-------|:--------------------------------------------------- |
|        | `1.40 s` (✅ **1.00x**)                              |

### squareroot

|                          | `for edonbls12_381_optimized::froptimized`          | `for edonbls12_381_optimized::fqoptimized`           |
|:-------------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`square root for qr`** | `12.10 us` (✅ **1.00x**)                            | `31.24 us` (❌ *2.58x slower*)                        |
| **`legendre for qr`**    | `12.27 us` (✅ **1.00x**)                            | `10.99 us` (✅ **1.12x faster**)                      |

### bitwise

|                               | `operations for edonbls12_381_optimized::froptimized::bigint`          | `operations for edonbls12_381_optimized::fqoptimized::bigint`           |
|:------------------------------|:-----------------------------------------------------------------------|:----------------------------------------------------------------------- |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                                                | `4.85 ns` (✅ **1.00x slower**)                                          |
| **`from little-endian bits`** | `130.64 ns` (✅ **1.00x**)                                              | `131.69 ns` (✅ **1.01x slower**)                                        |
| **`from big-endian bits`**    | `130.54 ns` (✅ **1.00x**)                                              | `130.72 ns` (✅ **1.00x slower**)                                        |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                                                | `4.91 ns` (✅ **1.00x slower**)                                          |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)                                                | `5.45 ns` (✅ **1.00x slower**)                                          |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                                                | `4.90 ns` (✅ **1.00x slower**)                                          |

### conversions

|                   | `for edonbls12_381_optimized::froptimized`          | `for edonbls12_381_optimized::fqoptimized`           |
|:------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`from bigint`** | `40.78 ns` (✅ **1.00x**)                            | `40.41 ns` (✅ **1.01x faster**)                      |
| **`into bigint`** | `25.40 ns` (✅ **1.00x**)                            | `25.64 ns` (✅ **1.01x slower**)                      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

