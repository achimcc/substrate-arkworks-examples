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
|        | `211.14 us` (✅ **1.00x**)                 | `2.04 ms` (❌ *9.68x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.58 us` (❌ *3.67x slower*)     | `23.07 ns` (🚀 **54.09x faster**) | `180.71 ns` (🚀 **6.91x faster**)  | `12.52 ns` (🚀 **99.69x faster**)  | `8.70 ns` (🚀 **143.35x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.30 us` (✅ **1.00x**)          | `4.64 us` (❌ *3.58x slower*)     | `23.28 ns` (🚀 **55.67x faster**) | `160.22 ns` (🚀 **8.09x faster**)  | `12.73 ns` (🚀 **101.80x faster**) | `8.79 ns` (🚀 **147.42x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `906.71 ns` (✅ **1.00x**)        | `3.32 us` (❌ *3.66x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `937.47 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.58x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `593.29 ns` (✅ **1.00x**)        | `2.25 us` (❌ *3.79x slower*)     | `12.33 ns` (🚀 **48.11x faster**) | `71.90 ns` (🚀 **8.25x faster**)   | `7.14 ns` (🚀 **83.08x faster**)   | `5.84 ns` (🚀 **101.63x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.93 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.44x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.48 ns` (❌ *3.81x slower*)    | `97.43 ns` (❌ *15.81x slower*)    | `18.73 ns` (❌ *3.04x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.75 ns` (❌ *6.26x slower*)   | `7.10 us` (❌ *163.50x slower*)    | `75.45 ns` (❌ *1.74x slower*)     | `43.42 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.11 ns` (❌ *6.68x slower*)   | `5.03 us` (❌ *138.80x slower*)    | `66.80 ns` (❌ *1.84x slower*)     | `36.25 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.02 us` (❌ *2.13x slower*)    | `27.31 us` (❌ *3.87x slower*)     | `14.62 us` (❌ *2.07x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `590.68 ns` (❌ *9.60x slower*)   | `14.54 us` (❌ *236.26x slower*)   | `117.92 ns` (❌ *1.92x slower*)    | `61.53 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `578.24 ns` (❌ *6.31x slower*)   | `14.54 us` (❌ *158.72x slower*)   | `162.59 ns` (❌ *1.78x slower*)    | `91.60 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.68 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.32 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.62 ns` (✅ **1.00x**)        | `4.63 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `159.99 ns` (✅ **1.00x**)        | `223.51 ns` (❌ *1.40x slower*)   | `31.04 ns` (🚀 **5.15x faster**)    | `56.41 ns` (🚀 **2.84x faster**)    | `111.19 ns` (✅ **1.44x faster**)    | `699.41 ns` (❌ *4.37x slower*)    |
| **`serialize_uncompressed`**             | `209.90 ns` (✅ **1.00x**)        | `347.65 ns` (❌ *1.66x slower*)   | `30.59 ns` (🚀 **6.86x faster**)    | `56.29 ns` (🚀 **3.73x faster**)    | `110.53 ns` (🚀 **1.90x faster**)    | `700.30 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `316.08 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `52.46 ns` (🚀 **6025.01x faster**) | `92.98 ns` (🚀 **3399.24x faster**) | `210.40 ns` (🚀 **1502.25x faster**) | `1.31 us` (🚀 **242.19x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.83 us` (✅ **1.00x**)         | `182.75 us` (❌ *2.69x slower*)   | `52.40 ns` (🚀 **1294.45x faster**) | `93.03 ns` (🚀 **729.08x faster**)  | `209.52 ns` (🚀 **323.73x faster**)  | `1.30 us` (🚀 **52.06x faster**)   |
| **`deserialize_uncompressed`**           | `250.54 us` (✅ **1.00x**)        | `874.79 us` (❌ *3.49x slower*)   | `52.32 ns` (🚀 **4788.87x faster**) | `93.08 ns` (🚀 **2691.62x faster**) | `209.89 ns` (🚀 **1193.66x faster**) | `1.30 us` (🚀 **192.30x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `224.78 ns` (✅ **1.00x**)        | `466.33 ns` (❌ *2.07x slower*)   | `52.32 ns` (🚀 **4.30x faster**)    | `93.11 ns` (🚀 **2.41x faster**)    | `210.00 ns` (✅ **1.07x faster**)    | `1.30 us` (❌ *5.77x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.44 s` (❌ *3.58x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.31 us` (✅ **1.00x**) | `67.26 us` (❌ *2.15x slower*)   | `181.82 us` (❌ *5.81x slower*)    |
| **`legendre_for_qr`**    | `10.97 us` (✅ **1.00x**) | `31.68 us` (❌ *2.89x slower*)   | `32.89 us` (❌ *3.00x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `50.15 ns` (✅ **1.00x**)       | `89.25 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `50.04 ns` (✅ **1.00x**)       | `88.94 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.59 ns` (✅ **1.00x**) | `75.13 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.90 ns` (✅ **1.00x**) | `46.88 ns` (❌ *2.05x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

