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
|        | `146.00 us` (✅ **1.00x**)                                |

### arithmetic

|                                       | `for edonbls12_381_optimized::froptimized::bigint`          | `for edonbls12_381_optimized::fqoptimized::bigint`          | `for edonbls12_381_optimized::goptimized`          | `for edonbls12_381_optimized::fqoptimized`          | `for edonbls12_381_optimized::froptimized`           |
|:--------------------------------------|:------------------------------------------------------------|:------------------------------------------------------------|:---------------------------------------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`addition`**                        | `N/A`                                                       | `N/A`                                                       | `425.92 ns` (✅ **1.00x**)                          | `8.79 ns` (🚀 **48.44x faster**)                     | `8.80 ns` (🚀 **48.40x faster**)                      |
| **`subtraction`**                     | `N/A`                                                       | `N/A`                                                       | `442.54 ns` (✅ **1.00x**)                          | `9.02 ns` (🚀 **49.08x faster**)                     | `9.03 ns` (🚀 **49.03x faster**)                      |
| **`mixed addition`**                  | `N/A`                                                       | `N/A`                                                       | `435.34 ns` (✅ **1.00x**)                          | `N/A`                                               | `N/A`                                                |
| **`mixed subtraction`**               | `N/A`                                                       | `N/A`                                                       | `433.85 ns` (✅ **1.00x**)                          | `N/A`                                               | `N/A`                                                |
| **`double`**                          | `N/A`                                                       | `N/A`                                                       | `324.86 ns` (✅ **1.00x**)                          | `5.83 ns` (🚀 **55.74x faster**)                     | `5.85 ns` (🚀 **55.54x faster**)                      |
| **`scalar multiplication`**           | `N/A`                                                       | `N/A`                                                       | `222.48 us` (✅ **1.00x**)                          | `N/A`                                               | `N/A`                                                |
| **`negation`**                        | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `6.29 ns` (✅ **1.00x faster**)                      | `6.29 ns` (✅ **1.00x**)                              |
| **`multiplication`**                  | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `43.98 ns` (✅ **1.02x slower**)                     | `43.27 ns` (✅ **1.00x**)                             |
| **`square`**                          | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `36.93 ns` (✅ **1.05x slower**)                     | `35.15 ns` (✅ **1.00x**)                             |
| **`inverse`**                         | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `7.34 us` (✅ **1.01x slower**)                      | `7.30 us` (✅ **1.00x**)                              |
| **`sum of products of size 2`**       | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `61.58 ns` (✅ **1.03x slower**)                     | `60.08 ns` (✅ **1.00x**)                             |
| **`naive sum of products of size 2`** | `N/A`                                                       | `N/A`                                                       | `N/A`                                              | `89.74 ns` (✅ **1.01x slower**)                     | `88.95 ns` (✅ **1.00x**)                             |
| **`addition with carry`**             | `7.61 ns` (✅ **1.00x**)                                     | `7.61 ns` (✅ **1.00x faster**)                              | `N/A`                                              | `N/A`                                               | `N/A`                                                |
| **`subtraction with borrow`**         | `8.75 ns` (✅ **1.00x**)                                     | `8.74 ns` (✅ **1.00x faster**)                              | `N/A`                                              | `N/A`                                               | `N/A`                                                |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)                                     | `4.69 ns` (✅ **1.00x faster**)                              | `N/A`                                              | `N/A`                                               | `N/A`                                                |
| **`division by 2`**                   | `4.74 ns` (✅ **1.00x**)                                     | `4.72 ns` (✅ **1.00x faster**)                              | `N/A`                                              | `N/A`                                               | `N/A`                                                |

### serialization

|                                          | `for edonbls12_381_optimized::goptimized`          | `for edonbls12_381_optimized::froptimized`          | `for edonbls12_381_optimized::fqoptimized`           |
|:-----------------------------------------|:---------------------------------------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`serialize compressed`**               | `83.56 ns` (✅ **1.00x**)                           | `29.04 ns` (🚀 **2.88x faster**)                     | `28.74 ns` (🚀 **2.91x faster**)                      |
| **`serialize uncompressed`**             | `102.74 ns` (✅ **1.00x**)                          | `29.09 ns` (🚀 **3.53x faster**)                     | `28.75 ns` (🚀 **3.57x faster**)                      |
| **`deserialize compressed`**             | `224.45 us` (✅ **1.00x**)                          | `58.39 ns` (🚀 **3843.85x faster**)                  | `56.96 ns` (🚀 **3940.69x faster**)                   |
| **`deserialize compressed unchecked`**   | `39.06 us` (✅ **1.00x**)                           | `58.40 ns` (🚀 **668.82x faster**)                   | `56.92 ns` (🚀 **686.25x faster**)                    |
| **`deserialize uncompressed`**           | `185.37 us` (✅ **1.00x**)                          | `58.21 ns` (🚀 **3184.49x faster**)                  | `56.75 ns` (🚀 **3266.63x faster**)                   |
| **`deserialize uncompressed unchecked`** | `168.88 ns` (✅ **1.00x**)                          | `58.20 ns` (🚀 **2.90x faster**)                     | `56.74 ns` (🚀 **2.98x faster**)                      |

### msm

|        | `for edonbls12_381_optimized::goptimized`           |
|:-------|:--------------------------------------------------- |
|        | `1.40 s` (✅ **1.00x**)                              |

### squareroot

|                          | `for edonbls12_381_optimized::froptimized`          | `for edonbls12_381_optimized::fqoptimized`           |
|:-------------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`square root for qr`** | `12.09 us` (✅ **1.00x**)                            | `31.23 us` (❌ *2.58x slower*)                        |
| **`legendre for qr`**    | `12.24 us` (✅ **1.00x**)                            | `10.99 us` (✅ **1.11x faster**)                      |

### bitwise

|                               | `operations for edonbls12_381_optimized::froptimized::bigint`          | `operations for edonbls12_381_optimized::fqoptimized::bigint`           |
|:------------------------------|:-----------------------------------------------------------------------|:----------------------------------------------------------------------- |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                                                | `4.84 ns` (✅ **1.00x faster**)                                          |
| **`from little-endian bits`** | `130.86 ns` (✅ **1.00x**)                                              | `130.31 ns` (✅ **1.00x faster**)                                        |
| **`from big-endian bits`**    | `131.59 ns` (✅ **1.00x**)                                              | `130.54 ns` (✅ **1.01x faster**)                                        |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                                                | `4.91 ns` (✅ **1.00x slower**)                                          |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)                                                | `5.45 ns` (✅ **1.00x faster**)                                          |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                                                | `4.90 ns` (✅ **1.00x faster**)                                          |

### conversions

|                   | `for edonbls12_381_optimized::froptimized`          | `for edonbls12_381_optimized::fqoptimized`           |
|:------------------|:----------------------------------------------------|:---------------------------------------------------- |
| **`from bigint`** | `40.80 ns` (✅ **1.00x**)                            | `40.49 ns` (✅ **1.01x faster**)                      |
| **`into bigint`** | `25.28 ns` (✅ **1.00x**)                            | `25.61 ns` (✅ **1.01x slower**)                      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

