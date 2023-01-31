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

|        | `bls12_377_optimized::g1projectivebls12_377 elements`          | `bls12_377_optimized::g2projectivebls12_377 elements`           |
|:-------|:---------------------------------------------------------------|:--------------------------------------------------------------- |
|        | `348.54 us` (✅ **1.00x**)                                      | `2.44 ms` (❌ *7.01x slower*)                                    |

### arithmetic

|                                       | `for bls12_377_optimized::froptimized::bigint`          | `for bls12_377_optimized::fqoptimized::bigint`          | `for bls12_377_optimized::g1projectivebls12_377`          | `for bls12_377_optimized::g2projectivebls12_377`          | `for bls12_377_optimized::fq2optimized`          | `for bls12_377_optimized::fq12optimized`          | `for bls12_377_optimized::fqoptimized`          | `for bls12_377_optimized::froptimized`           |
|:--------------------------------------|:--------------------------------------------------------|:--------------------------------------------------------|:----------------------------------------------------------|:----------------------------------------------------------|:-------------------------------------------------|:--------------------------------------------------|:------------------------------------------------|:------------------------------------------------ |
| **`addition`**                        | `N/A`                                                   | `N/A`                                                   | `1.25 us` (✅ **1.00x**)                                   | `4.64 us` (❌ *3.71x slower*)                              | `23.00 ns` (🚀 **54.37x faster**)                 | `203.95 ns` (🚀 **6.13x faster**)                  | `12.39 ns` (🚀 **100.95x faster**)               | `8.79 ns` (🚀 **142.26x faster**)                 |
| **`subtraction`**                     | `N/A`                                                   | `N/A`                                                   | `1.29 us` (✅ **1.00x**)                                   | `4.69 us` (❌ *3.63x slower*)                              | `23.00 ns` (🚀 **56.23x faster**)                 | `188.00 ns` (🚀 **6.88x faster**)                  | `13.32 ns` (🚀 **97.11x faster**)                | `9.03 ns` (🚀 **143.26x faster**)                 |
| **`mixed addition`**                  | `N/A`                                                   | `N/A`                                                   | `918.77 ns` (✅ **1.00x**)                                 | `3.35 us` (❌ *3.64x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`mixed subtraction`**               | `N/A`                                                   | `N/A`                                                   | `951.76 ns` (✅ **1.00x**)                                 | `3.39 us` (❌ *3.56x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`double`**                          | `N/A`                                                   | `N/A`                                                   | `609.80 ns` (✅ **1.00x**)                                 | `2.25 us` (❌ *3.69x slower*)                              | `12.38 ns` (🚀 **49.25x faster**)                 | `152.06 ns` (🚀 **4.01x faster**)                  | `7.18 ns` (🚀 **84.90x faster**)                 | `5.87 ns` (🚀 **103.80x faster**)                 |
| **`scalar multiplication`**           | `N/A`                                                   | `N/A`                                                   | `477.38 us` (✅ **1.00x**)                                 | `1.57 ms` (❌ *3.28x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`negation`**                        | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `23.11 ns` (❌ *3.69x slower*)                    | `117.42 ns` (❌ *18.76x slower*)                   | `18.31 ns` (❌ *2.93x slower*)                   | `6.26 ns` (✅ **1.00x**)                          |
| **`multiplication`**                  | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `279.28 ns` (❌ *6.08x slower*)                   | `7.27 us` (❌ *158.30x slower*)                    | `76.40 ns` (❌ *1.66x slower*)                   | `45.95 ns` (✅ **1.00x**)                         |
| **`square`**                          | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `243.93 ns` (❌ *6.55x slower*)                   | `5.12 us` (❌ *137.61x slower*)                    | `67.30 ns` (❌ *1.81x slower*)                   | `37.24 ns` (✅ **1.00x**)                         |
| **`inverse`**                         | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `15.48 us` (❌ *2.11x slower*)                    | `28.06 us` (❌ *3.82x slower*)                     | `15.11 us` (❌ *2.05x slower*)                   | `7.35 us` (✅ **1.00x**)                          |
| **`sum of products of size 2`**       | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `605.04 ns` (❌ *9.82x slower*)                   | `14.77 us` (❌ *239.74x slower*)                   | `121.73 ns` (❌ *1.98x slower*)                  | `61.61 ns` (✅ **1.00x**)                         |
| **`naive sum of products of size 2`** | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `595.94 ns` (❌ *6.63x slower*)                   | `14.69 us` (❌ *163.55x slower*)                   | `163.20 ns` (❌ *1.82x slower*)                  | `89.84 ns` (✅ **1.00x**)                         |
| **`addition with carry`**             | `7.62 ns` (✅ **1.00x**)                                 | `8.68 ns` (❌ *1.14x slower*)                            | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`subtraction with borrow`**         | `8.72 ns` (✅ **1.00x**)                                 | `9.84 ns` (❌ *1.13x slower*)                            | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)                                 | `4.88 ns` (✅ **1.04x slower**)                          | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`division by 2`**                   | `4.52 ns` (✅ **1.00x**)                                 | `4.53 ns` (✅ **1.00x slower**)                          | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |

### serialization

|                                          | `for bls12_377_optimized::g1projectivebls12_377`          | `for bls12_377_optimized::g2projectivebls12_377`          | `for bls12_377_optimized::froptimized`          | `for bls12_377_optimized::fqoptimized`          | `for bls12_377_optimized::fq2optimized`          | `for bls12_377_optimized::fq12optimized`           |
|:-----------------------------------------|:----------------------------------------------------------|:----------------------------------------------------------|:------------------------------------------------|:------------------------------------------------|:-------------------------------------------------|:-------------------------------------------------- |
| **`serialize compressed`**               | `152.56 ns` (✅ **1.00x**)                                 | `216.12 ns` (❌ *1.42x slower*)                            | `28.42 ns` (🚀 **5.37x faster**)                 | `55.10 ns` (🚀 **2.77x faster**)                 | `107.77 ns` (✅ **1.42x faster**)                 | `696.84 ns` (❌ *4.57x slower*)                     |
| **`serialize uncompressed`**             | `208.19 ns` (✅ **1.00x**)                                 | `335.59 ns` (❌ *1.61x slower*)                            | `28.46 ns` (🚀 **7.31x faster**)                 | `55.09 ns` (🚀 **3.78x faster**)                 | `107.85 ns` (🚀 **1.93x faster**)                 | `698.13 ns` (❌ *3.35x slower*)                     |
| **`deserialize compressed`**             | `386.18 us` (✅ **1.00x**)                                 | `1.26 ms` (❌ *3.26x slower*)                              | `56.07 ns` (🚀 **6887.16x faster**)              | `99.55 ns` (🚀 **3879.05x faster**)              | `210.92 ns` (🚀 **1830.91x faster**)              | `1.35 us` (🚀 **285.93x faster**)                   |
| **`deserialize compressed unchecked`**   | `68.50 us` (✅ **1.00x**)                                  | `185.50 us` (❌ *2.71x slower*)                            | `56.06 ns` (🚀 **1221.84x faster**)              | `99.27 ns` (🚀 **690.07x faster**)               | `210.34 ns` (🚀 **325.66x faster**)               | `1.34 us` (🚀 **51.22x faster**)                    |
| **`deserialize uncompressed`**           | `316.63 us` (✅ **1.00x**)                                 | `1.07 ms` (❌ *3.39x slower*)                              | `56.05 ns` (🚀 **5649.26x faster**)              | `99.53 ns` (🚀 **3181.07x faster**)              | `210.89 ns` (🚀 **1501.35x faster**)              | `1.35 us` (🚀 **234.87x faster**)                   |
| **`deserialize uncompressed unchecked`** | `238.78 ns` (✅ **1.00x**)                                 | `516.66 ns` (❌ *2.16x slower*)                            | `56.05 ns` (🚀 **4.26x faster**)                 | `99.49 ns` (🚀 **2.40x faster**)                 | `210.87 ns` (✅ **1.13x faster**)                 | `1.35 us` (❌ *5.65x slower*)                       |

### msm

|        | `for bls12_377_optimized::g1projectivebls12_377`          | `for bls12_377_optimized::g2projectivebls12_377`           |
|:-------|:----------------------------------------------------------|:---------------------------------------------------------- |
|        | `2.38 s` (✅ **1.00x**)                                    | `8.50 s` (❌ *3.57x slower*)                                |

### squareroot

|                          | `for bls12_377_optimized::froptimized`          | `for bls12_377_optimized::fqoptimized`          | `for bls12_377_optimized::fq2optimized`           |
|:-------------------------|:------------------------------------------------|:------------------------------------------------|:------------------------------------------------- |
| **`square root for qr`** | `31.40 us` (✅ **1.00x**)                        | `68.06 us` (❌ *2.17x slower*)                   | `184.45 us` (❌ *5.87x slower*)                    |
| **`legendre for qr`**    | `10.99 us` (✅ **1.00x**)                        | `32.11 us` (❌ *2.92x slower*)                   | `31.99 us` (❌ *2.91x slower*)                     |

### bitwise

|                               | `operations for bls12_377_optimized::froptimized::bigint`          | `operations for bls12_377_optimized::fqoptimized::bigint`           |
|:------------------------------|:-------------------------------------------------------------------|:------------------------------------------------------------------- |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                                            | `5.03 ns` (✅ **1.04x slower**)                                      |
| **`from little-endian bits`** | `132.22 ns` (✅ **1.00x**)                                          | `189.40 ns` (❌ *1.43x slower*)                                      |
| **`from big-endian bits`**    | `132.31 ns` (✅ **1.00x**)                                          | `189.26 ns` (❌ *1.43x slower*)                                      |
| **`comparison`**              | `4.92 ns` (✅ **1.00x**)                                            | `5.14 ns` (✅ **1.05x slower**)                                      |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)                                            | `5.75 ns` (✅ **1.06x slower**)                                      |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                                            | `5.21 ns` (✅ **1.06x slower**)                                      |

### conversions

|                   | `for bls12_377_optimized::froptimized`          | `for bls12_377_optimized::fqoptimized`           |
|:------------------|:------------------------------------------------|:------------------------------------------------ |
| **`from bigint`** | `40.78 ns` (✅ **1.00x**)                        | `75.74 ns` (❌ *1.86x slower*)                    |
| **`into bigint`** | `25.75 ns` (✅ **1.00x**)                        | `47.01 ns` (❌ *1.83x slower*)                    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

