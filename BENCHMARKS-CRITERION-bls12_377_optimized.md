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
|        | `211.08 us` (✅ **1.00x**)                 | `2.04 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.58 us` (❌ *3.67x slower*)     | `23.13 ns` (🚀 **53.95x faster**) | `181.70 ns` (🚀 **6.87x faster**)  | `12.52 ns` (🚀 **99.72x faster**)  | `8.70 ns` (🚀 **143.45x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.30 us` (✅ **1.00x**)          | `4.63 us` (❌ *3.57x slower*)     | `23.26 ns` (🚀 **55.68x faster**) | `159.66 ns` (🚀 **8.11x faster**)  | `12.73 ns` (🚀 **101.78x faster**) | `8.79 ns` (🚀 **147.34x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `906.94 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.66x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `944.78 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.55x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `592.92 ns` (✅ **1.00x**)        | `2.25 us` (❌ *3.80x slower*)     | `12.31 ns` (🚀 **48.18x faster**) | `70.94 ns` (🚀 **8.36x faster**)   | `7.14 ns` (🚀 **83.02x faster**)   | `5.84 ns` (🚀 **101.57x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `341.05 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.44x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.32 ns` (❌ *3.76x slower*)    | `98.70 ns` (❌ *15.91x slower*)    | `18.73 ns` (❌ *3.02x slower*)     | `6.20 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.08 ns` (❌ *6.26x slower*)   | `7.10 us` (❌ *163.74x slower*)    | `75.61 ns` (❌ *1.74x slower*)     | `43.34 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `243.18 ns` (❌ *6.76x slower*)   | `5.05 us` (❌ *140.44x slower*)    | `66.79 ns` (❌ *1.86x slower*)     | `35.96 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `14.97 us` (❌ *2.12x slower*)    | `27.26 us` (❌ *3.86x slower*)     | `14.59 us` (❌ *2.07x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `590.06 ns` (❌ *9.62x slower*)   | `14.53 us` (❌ *236.96x slower*)   | `117.90 ns` (❌ *1.92x slower*)    | `61.33 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `578.66 ns` (❌ *6.32x slower*)   | `14.49 us` (❌ *158.26x slower*)   | `162.96 ns` (❌ *1.78x slower*)    | `91.56 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.67 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.31 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.63 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.01x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `159.76 ns` (✅ **1.00x**)        | `223.32 ns` (❌ *1.40x slower*)   | `31.28 ns` (🚀 **5.11x faster**)    | `57.04 ns` (🚀 **2.80x faster**)    | `110.31 ns` (✅ **1.45x faster**)    | `701.18 ns` (❌ *4.39x slower*)    |
| **`serialize_uncompressed`**             | `209.55 ns` (✅ **1.00x**)        | `344.39 ns` (❌ *1.64x slower*)   | `30.49 ns` (🚀 **6.87x faster**)    | `55.91 ns` (🚀 **3.75x faster**)    | `110.07 ns` (🚀 **1.90x faster**)    | `699.54 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `316.06 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `52.40 ns` (🚀 **6032.22x faster**) | `92.84 ns` (🚀 **3404.33x faster**) | `210.07 ns` (🚀 **1504.59x faster**) | `1.30 us` (🚀 **242.89x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.83 us` (✅ **1.00x**)         | `184.40 us` (❌ *2.72x slower*)   | `52.38 ns` (🚀 **1294.98x faster**) | `92.84 ns` (🚀 **730.53x faster**)  | `211.38 ns` (🚀 **320.87x faster**)  | `1.29 us` (🚀 **52.45x faster**)   |
| **`deserialize_uncompressed`**           | `248.48 us` (✅ **1.00x**)        | `874.36 us` (❌ *3.52x slower*)   | `52.37 ns` (🚀 **4744.88x faster**) | `92.96 ns` (🚀 **2673.07x faster**) | `209.93 ns` (🚀 **1183.62x faster**) | `1.30 us` (🚀 **190.65x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `224.72 ns` (✅ **1.00x**)        | `467.17 ns` (❌ *2.08x slower*)   | `52.29 ns` (🚀 **4.30x faster**)    | `92.92 ns` (🚀 **2.42x faster**)    | `209.95 ns` (✅ **1.07x faster**)    | `1.30 us` (❌ *5.79x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.39 s` (❌ *3.56x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.31 us` (✅ **1.00x**) | `67.26 us` (❌ *2.15x slower*)   | `181.87 us` (❌ *5.81x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.64 us` (❌ *2.89x slower*)   | `32.88 us` (❌ *3.00x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.96 ns` (✅ **1.00x**)       | `90.23 ns` (❌ *1.84x slower*)    |
| **`from_big-endian_bits`**    | `48.96 ns` (✅ **1.00x**)       | `89.64 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.60 ns` (✅ **1.00x**) | `75.12 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.92 ns` (✅ **1.00x**) | `46.86 ns` (❌ *2.04x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

