# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381_optimized](#sample_bls12_381_optimized)
    - [arithmetic_for_bls12_381_optimized](#arithmetic_for_bls12_381_optimized)
    - [serialization_for_bls12_381_optimized](#serialization_for_bls12_381_optimized)
    - [msm_for_bls12_381_optimized](#msm_for_bls12_381_optimized)
    - [squareroot_for_bls12_381_optimized](#squareroot_for_bls12_381_optimized)
    - [bitwise_operations_for_bls12_381_optimized](#bitwise_operations_for_bls12_381_optimized)
    - [conversions_for_bls12_381_optimized](#conversions_for_bls12_381_optimized)

## Benchmark Results

### sample_bls12_381_optimized

|        | `g1projectivebls12_381_elements`          | `g2projectivebls12_381_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `305.43 us` (✅ **1.00x**)                 | `2.09 ms` (❌ *6.86x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.23 us` (✅ **1.00x**)          | `3.88 us` (❌ *3.15x slower*)     | `23.42 ns` (🚀 **52.63x faster**) | `195.00 ns` (🚀 **6.32x faster**)  | `12.69 ns` (🚀 **97.13x faster**) | `8.65 ns` (🚀 **142.53x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.95 us` (❌ *3.10x slower*)     | `23.39 ns` (🚀 **54.42x faster**) | `161.67 ns` (🚀 **7.87x faster**)  | `13.08 ns` (🚀 **97.29x faster**) | `8.77 ns` (🚀 **145.10x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `895.17 ns` (✅ **1.00x**)        | `2.80 us` (❌ *3.13x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `924.31 ns` (✅ **1.00x**)        | `2.84 us` (❌ *3.07x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `608.57 ns` (✅ **1.00x**)        | `1.80 us` (❌ *2.95x slower*)     | `12.68 ns` (🚀 **47.98x faster**) | `71.86 ns` (🚀 **8.47x faster**)   | `7.22 ns` (🚀 **84.34x faster**)  | `5.92 ns` (🚀 **102.84x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `424.46 us` (✅ **1.00x**)        | `1.25 ms` (❌ *2.96x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.94 ns` (❌ *3.74x slower*)    | `95.34 ns` (❌ *15.55x slower*)    | `18.42 ns` (❌ *3.00x slower*)    | `6.13 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.22 ns` (❌ *5.27x slower*)   | `6.27 us` (❌ *136.25x slower*)    | `78.58 ns` (❌ *1.71x slower*)    | `46.00 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `172.85 ns` (❌ *4.59x slower*)   | `4.37 us` (❌ *115.98x slower*)    | `65.23 ns` (❌ *1.73x slower*)    | `37.69 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.40 us` (❌ *2.09x slower*)    | `25.68 us` (❌ *3.49x slower*)     | `15.06 us` (❌ *2.05x slower*)    | `7.35 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `528.10 ns` (❌ *6.20x slower*)   | `12.79 us` (❌ *150.19x slower*)   | `118.49 ns` (❌ *1.39x slower*)   | `85.15 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `515.73 ns` (❌ *5.89x slower*)   | `12.73 us` (❌ *145.26x slower*)   | `164.34 ns` (❌ *1.88x slower*)   | `87.63 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.63 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.67 ns` (✅ **1.00x**)        | `10.41 ns` (❌ *1.20x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.79 ns` (✅ **1.00x**)        | `4.80 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `155.53 ns` (✅ **1.00x**)        | `201.01 ns` (❌ *1.29x slower*)   | `32.69 ns` (🚀 **4.76x faster**)    | `56.14 ns` (🚀 **2.77x faster**)    | `109.59 ns` (✅ **1.42x faster**)    | `724.91 ns` (❌ *4.66x slower*)    |
| **`serialize_uncompressed`**             | `194.52 ns` (✅ **1.00x**)        | `290.61 ns` (❌ *1.49x slower*)   | `33.10 ns` (🚀 **5.88x faster**)    | `55.22 ns` (🚀 **3.52x faster**)    | `109.55 ns` (✅ **1.78x faster**)    | `728.68 ns` (❌ *3.75x slower*)    |
| **`deserialize_compressed`**             | `329.32 us` (✅ **1.00x**)        | `562.66 us` (❌ *1.71x slower*)   | `51.80 ns` (🚀 **6357.94x faster**) | `94.37 ns` (🚀 **3489.45x faster**) | `216.38 ns` (🚀 **1521.94x faster**) | `1.32 us` (🚀 **249.57x faster**)  |
| **`deserialize_compressed_unchecked`**   | `40.36 us` (✅ **1.00x**)         | `137.35 us` (❌ *3.40x slower*)   | `51.75 ns` (🚀 **780.06x faster**)  | `94.35 ns` (🚀 **427.81x faster**)  | `216.41 ns` (🚀 **186.52x faster**)  | `1.32 us` (🚀 **30.62x faster**)   |
| **`deserialize_uncompressed`**           | `289.59 us` (✅ **1.00x**)        | `425.62 us` (❌ *1.47x slower*)   | `50.99 ns` (🚀 **5679.41x faster**) | `94.31 ns` (🚀 **3070.78x faster**) | `216.22 ns` (🚀 **1339.37x faster**) | `1.32 us` (🚀 **219.49x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.07 ns` (✅ **1.00x**)        | `470.89 ns` (❌ *2.11x slower*)   | `51.60 ns` (🚀 **4.32x faster**)    | `94.16 ns` (🚀 **2.37x faster**)    | `216.30 ns` (✅ **1.03x faster**)    | `1.32 us` (❌ *5.91x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.37 s` (✅ **1.00x**)           | `7.02 s` (❌ *2.96x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.30 us` (✅ **1.00x**) | `40.06 us` (❌ *1.58x slower*)   | `136.38 us` (❌ *5.39x slower*)    |
| **`legendre_for_qr`**    | `14.37 us` (✅ **1.00x**) | `40.11 us` (❌ *2.79x slower*)   | `39.97 us` (❌ *2.78x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.07 ns` (✅ **1.00x**)       | `90.21 ns` (❌ *1.84x slower*)    |
| **`from_big-endian_bits`**    | `49.17 ns` (✅ **1.00x**)       | `89.11 ns` (❌ *1.81x slower*)    |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)        | `5.08 ns` (✅ **1.04x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.15 ns` (✅ **1.00x**) | `76.40 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.56 ns` (✅ **1.00x**) | `48.03 ns` (❌ *2.13x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

