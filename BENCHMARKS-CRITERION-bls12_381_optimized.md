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

|        | `bls12_381_optimized::g1projectivebls12_381 elements`          | `bls12_381_optimized::g2projectivebls12_381 elements`           |
|:-------|:---------------------------------------------------------------|:--------------------------------------------------------------- |
|        | `302.23 us` (✅ **1.00x**)                                      | `2.14 ms` (❌ *7.07x slower*)                                    |

### arithmetic

|                                       | `for bls12_381_optimized::froptimized::bigint`          | `for bls12_381_optimized::fqoptimized::bigint`          | `for bls12_381_optimized::g1projectivebls12_381`          | `for bls12_381_optimized::g2projectivebls12_381`          | `for bls12_381_optimized::fq2optimized`          | `for bls12_381_optimized::fq12optimized`          | `for bls12_381_optimized::fqoptimized`          | `for bls12_381_optimized::froptimized`           |
|:--------------------------------------|:--------------------------------------------------------|:--------------------------------------------------------|:----------------------------------------------------------|:----------------------------------------------------------|:-------------------------------------------------|:--------------------------------------------------|:------------------------------------------------|:------------------------------------------------ |
| **`addition`**                        | `N/A`                                                   | `N/A`                                                   | `1.24 us` (✅ **1.00x**)                                   | `4.04 us` (❌ *3.25x slower*)                              | `23.08 ns` (🚀 **53.80x faster**)                 | `205.30 ns` (🚀 **6.05x faster**)                  | `12.44 ns` (🚀 **99.84x faster**)                | `8.80 ns` (🚀 **141.11x faster**)                 |
| **`subtraction`**                     | `N/A`                                                   | `N/A`                                                   | `1.27 us` (✅ **1.00x**)                                   | `4.09 us` (❌ *3.22x slower*)                              | `23.13 ns` (🚀 **54.91x faster**)                 | `183.05 ns` (🚀 **6.94x faster**)                  | `13.26 ns` (🚀 **95.78x faster**)                | `9.00 ns` (🚀 **141.08x faster**)                 |
| **`mixed addition`**                  | `N/A`                                                   | `N/A`                                                   | `890.29 ns` (✅ **1.00x**)                                 | `2.90 us` (❌ *3.26x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`mixed subtraction`**               | `N/A`                                                   | `N/A`                                                   | `920.50 ns` (✅ **1.00x**)                                 | `2.95 us` (❌ *3.20x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`double`**                          | `N/A`                                                   | `N/A`                                                   | `608.61 ns` (✅ **1.00x**)                                 | `1.82 us` (❌ *2.99x slower*)                              | `12.45 ns` (🚀 **48.88x faster**)                 | `147.67 ns` (🚀 **4.12x faster**)                  | `7.42 ns` (🚀 **82.06x faster**)                 | `5.89 ns` (🚀 **103.30x faster**)                 |
| **`scalar multiplication`**           | `N/A`                                                   | `N/A`                                                   | `419.52 us` (✅ **1.00x**)                                 | `1.28 ms` (❌ *3.06x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`negation`**                        | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `22.13 ns` (❌ *3.51x slower*)                    | `117.11 ns` (❌ *18.59x slower*)                   | `18.38 ns` (❌ *2.92x slower*)                   | `6.30 ns` (✅ **1.00x**)                          |
| **`multiplication`**                  | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `255.44 ns` (❌ *5.50x slower*)                   | `6.51 us` (❌ *140.20x slower*)                    | `78.62 ns` (❌ *1.69x slower*)                   | `46.42 ns` (✅ **1.00x**)                         |
| **`square`**                          | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `170.52 ns` (❌ *4.55x slower*)                   | `4.58 us` (❌ *122.28x slower*)                    | `66.52 ns` (❌ *1.78x slower*)                   | `37.44 ns` (✅ **1.00x**)                         |
| **`inverse`**                         | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `15.72 us` (❌ *2.09x slower*)                    | `26.40 us` (❌ *3.50x slower*)                     | `15.38 us` (❌ *2.04x slower*)                   | `7.53 us` (✅ **1.00x**)                          |
| **`sum of products of size 2`**       | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `550.09 ns` (❌ *6.36x slower*)                   | `13.37 us` (❌ *154.55x slower*)                   | `126.65 ns` (❌ *1.46x slower*)                  | `86.50 ns` (✅ **1.00x**)                         |
| **`naive sum of products of size 2`** | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `540.31 ns` (❌ *6.12x slower*)                   | `13.31 us` (❌ *150.72x slower*)                   | `163.98 ns` (❌ *1.86x slower*)                  | `88.31 ns` (✅ **1.00x**)                         |
| **`addition with carry`**             | `7.61 ns` (✅ **1.00x**)                                 | `8.68 ns` (❌ *1.14x slower*)                            | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`subtraction with borrow`**         | `8.74 ns` (✅ **1.00x**)                                 | `9.92 ns` (❌ *1.13x slower*)                            | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)                                 | `4.88 ns` (✅ **1.04x slower**)                          | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`division by 2`**                   | `4.54 ns` (✅ **1.00x**)                                 | `4.52 ns` (✅ **1.00x faster**)                          | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |

### serialization

|                                          | `for bls12_381_optimized::g1projectivebls12_381`          | `for bls12_381_optimized::g2projectivebls12_381`          | `for bls12_381_optimized::froptimized`          | `for bls12_381_optimized::fqoptimized`          | `for bls12_381_optimized::fq2optimized`          | `for bls12_381_optimized::fq12optimized`           |
|:-----------------------------------------|:----------------------------------------------------------|:----------------------------------------------------------|:------------------------------------------------|:------------------------------------------------|:-------------------------------------------------|:-------------------------------------------------- |
| **`serialize compressed`**               | `164.10 ns` (✅ **1.00x**)                                 | `227.68 ns` (❌ *1.39x slower*)                            | `29.55 ns` (🚀 **5.55x faster**)                 | `55.73 ns` (🚀 **2.94x faster**)                 | `110.26 ns` (✅ **1.49x faster**)                 | `728.00 ns` (❌ *4.44x slower*)                     |
| **`serialize uncompressed`**             | `216.91 ns` (✅ **1.00x**)                                 | `328.43 ns` (❌ *1.51x slower*)                            | `29.54 ns` (🚀 **7.34x faster**)                 | `55.76 ns` (🚀 **3.89x faster**)                 | `110.28 ns` (🚀 **1.97x faster**)                 | `727.92 ns` (❌ *3.36x slower*)                     |
| **`deserialize compressed`**             | `326.00 us` (✅ **1.00x**)                                 | `560.89 us` (❌ *1.72x slower*)                            | `57.20 ns` (🚀 **5698.80x faster**)              | `100.90 ns` (🚀 **3230.88x faster**)             | `217.01 ns` (🚀 **1502.23x faster**)              | `1.34 us` (🚀 **242.93x faster**)                   |
| **`deserialize compressed unchecked`**   | `39.90 us` (✅ **1.00x**)                                  | `136.89 us` (❌ *3.43x slower*)                            | `56.96 ns` (🚀 **700.40x faster**)               | `100.85 ns` (🚀 **395.62x faster**)              | `217.25 ns` (🚀 **183.64x faster**)               | `1.36 us` (🚀 **29.43x faster**)                    |
| **`deserialize uncompressed`**           | `286.09 us` (✅ **1.00x**)                                 | `424.40 us` (❌ *1.48x slower*)                            | `57.15 ns` (🚀 **5005.49x faster**)              | `100.76 ns` (🚀 **2839.39x faster**)             | `217.19 ns` (🚀 **1317.22x faster**)              | `1.36 us` (🚀 **211.09x faster**)                   |
| **`deserialize uncompressed unchecked`** | `233.96 ns` (✅ **1.00x**)                                 | `521.16 ns` (❌ *2.23x slower*)                            | `57.17 ns` (🚀 **4.09x faster**)                 | `100.73 ns` (🚀 **2.32x faster**)                | `217.41 ns` (✅ **1.08x faster**)                 | `1.34 us` (❌ *5.73x slower*)                       |

### msm

|        | `for bls12_381_optimized::g1projectivebls12_381`          | `for bls12_381_optimized::g2projectivebls12_381`           |
|:-------|:----------------------------------------------------------|:---------------------------------------------------------- |
|        | `2.38 s` (✅ **1.00x**)                                    | `7.26 s` (❌ *3.05x slower*)                                |

### squareroot

|                          | `for bls12_381_optimized::froptimized`          | `for bls12_381_optimized::fqoptimized`          | `for bls12_381_optimized::fq2optimized`           |
|:-------------------------|:------------------------------------------------|:------------------------------------------------|:------------------------------------------------- |
| **`square root for qr`** | `25.88 us` (✅ **1.00x**)                        | `39.49 us` (❌ *1.53x slower*)                   | `135.31 us` (❌ *5.23x slower*)                    |
| **`legendre for qr`**    | `14.33 us` (✅ **1.00x**)                        | `39.54 us` (❌ *2.76x slower*)                   | `39.59 us` (❌ *2.76x slower*)                     |

### bitwise

|                               | `operations for bls12_381_optimized::froptimized::bigint`          | `operations for bls12_381_optimized::fqoptimized::bigint`           |
|:------------------------------|:-------------------------------------------------------------------|:------------------------------------------------------------------- |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                                            | `5.03 ns` (✅ **1.04x slower**)                                      |
| **`from little-endian bits`** | `132.28 ns` (✅ **1.00x**)                                          | `189.36 ns` (❌ *1.43x slower*)                                      |
| **`from big-endian bits`**    | `132.34 ns` (✅ **1.00x**)                                          | `189.24 ns` (❌ *1.43x slower*)                                      |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                                            | `5.08 ns` (✅ **1.03x slower**)                                      |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)                                            | `5.67 ns` (✅ **1.05x slower**)                                      |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                                            | `5.21 ns` (✅ **1.06x slower**)                                      |

### conversions

|                   | `for bls12_381_optimized::froptimized`          | `for bls12_381_optimized::fqoptimized`           |
|:------------------|:------------------------------------------------|:------------------------------------------------ |
| **`from bigint`** | `41.08 ns` (✅ **1.00x**)                        | `77.01 ns` (❌ *1.87x slower*)                    |
| **`into bigint`** | `26.45 ns` (✅ **1.00x**)                        | `47.71 ns` (❌ *1.80x slower*)                    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

