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
|        | `209.95 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.75x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.24 us` (✅ **1.00x**)          | `4.56 us` (❌ *3.68x slower*)     | `22.99 ns` (🚀 **53.86x faster**) | `194.62 ns` (🚀 **6.36x faster**)  | `12.48 ns` (🚀 **99.21x faster**)  | `8.72 ns` (🚀 **142.05x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.64 us` (❌ *3.60x slower*)     | `23.22 ns` (🚀 **55.41x faster**) | `162.03 ns` (🚀 **7.94x faster**)  | `12.73 ns` (🚀 **101.04x faster**) | `8.82 ns` (🚀 **145.92x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `903.14 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.67x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `933.93 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.59x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `609.92 ns` (✅ **1.00x**)        | `2.26 us` (❌ *3.70x slower*)     | `12.36 ns` (🚀 **49.33x faster**) | `71.97 ns` (🚀 **8.47x faster**)   | `7.13 ns` (🚀 **85.52x faster**)   | `5.92 ns` (🚀 **103.10x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.79 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.46x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.34 ns` (❌ *3.63x slower*)    | `95.77 ns` (❌ *15.54x slower*)    | `18.29 ns` (❌ *2.97x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.61 ns` (❌ *6.21x slower*)   | `7.18 us` (❌ *166.47x slower*)    | `76.25 ns` (❌ *1.77x slower*)     | `43.10 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `240.27 ns` (❌ *6.64x slower*)   | `5.05 us` (❌ *139.55x slower*)    | `66.46 ns` (❌ *1.84x slower*)     | `36.18 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.12 us` (❌ *2.15x slower*)    | `27.55 us` (❌ *3.91x slower*)     | `14.78 us` (❌ *2.10x slower*)     | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `578.42 ns` (❌ *9.42x slower*)   | `14.63 us` (❌ *238.36x slower*)   | `117.66 ns` (❌ *1.92x slower*)    | `61.38 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `565.33 ns` (❌ *6.18x slower*)   | `14.53 us` (❌ *158.80x slower*)   | `162.30 ns` (❌ *1.77x slower*)    | `91.52 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**)        | `10.49 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `160.21 ns` (✅ **1.00x**)        | `220.55 ns` (❌ *1.38x slower*)   | `30.60 ns` (🚀 **5.24x faster**)    | `55.49 ns` (🚀 **2.89x faster**)    | `110.45 ns` (✅ **1.45x faster**)    | `853.85 ns` (❌ *5.33x slower*)    |
| **`serialize_uncompressed`**             | `208.66 ns` (✅ **1.00x**)        | `338.76 ns` (❌ *1.62x slower*)   | `32.66 ns` (🚀 **6.39x faster**)    | `55.80 ns` (🚀 **3.74x faster**)    | `110.38 ns` (🚀 **1.89x faster**)    | `855.29 ns` (❌ *4.10x slower*)    |
| **`deserialize_compressed`**             | `314.36 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.37x slower*)     | `51.39 ns` (🚀 **6116.54x faster**) | `92.85 ns` (🚀 **3385.69x faster**) | `209.19 ns` (🚀 **1502.73x faster**) | `1.27 us` (🚀 **247.73x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.42 us` (✅ **1.00x**)         | `182.27 us` (❌ *2.70x slower*)   | `51.31 ns` (🚀 **1313.93x faster**) | `92.97 ns` (🚀 **725.18x faster**)  | `208.90 ns` (🚀 **322.74x faster**)  | `1.27 us` (🚀 **53.14x faster**)   |
| **`deserialize_uncompressed`**           | `247.07 us` (✅ **1.00x**)        | `875.34 us` (❌ *3.54x slower*)   | `51.42 ns` (🚀 **4805.18x faster**) | `93.24 ns` (🚀 **2649.79x faster**) | `209.45 ns` (🚀 **1179.62x faster**) | `1.27 us` (🚀 **195.03x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `222.25 ns` (✅ **1.00x**)        | `463.59 ns` (❌ *2.09x slower*)   | `51.33 ns` (🚀 **4.33x faster**)    | `93.30 ns` (🚀 **2.38x faster**)    | `208.86 ns` (✅ **1.06x faster**)    | `1.27 us` (❌ *5.70x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.31 s` (❌ *3.52x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.15 us` (✅ **1.00x**) | `66.90 us` (❌ *2.15x slower*)   | `181.73 us` (❌ *5.83x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.57 us` (❌ *2.88x slower*)   | `31.52 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.93 ns` (✅ **1.00x**)       | `83.37 ns` (❌ *1.70x slower*)    |
| **`from_big-endian_bits`**    | `48.91 ns` (✅ **1.00x**)       | `83.61 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.39 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.54 ns` (✅ **1.00x**) | `75.22 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `23.76 ns` (✅ **1.00x**) | `47.01 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

