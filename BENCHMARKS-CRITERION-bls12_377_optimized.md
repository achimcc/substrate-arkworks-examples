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
|        | `210.20 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.74x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.24 us` (✅ **1.00x**)          | `4.56 us` (❌ *3.68x slower*)     | `23.04 ns` (🚀 **53.77x faster**) | `194.95 ns` (🚀 **6.35x faster**)  | `12.49 ns` (🚀 **99.17x faster**)  | `8.71 ns` (🚀 **142.18x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.61 us` (❌ *3.59x slower*)     | `23.25 ns` (🚀 **55.35x faster**) | `158.60 ns` (🚀 **8.11x faster**)  | `12.75 ns` (🚀 **100.93x faster**) | `8.81 ns` (🚀 **146.01x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `904.08 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.66x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `936.04 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.58x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `608.26 ns` (✅ **1.00x**)        | `2.26 us` (❌ *3.71x slower*)     | `12.34 ns` (🚀 **49.28x faster**) | `68.81 ns` (🚀 **8.84x faster**)   | `7.14 ns` (🚀 **85.17x faster**)   | `5.92 ns` (🚀 **102.74x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.77 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.45x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.16 ns` (❌ *3.76x slower*)    | `96.29 ns` (❌ *15.63x slower*)    | `18.77 ns` (❌ *3.05x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.31 ns` (❌ *6.04x slower*)   | `7.18 us` (❌ *162.15x slower*)    | `76.16 ns` (❌ *1.72x slower*)     | `44.25 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `239.63 ns` (❌ *6.62x slower*)   | `5.04 us` (❌ *139.39x slower*)    | `66.46 ns` (❌ *1.84x slower*)     | `36.19 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.12 us` (❌ *2.14x slower*)    | `27.61 us` (❌ *3.92x slower*)     | `14.77 us` (❌ *2.10x slower*)     | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `578.34 ns` (❌ *9.42x slower*)   | `14.67 us` (❌ *239.04x slower*)   | `117.69 ns` (❌ *1.92x slower*)    | `61.38 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `565.46 ns` (❌ *6.18x slower*)   | `14.57 us` (❌ *159.23x slower*)   | `162.57 ns` (❌ *1.78x slower*)    | `91.52 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.50 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `158.52 ns` (✅ **1.00x**)        | `220.53 ns` (❌ *1.39x slower*)   | `31.44 ns` (🚀 **5.04x faster**)    | `55.78 ns` (🚀 **2.84x faster**)    | `109.69 ns` (✅ **1.45x faster**)    | `706.08 ns` (❌ *4.45x slower*)    |
| **`serialize_uncompressed`**             | `208.11 ns` (✅ **1.00x**)        | `338.06 ns` (❌ *1.62x slower*)   | `32.61 ns` (🚀 **6.38x faster**)    | `55.71 ns` (🚀 **3.74x faster**)    | `110.18 ns` (🚀 **1.89x faster**)    | `705.17 ns` (❌ *3.39x slower*)    |
| **`deserialize_compressed`**             | `314.40 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.37x slower*)     | `51.57 ns` (🚀 **6096.11x faster**) | `94.22 ns` (🚀 **3336.87x faster**) | `210.38 ns` (🚀 **1494.44x faster**) | `1.27 us` (🚀 **247.10x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.41 us` (✅ **1.00x**)         | `182.38 us` (❌ *2.71x slower*)   | `51.62 ns` (🚀 **1305.91x faster**) | `94.20 ns` (🚀 **715.64x faster**)  | `210.24 ns` (🚀 **320.64x faster**)  | `1.27 us` (🚀 **53.06x faster**)   |
| **`deserialize_uncompressed`**           | `247.13 us` (✅ **1.00x**)        | `875.40 us` (❌ *3.54x slower*)   | `51.60 ns` (🚀 **4789.15x faster**) | `94.27 ns` (🚀 **2621.43x faster**) | `209.44 ns` (🚀 **1179.97x faster**) | `1.27 us` (🚀 **194.06x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.52 ns` (✅ **1.00x**)        | `466.84 ns` (❌ *2.09x slower*)   | `51.53 ns` (🚀 **4.34x faster**)    | `94.40 ns` (🚀 **2.37x faster**)    | `209.53 ns` (✅ **1.07x faster**)    | `1.27 us` (❌ *5.68x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.31 s` (❌ *3.52x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.14 us` (✅ **1.00x**) | `66.91 us` (❌ *2.15x slower*)   | `181.53 us` (❌ *5.83x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.63 us` (❌ *2.89x slower*)   | `31.53 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.81 ns` (✅ **1.00x**)       | `83.93 ns` (❌ *1.72x slower*)    |
| **`from_big-endian_bits`**    | `48.82 ns` (✅ **1.00x**)       | `83.56 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.38 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.90 ns` (✅ **1.00x**) | `75.15 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `23.79 ns` (✅ **1.00x**) | `46.96 ns` (❌ *1.97x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

