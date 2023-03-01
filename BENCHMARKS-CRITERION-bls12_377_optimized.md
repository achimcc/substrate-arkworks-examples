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
|        | `211.55 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.26 us` (✅ **1.00x**)          | `4.52 us` (❌ *3.59x slower*)     | `22.88 ns` (🚀 **55.13x faster**) | `188.16 ns` (🚀 **6.70x faster**)  | `12.48 ns` (🚀 **101.09x faster**) | `8.70 ns` (🚀 **144.91x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.31 us` (✅ **1.00x**)          | `4.57 us` (❌ *3.50x slower*)     | `23.34 ns` (🚀 **55.94x faster**) | `160.06 ns` (🚀 **8.16x faster**)  | `12.76 ns` (🚀 **102.36x faster**) | `8.81 ns` (🚀 **148.28x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `941.18 ns` (✅ **1.00x**)        | `3.27 us` (❌ *3.48x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `952.97 ns` (✅ **1.00x**)        | `3.30 us` (❌ *3.47x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `607.05 ns` (✅ **1.00x**)        | `2.22 us` (❌ *3.66x slower*)     | `12.39 ns` (🚀 **49.00x faster**) | `67.30 ns` (🚀 **9.02x faster**)   | `7.14 ns` (🚀 **85.02x faster**)   | `5.86 ns` (🚀 **103.52x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `339.40 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.48x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.42 ns` (❌ *3.64x slower*)    | `95.39 ns` (❌ *15.50x slower*)    | `18.59 ns` (❌ *3.02x slower*)     | `6.15 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.09 ns` (❌ *6.04x slower*)   | `7.10 us` (❌ *160.44x slower*)    | `75.90 ns` (❌ *1.72x slower*)     | `44.24 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `233.73 ns` (❌ *6.31x slower*)   | `5.00 us` (❌ *135.11x slower*)    | `66.80 ns` (❌ *1.80x slower*)     | `37.01 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.11 us` (❌ *2.14x slower*)    | `27.47 us` (❌ *3.88x slower*)     | `14.75 us` (❌ *2.08x slower*)     | `7.07 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `577.18 ns` (❌ *9.34x slower*)   | `14.56 us` (❌ *235.52x slower*)   | `118.03 ns` (❌ *1.91x slower*)    | `61.82 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `563.90 ns` (❌ *6.17x slower*)   | `14.44 us` (❌ *157.91x slower*)   | `162.59 ns` (❌ *1.78x slower*)    | `91.41 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `8.72 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**)        | `10.50 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.73 ns` (✅ **1.00x**)        | `4.70 ns` (✅ **1.01x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `160.06 ns` (✅ **1.00x**)        | `221.90 ns` (❌ *1.39x slower*)   | `31.08 ns` (🚀 **5.15x faster**)    | `56.79 ns` (🚀 **2.82x faster**)    | `109.93 ns` (✅ **1.46x faster**)    | `707.58 ns` (❌ *4.42x slower*)    |
| **`serialize_uncompressed`**             | `210.77 ns` (✅ **1.00x**)        | `352.60 ns` (❌ *1.67x slower*)   | `30.69 ns` (🚀 **6.87x faster**)    | `56.27 ns` (🚀 **3.75x faster**)    | `109.96 ns` (🚀 **1.92x faster**)    | `703.57 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `315.98 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.36x slower*)     | `51.88 ns` (🚀 **6090.75x faster**) | `93.23 ns` (🚀 **3389.10x faster**) | `212.89 ns` (🚀 **1484.22x faster**) | `1.34 us` (🚀 **236.49x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.98 us` (✅ **1.00x**)         | `183.82 us` (❌ *2.70x slower*)   | `51.85 ns` (🚀 **1311.07x faster**) | `93.58 ns` (🚀 **726.51x faster**)  | `212.50 ns` (🚀 **319.93x faster**)  | `1.34 us` (🚀 **50.86x faster**)   |
| **`deserialize_uncompressed`**           | `247.99 us` (✅ **1.00x**)        | `875.91 us` (❌ *3.53x slower*)   | `51.65 ns` (🚀 **4801.00x faster**) | `92.87 ns` (🚀 **2670.17x faster**) | `212.51 ns` (🚀 **1166.94x faster**) | `1.34 us` (🚀 **185.46x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `230.40 ns` (✅ **1.00x**)        | `468.22 ns` (❌ *2.03x slower*)   | `51.58 ns` (🚀 **4.47x faster**)    | `93.26 ns` (🚀 **2.47x faster**)    | `212.44 ns` (✅ **1.08x faster**)    | `1.34 us` (❌ *5.80x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.37 s` (✅ **1.00x**)           | `8.29 s` (❌ *3.50x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.08 us` (✅ **1.00x**) | `67.57 us` (❌ *2.17x slower*)   | `182.80 us` (❌ *5.88x slower*)    |
| **`legendre_for_qr`**    | `11.00 us` (✅ **1.00x**) | `31.71 us` (❌ *2.88x slower*)   | `31.56 us` (❌ *2.87x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.89 ns` (✅ **1.00x**)       | `89.34 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `48.88 ns` (✅ **1.00x**)       | `88.93 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.75 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.77 ns` (✅ **1.00x**) | `75.44 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `23.77 ns` (✅ **1.00x**) | `46.88 ns` (❌ *1.97x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

