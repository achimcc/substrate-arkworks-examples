# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377_optimized](#sample_bls12_377_optimized)
    - [arithmetic_for_bls12_377_optimized](#arithmetic_for_bls12_377_optimized)
    - [serialization_for_bls12_377_optimized](#serialization_for_bls12_377_optimized)
    - [msm_for_bls12_377_optimized](#msm_for_bls12_377_optimized)
    - [squareroot_for_bls12_377_optimized](#squareroot_for_bls12_377_optimized)
    - [bitwise_operations_for_bls12_377_optimized](#bitwise_operations_for_bls12_377_optimized)
    - [conversions_for_bls12_377_optimized](#conversions_for_bls12_377_optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `211.06 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.73x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.24 us` (✅ **1.00x**)          | `4.56 us` (❌ *3.67x slower*)     | `23.17 ns` (🚀 **53.64x faster**) | `178.64 ns` (🚀 **6.96x faster**)  | `12.49 ns` (🚀 **99.48x faster**)  | `8.71 ns` (🚀 **142.66x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.61 us` (❌ *3.56x slower*)     | `23.31 ns` (🚀 **55.45x faster**) | `158.48 ns` (🚀 **8.16x faster**)  | `12.70 ns` (🚀 **101.76x faster**) | `8.80 ns` (🚀 **146.94x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `905.45 ns` (✅ **1.00x**)        | `3.29 us` (❌ *3.63x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `944.48 ns` (✅ **1.00x**)        | `3.33 us` (❌ *3.53x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `593.17 ns` (✅ **1.00x**)        | `2.25 us` (❌ *3.80x slower*)     | `12.34 ns` (🚀 **48.07x faster**) | `67.35 ns` (🚀 **8.81x faster**)   | `7.14 ns` (🚀 **83.11x faster**)   | `5.86 ns` (🚀 **101.30x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `339.45 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.48x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.69 ns` (❌ *3.67x slower*)    | `92.69 ns` (❌ *15.00x slower*)    | `18.25 ns` (❌ *2.95x slower*)     | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.17 ns` (❌ *6.30x slower*)   | `7.11 us` (❌ *165.09x slower*)    | `75.75 ns` (❌ *1.76x slower*)     | `43.07 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `236.52 ns` (❌ *6.69x slower*)   | `5.02 us` (❌ *142.09x slower*)    | `66.57 ns` (❌ *1.88x slower*)     | `35.36 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.09 us` (❌ *2.14x slower*)    | `27.33 us` (❌ *3.88x slower*)     | `14.76 us` (❌ *2.09x slower*)     | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `589.93 ns` (❌ *9.57x slower*)   | `14.53 us` (❌ *235.75x slower*)   | `117.96 ns` (❌ *1.91x slower*)    | `61.64 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `578.33 ns` (❌ *6.32x slower*)   | `14.46 us` (❌ *158.02x slower*)   | `163.62 ns` (❌ *1.79x slower*)    | `91.50 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.63 ns` (❌ *1.13x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.70 ns` (✅ **1.00x**)        | `10.30 ns` (❌ *1.18x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.68 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.01x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `190.17 ns` (✅ **1.00x**)        | `223.66 ns` (❌ *1.18x slower*)   | `31.31 ns` (🚀 **6.07x faster**)    | `56.33 ns` (🚀 **3.38x faster**)    | `113.76 ns` (✅ **1.67x faster**)    | `707.72 ns` (❌ *3.72x slower*)    |
| **`serialize_uncompressed`**             | `208.70 ns` (✅ **1.00x**)        | `342.66 ns` (❌ *1.64x slower*)   | `31.97 ns` (🚀 **6.53x faster**)    | `55.92 ns` (🚀 **3.73x faster**)    | `113.74 ns` (🚀 **1.83x faster**)    | `707.78 ns` (❌ *3.39x slower*)    |
| **`deserialize_compressed`**             | `314.68 us` (✅ **1.00x**)        | `1.07 ms` (❌ *3.38x slower*)     | `52.56 ns` (🚀 **5987.37x faster**) | `92.08 ns` (🚀 **3417.37x faster**) | `215.56 ns` (🚀 **1459.80x faster**) | `1.27 us` (🚀 **248.25x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.97 us` (✅ **1.00x**)         | `183.47 us` (❌ *2.70x slower*)   | `52.55 ns` (🚀 **1293.45x faster**) | `92.06 ns` (🚀 **738.33x faster**)  | `215.46 ns` (🚀 **315.47x faster**)  | `1.27 us` (🚀 **53.67x faster**)   |
| **`deserialize_uncompressed`**           | `246.78 us` (✅ **1.00x**)        | `879.40 us` (❌ *3.56x slower*)   | `52.49 ns` (🚀 **4701.73x faster**) | `92.06 ns` (🚀 **2680.74x faster**) | `214.80 ns` (🚀 **1148.88x faster**) | `1.27 us` (🚀 **194.79x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `222.49 ns` (✅ **1.00x**)        | `490.38 ns` (❌ *2.20x slower*)   | `52.46 ns` (🚀 **4.24x faster**)    | `92.03 ns` (🚀 **2.42x faster**)    | `214.91 ns` (✅ **1.04x faster**)    | `1.27 us` (❌ *5.69x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.39 s` (✅ **1.00x**)           | `8.30 s` (❌ *3.47x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.08 us` (✅ **1.00x**) | `67.49 us` (❌ *2.17x slower*)   | `182.25 us` (❌ *5.86x slower*)    |
| **`legendre_for_qr`**    | `10.90 us` (✅ **1.00x**) | `31.46 us` (❌ *2.89x slower*)   | `31.94 us` (❌ *2.93x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.78 ns` (✅ **1.00x**)       | `90.01 ns` (❌ *1.85x slower*)    |
| **`from_big-endian_bits`**    | `48.78 ns` (✅ **1.00x**)       | `89.80 ns` (❌ *1.84x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.20 ns` (✅ **1.06x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.76 ns` (✅ **1.00x**) | `74.75 ns` (❌ *1.83x slower*)    |
| **`into_bigint`** | `22.97 ns` (✅ **1.00x**) | `46.95 ns` (❌ *2.04x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

