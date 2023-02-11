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
|        | `210.99 us` (✅ **1.00x**)                 | `2.06 ms` (❌ *9.74x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.24 us` (✅ **1.00x**)          | `4.55 us` (❌ *3.67x slower*)     | `23.20 ns` (🚀 **53.54x faster**) | `177.89 ns` (🚀 **6.98x faster**)  | `12.49 ns` (🚀 **99.41x faster**)  | `8.73 ns` (🚀 **142.21x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.28 us` (✅ **1.00x**)          | `4.60 us` (❌ *3.59x slower*)     | `23.29 ns` (🚀 **55.10x faster**) | `157.23 ns` (🚀 **8.16x faster**)  | `12.70 ns` (🚀 **101.01x faster**) | `8.80 ns` (🚀 **145.85x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `906.44 ns` (✅ **1.00x**)        | `3.28 us` (❌ *3.62x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `944.42 ns` (✅ **1.00x**)        | `3.33 us` (❌ *3.53x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `592.38 ns` (✅ **1.00x**)        | `2.23 us` (❌ *3.76x slower*)     | `12.33 ns` (🚀 **48.05x faster**) | `67.32 ns` (🚀 **8.80x faster**)   | `7.13 ns` (🚀 **83.02x faster**)   | `5.85 ns` (🚀 **101.27x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `339.29 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.48x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.61 ns` (❌ *3.66x slower*)    | `92.63 ns` (❌ *14.98x slower*)    | `18.24 ns` (❌ *2.95x slower*)     | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.18 ns` (❌ *6.34x slower*)   | `7.10 us` (❌ *165.93x slower*)    | `76.13 ns` (❌ *1.78x slower*)     | `42.80 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `236.96 ns` (❌ *6.70x slower*)   | `5.01 us` (❌ *141.54x slower*)    | `66.37 ns` (❌ *1.88x slower*)     | `35.38 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.09 us` (❌ *2.14x slower*)    | `27.33 us` (❌ *3.87x slower*)     | `14.78 us` (❌ *2.09x slower*)     | `7.07 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `589.68 ns` (❌ *9.56x slower*)   | `14.52 us` (❌ *235.51x slower*)   | `117.97 ns` (❌ *1.91x slower*)    | `61.66 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `577.88 ns` (❌ *6.32x slower*)   | `14.44 us` (❌ *157.85x slower*)   | `163.74 ns` (❌ *1.79x slower*)    | `91.49 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.64 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**)        | `10.30 ns` (❌ *1.18x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.79 ns` (✅ **1.00x**)        | `4.80 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `160.06 ns` (✅ **1.00x**)        | `222.87 ns` (❌ *1.39x slower*)   | `31.42 ns` (🚀 **5.09x faster**)    | `56.72 ns` (🚀 **2.82x faster**)    | `113.61 ns` (✅ **1.41x faster**)    | `707.45 ns` (❌ *4.42x slower*)    |
| **`serialize_uncompressed`**             | `208.74 ns` (✅ **1.00x**)        | `341.08 ns` (❌ *1.63x slower*)   | `32.12 ns` (🚀 **6.50x faster**)    | `55.83 ns` (🚀 **3.74x faster**)    | `113.41 ns` (🚀 **1.84x faster**)    | `706.21 ns` (❌ *3.38x slower*)    |
| **`deserialize_compressed`**             | `314.58 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.39x slower*)     | `52.51 ns` (🚀 **5990.72x faster**) | `92.03 ns` (🚀 **3418.20x faster**) | `215.54 ns` (🚀 **1459.52x faster**) | `1.27 us` (🚀 **248.56x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.96 us` (✅ **1.00x**)         | `183.29 us` (❌ *2.70x slower*)   | `52.52 ns` (🚀 **1293.94x faster**) | `92.13 ns` (🚀 **737.68x faster**)  | `215.62 ns` (🚀 **315.19x faster**)  | `1.27 us` (🚀 **53.60x faster**)   |
| **`deserialize_uncompressed`**           | `246.68 us` (✅ **1.00x**)        | `879.03 us` (❌ *3.56x slower*)   | `52.45 ns` (🚀 **4703.24x faster**) | `92.20 ns` (🚀 **2675.58x faster**) | `215.28 ns` (🚀 **1145.85x faster**) | `1.27 us` (🚀 **194.90x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `222.57 ns` (✅ **1.00x**)        | `470.72 ns` (❌ *2.11x slower*)   | `52.44 ns` (🚀 **4.24x faster**)    | `91.92 ns` (🚀 **2.42x faster**)    | `214.97 ns` (✅ **1.04x faster**)    | `1.27 us` (❌ *5.69x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.39 s` (✅ **1.00x**)           | `8.25 s` (❌ *3.46x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.07 us` (✅ **1.00x**) | `67.46 us` (❌ *2.17x slower*)   | `182.24 us` (❌ *5.87x slower*)    |
| **`legendre_for_qr`**    | `10.90 us` (✅ **1.00x**) | `31.45 us` (❌ *2.88x slower*)   | `31.93 us` (❌ *2.93x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.74 ns` (✅ **1.00x**)       | `89.65 ns` (❌ *1.84x slower*)    |
| **`from_big-endian_bits`**    | `48.76 ns` (✅ **1.00x**)       | `89.92 ns` (❌ *1.84x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.20 ns` (✅ **1.06x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.73 ns` (✅ **1.00x**) | `74.81 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `22.95 ns` (✅ **1.00x**) | `46.94 ns` (❌ *2.05x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

