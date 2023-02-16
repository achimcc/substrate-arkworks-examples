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
|        | `211.40 us` (✅ **1.00x**)                 | `2.04 ms` (❌ *9.66x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.58 us` (❌ *3.67x slower*)     | `23.15 ns` (🚀 **53.86x faster**) | `178.00 ns` (🚀 **7.01x faster**)  | `12.52 ns` (🚀 **99.63x faster**)  | `8.71 ns` (🚀 **143.23x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.30 us` (✅ **1.00x**)          | `4.64 us` (❌ *3.58x slower*)     | `23.27 ns` (🚀 **55.65x faster**) | `158.55 ns` (🚀 **8.17x faster**)  | `12.74 ns` (🚀 **101.63x faster**) | `8.79 ns` (🚀 **147.34x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `905.13 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.66x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `938.79 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.57x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `599.04 ns` (✅ **1.00x**)        | `2.25 us` (❌ *3.76x slower*)     | `12.35 ns` (🚀 **48.50x faster**) | `70.95 ns` (🚀 **8.44x faster**)   | `7.13 ns` (🚀 **84.04x faster**)   | `5.84 ns` (🚀 **102.58x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.86 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.44x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.14 ns` (❌ *3.76x slower*)    | `93.25 ns` (❌ *15.14x slower*)    | `18.20 ns` (❌ *2.96x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.63 ns` (❌ *6.24x slower*)   | `7.09 us` (❌ *162.85x slower*)    | `75.27 ns` (❌ *1.73x slower*)     | `43.55 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `241.35 ns` (❌ *6.72x slower*)   | `5.02 us` (❌ *139.70x slower*)    | `66.80 ns` (❌ *1.86x slower*)     | `35.92 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `14.93 us` (❌ *2.12x slower*)    | `27.32 us` (❌ *3.87x slower*)     | `14.64 us` (❌ *2.07x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `590.48 ns` (❌ *9.63x slower*)   | `14.55 us` (❌ *237.24x slower*)   | `117.85 ns` (❌ *1.92x slower*)    | `61.33 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `577.48 ns` (❌ *6.30x slower*)   | `14.47 us` (❌ *157.87x slower*)   | `162.52 ns` (❌ *1.77x slower*)    | `91.64 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.67 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.70 ns` (✅ **1.00x**)        | `10.32 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.73 ns` (✅ **1.00x**)        | `4.77 ns` (✅ **1.01x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `160.08 ns` (✅ **1.00x**)        | `222.91 ns` (❌ *1.39x slower*)   | `30.90 ns` (🚀 **5.18x faster**)    | `56.09 ns` (🚀 **2.85x faster**)    | `110.47 ns` (✅ **1.45x faster**)    | `699.28 ns` (❌ *4.37x slower*)    |
| **`serialize_uncompressed`**             | `209.28 ns` (✅ **1.00x**)        | `344.38 ns` (❌ *1.65x slower*)   | `30.59 ns` (🚀 **6.84x faster**)    | `55.87 ns` (🚀 **3.75x faster**)    | `110.40 ns` (🚀 **1.90x faster**)    | `699.44 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `315.91 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `52.44 ns` (🚀 **6024.68x faster**) | `93.54 ns` (🚀 **3377.32x faster**) | `210.02 ns` (🚀 **1504.17x faster**) | `1.32 us` (🚀 **239.43x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.81 us` (✅ **1.00x**)         | `182.72 us` (❌ *2.69x slower*)   | `52.49 ns` (🚀 **1291.87x faster**) | `93.51 ns` (🚀 **725.14x faster**)  | `210.15 ns` (🚀 **322.65x faster**)  | `1.31 us` (🚀 **51.65x faster**)   |
| **`deserialize_uncompressed`**           | `249.77 us` (✅ **1.00x**)        | `873.70 us` (❌ *3.50x slower*)   | `52.32 ns` (🚀 **4773.46x faster**) | `93.69 ns` (🚀 **2665.98x faster**) | `209.66 ns` (🚀 **1191.34x faster**) | `1.31 us` (🚀 **190.23x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.41 ns` (✅ **1.00x**)        | `464.65 ns` (❌ *2.08x slower*)   | `52.49 ns` (🚀 **4.26x faster**)    | `93.78 ns` (🚀 **2.38x faster**)    | `209.52 ns` (✅ **1.07x faster**)    | `1.31 us` (❌ *5.87x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.37 s` (❌ *3.55x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.32 us` (✅ **1.00x**) | `67.29 us` (❌ *2.15x slower*)   | `181.70 us` (❌ *5.80x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.66 us` (❌ *2.89x slower*)   | `32.88 us` (❌ *3.00x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.89 ns` (✅ **1.00x**)       | `89.28 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `48.90 ns` (✅ **1.00x**)       | `89.21 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.56 ns` (✅ **1.00x**) | `75.08 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.93 ns` (✅ **1.00x**) | `46.87 ns` (❌ *2.04x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

