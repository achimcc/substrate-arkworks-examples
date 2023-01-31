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
|        | `348.19 us` (✅ **1.00x**)                                      | `2.44 ms` (❌ *7.02x slower*)                                    |

### arithmetic

|                                       | `for bls12_377_optimized::froptimized::bigint`          | `for bls12_377_optimized::fqoptimized::bigint`          | `for bls12_377_optimized::g1projectivebls12_377`          | `for bls12_377_optimized::g2projectivebls12_377`          | `for bls12_377_optimized::fq2optimized`          | `for bls12_377_optimized::fq12optimized`          | `for bls12_377_optimized::fqoptimized`          | `for bls12_377_optimized::froptimized`           |
|:--------------------------------------|:--------------------------------------------------------|:--------------------------------------------------------|:----------------------------------------------------------|:----------------------------------------------------------|:-------------------------------------------------|:--------------------------------------------------|:------------------------------------------------|:------------------------------------------------ |
| **`addition`**                        | `N/A`                                                   | `N/A`                                                   | `1.25 us` (✅ **1.00x**)                                   | `4.64 us` (❌ *3.71x slower*)                              | `22.99 ns` (🚀 **54.39x faster**)                 | `200.51 ns` (🚀 **6.24x faster**)                  | `12.44 ns` (🚀 **100.52x faster**)               | `8.80 ns` (🚀 **142.13x faster**)                 |
| **`subtraction`**                     | `N/A`                                                   | `N/A`                                                   | `1.29 us` (✅ **1.00x**)                                   | `4.69 us` (❌ *3.63x slower*)                              | `23.01 ns` (🚀 **56.20x faster**)                 | `187.44 ns` (🚀 **6.90x faster**)                  | `13.31 ns` (🚀 **97.16x faster**)                | `9.01 ns` (🚀 **143.53x faster**)                 |
| **`mixed addition`**                  | `N/A`                                                   | `N/A`                                                   | `918.23 ns` (✅ **1.00x**)                                 | `3.34 us` (❌ *3.64x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`mixed subtraction`**               | `N/A`                                                   | `N/A`                                                   | `951.12 ns` (✅ **1.00x**)                                 | `3.39 us` (❌ *3.56x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`double`**                          | `N/A`                                                   | `N/A`                                                   | `608.65 ns` (✅ **1.00x**)                                 | `2.25 us` (❌ *3.70x slower*)                              | `12.31 ns` (🚀 **49.44x faster**)                 | `148.81 ns` (🚀 **4.09x faster**)                  | `7.19 ns` (🚀 **84.62x faster**)                 | `5.87 ns` (🚀 **103.65x faster**)                 |
| **`scalar multiplication`**           | `N/A`                                                   | `N/A`                                                   | `477.15 us` (✅ **1.00x**)                                 | `1.57 ms` (❌ *3.28x slower*)                              | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`negation`**                        | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `23.15 ns` (❌ *3.70x slower*)                    | `115.89 ns` (❌ *18.52x slower*)                   | `17.94 ns` (❌ *2.87x slower*)                   | `6.26 ns` (✅ **1.00x**)                          |
| **`multiplication`**                  | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `279.40 ns` (❌ *6.08x slower*)                   | `7.25 us` (❌ *157.78x slower*)                    | `76.73 ns` (❌ *1.67x slower*)                   | `45.98 ns` (✅ **1.00x**)                         |
| **`square`**                          | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `243.84 ns` (❌ *6.55x slower*)                   | `5.13 us` (❌ *137.67x slower*)                    | `66.93 ns` (❌ *1.80x slower*)                   | `37.24 ns` (✅ **1.00x**)                         |
| **`inverse`**                         | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `15.50 us` (❌ *2.11x slower*)                    | `28.09 us` (❌ *3.82x slower*)                     | `15.12 us` (❌ *2.06x slower*)                   | `7.35 us` (✅ **1.00x**)                          |
| **`sum of products of size 2`**       | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `605.89 ns` (❌ *9.84x slower*)                   | `14.77 us` (❌ *239.88x slower*)                   | `121.56 ns` (❌ *1.97x slower*)                  | `61.59 ns` (✅ **1.00x**)                         |
| **`naive sum of products of size 2`** | `N/A`                                                   | `N/A`                                                   | `N/A`                                                     | `N/A`                                                     | `595.04 ns` (❌ *6.63x slower*)                   | `14.70 us` (❌ *163.65x slower*)                   | `162.90 ns` (❌ *1.81x slower*)                  | `89.81 ns` (✅ **1.00x**)                         |
| **`addition with carry`**             | `7.62 ns` (✅ **1.00x**)                                 | `8.67 ns` (❌ *1.14x slower*)                            | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`subtraction with borrow`**         | `8.70 ns` (✅ **1.00x**)                                 | `9.83 ns` (❌ *1.13x slower*)                            | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)                                 | `4.87 ns` (✅ **1.04x slower**)                          | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |
| **`division by 2`**                   | `4.54 ns` (✅ **1.00x**)                                 | `4.53 ns` (✅ **1.00x faster**)                          | `N/A`                                                     | `N/A`                                                     | `N/A`                                            | `N/A`                                             | `N/A`                                           | `N/A`                                            |

### serialization

|                                          | `for bls12_377_optimized::g1projectivebls12_377`          | `for bls12_377_optimized::g2projectivebls12_377`          | `for bls12_377_optimized::froptimized`          | `for bls12_377_optimized::fqoptimized`          | `for bls12_377_optimized::fq2optimized`          | `for bls12_377_optimized::fq12optimized`           |
|:-----------------------------------------|:----------------------------------------------------------|:----------------------------------------------------------|:------------------------------------------------|:------------------------------------------------|:-------------------------------------------------|:-------------------------------------------------- |
| **`serialize compressed`**               | `152.80 ns` (✅ **1.00x**)                                 | `216.41 ns` (❌ *1.42x slower*)                            | `28.34 ns` (🚀 **5.39x faster**)                 | `55.00 ns` (🚀 **2.78x faster**)                 | `107.54 ns` (✅ **1.42x faster**)                 | `697.58 ns` (❌ *4.57x slower*)                     |
| **`serialize uncompressed`**             | `207.83 ns` (✅ **1.00x**)                                 | `336.22 ns` (❌ *1.62x slower*)                            | `28.48 ns` (🚀 **7.30x faster**)                 | `54.93 ns` (🚀 **3.78x faster**)                 | `107.48 ns` (🚀 **1.93x faster**)                 | `697.22 ns` (❌ *3.35x slower*)                     |
| **`deserialize compressed`**             | `384.91 us` (✅ **1.00x**)                                 | `1.26 ms` (❌ *3.27x slower*)                              | `55.95 ns` (🚀 **6879.05x faster**)              | `99.63 ns` (🚀 **3863.47x faster**)              | `210.58 ns` (🚀 **1827.91x faster**)              | `1.35 us` (🚀 **285.09x faster**)                   |
| **`deserialize compressed unchecked`**   | `68.45 us` (✅ **1.00x**)                                  | `185.42 us` (❌ *2.71x slower*)                            | `55.97 ns` (🚀 **1223.00x faster**)              | `99.61 ns` (🚀 **687.19x faster**)               | `210.59 ns` (🚀 **325.04x faster**)               | `1.34 us` (🚀 **51.24x faster**)                    |
| **`deserialize uncompressed`**           | `316.51 us` (✅ **1.00x**)                                 | `1.07 ms` (❌ *3.39x slower*)                              | `55.91 ns` (🚀 **5661.04x faster**)              | `99.42 ns` (🚀 **3183.59x faster**)              | `210.74 ns` (🚀 **1501.87x faster**)              | `1.35 us` (🚀 **234.96x faster**)                   |
| **`deserialize uncompressed unchecked`** | `238.66 ns` (✅ **1.00x**)                                 | `516.26 ns` (❌ *2.16x slower*)                            | `55.91 ns` (🚀 **4.27x faster**)                 | `99.49 ns` (🚀 **2.40x faster**)                 | `210.68 ns` (✅ **1.13x faster**)                 | `1.35 us` (❌ *5.65x slower*)                       |

### msm

|        | `for bls12_377_optimized::g1projectivebls12_377`          | `for bls12_377_optimized::g2projectivebls12_377`           |
|:-------|:----------------------------------------------------------|:---------------------------------------------------------- |
|        | `2.37 s` (✅ **1.00x**)                                    | `8.43 s` (❌ *3.55x slower*)                                |

### squareroot

|                          | `for bls12_377_optimized::froptimized`          | `for bls12_377_optimized::fqoptimized`          | `for bls12_377_optimized::fq2optimized`           |
|:-------------------------|:------------------------------------------------|:------------------------------------------------|:------------------------------------------------- |
| **`square root for qr`** | `31.38 us` (✅ **1.00x**)                        | `68.05 us` (❌ *2.17x slower*)                   | `184.55 us` (❌ *5.88x slower*)                    |
| **`legendre for qr`**    | `10.99 us` (✅ **1.00x**)                        | `32.09 us` (❌ *2.92x slower*)                   | `31.98 us` (❌ *2.91x slower*)                     |

### bitwise

|                               | `operations for bls12_377_optimized::froptimized::bigint`          | `operations for bls12_377_optimized::fqoptimized::bigint`           |
|:------------------------------|:-------------------------------------------------------------------|:------------------------------------------------------------------- |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                                            | `5.02 ns` (✅ **1.04x slower**)                                      |
| **`from little-endian bits`** | `130.49 ns` (✅ **1.00x**)                                          | `189.18 ns` (❌ *1.45x slower*)                                      |
| **`from big-endian bits`**    | `130.54 ns` (✅ **1.00x**)                                          | `189.20 ns` (❌ *1.45x slower*)                                      |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                                            | `5.14 ns` (✅ **1.05x slower**)                                      |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)                                            | `5.72 ns` (✅ **1.06x slower**)                                      |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                                            | `5.21 ns` (✅ **1.06x slower**)                                      |

### conversions

|                   | `for bls12_377_optimized::froptimized`          | `for bls12_377_optimized::fqoptimized`           |
|:------------------|:------------------------------------------------|:------------------------------------------------ |
| **`from bigint`** | `40.76 ns` (✅ **1.00x**)                        | `75.69 ns` (❌ *1.86x slower*)                    |
| **`into bigint`** | `25.65 ns` (✅ **1.00x**)                        | `46.92 ns` (❌ *1.83x slower*)                    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

