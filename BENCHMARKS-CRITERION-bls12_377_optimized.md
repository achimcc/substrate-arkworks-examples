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
|        | `211.54 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.51 us` (❌ *3.61x slower*)     | `23.11 ns` (🚀 **54.01x faster**) | `194.09 ns` (🚀 **6.43x faster**)  | `12.50 ns` (🚀 **99.89x faster**)  | `8.72 ns` (🚀 **143.22x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.57 us` (❌ *3.55x slower*)     | `23.32 ns` (🚀 **55.27x faster**) | `161.47 ns` (🚀 **7.98x faster**)  | `12.75 ns` (🚀 **101.10x faster**) | `8.80 ns` (🚀 **146.43x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `916.85 ns` (✅ **1.00x**)        | `3.28 us` (❌ *3.57x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `927.41 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.56x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `595.14 ns` (✅ **1.00x**)        | `2.23 us` (❌ *3.74x slower*)     | `12.33 ns` (🚀 **48.28x faster**) | `68.92 ns` (🚀 **8.64x faster**)   | `7.12 ns` (🚀 **83.62x faster**)   | `5.86 ns` (🚀 **101.50x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `339.36 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.47x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.88 ns` (❌ *3.72x slower*)    | `96.59 ns` (❌ *15.70x slower*)    | `18.46 ns` (❌ *3.00x slower*)     | `6.15 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `268.18 ns` (❌ *6.11x slower*)   | `7.13 us` (❌ *162.43x slower*)    | `76.05 ns` (❌ *1.73x slower*)     | `43.90 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `233.41 ns` (❌ *6.30x slower*)   | `5.00 us` (❌ *134.93x slower*)    | `66.64 ns` (❌ *1.80x slower*)     | `37.03 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.10 us` (❌ *2.14x slower*)    | `27.47 us` (❌ *3.89x slower*)     | `14.75 us` (❌ *2.09x slower*)     | `7.07 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `578.43 ns` (❌ *9.33x slower*)   | `14.53 us` (❌ *234.38x slower*)   | `117.99 ns` (❌ *1.90x slower*)    | `61.98 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `564.83 ns` (❌ *6.18x slower*)   | `14.44 us` (❌ *157.86x slower*)   | `162.51 ns` (❌ *1.78x slower*)    | `91.47 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `8.66 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.50 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.70 ns` (✅ **1.00x**)        | `4.70 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `159.59 ns` (✅ **1.00x**)        | `221.99 ns` (❌ *1.39x slower*)   | `31.68 ns` (🚀 **5.04x faster**)    | `56.52 ns` (🚀 **2.82x faster**)    | `110.67 ns` (✅ **1.44x faster**)    | `709.26 ns` (❌ *4.44x slower*)    |
| **`serialize_uncompressed`**             | `210.83 ns` (✅ **1.00x**)        | `346.91 ns` (❌ *1.65x slower*)   | `30.84 ns` (🚀 **6.84x faster**)    | `56.15 ns` (🚀 **3.75x faster**)    | `110.65 ns` (🚀 **1.91x faster**)    | `707.99 ns` (❌ *3.36x slower*)    |
| **`deserialize_compressed`**             | `315.80 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.36x slower*)     | `51.82 ns` (🚀 **6094.12x faster**) | `94.64 ns` (🚀 **3336.86x faster**) | `212.16 ns` (🚀 **1488.49x faster**) | `1.34 us` (🚀 **235.91x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.98 us` (✅ **1.00x**)         | `183.76 us` (❌ *2.70x slower*)   | `51.66 ns` (🚀 **1315.78x faster**) | `94.18 ns` (🚀 **721.78x faster**)  | `211.98 ns` (🚀 **320.68x faster**)  | `1.34 us` (🚀 **50.79x faster**)   |
| **`deserialize_uncompressed`**           | `247.86 us` (✅ **1.00x**)        | `875.68 us` (❌ *3.53x slower*)   | `51.77 ns` (🚀 **4787.46x faster**) | `94.73 ns` (🚀 **2616.56x faster**) | `211.74 ns` (🚀 **1170.55x faster**) | `1.34 us` (🚀 **185.16x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `229.66 ns` (✅ **1.00x**)        | `473.23 ns` (❌ *2.06x slower*)   | `51.64 ns` (🚀 **4.45x faster**)    | `94.67 ns` (🚀 **2.43x faster**)    | `211.84 ns` (✅ **1.08x faster**)    | `1.34 us` (❌ *5.83x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.26 s` (❌ *3.49x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.11 us` (✅ **1.00x**) | `67.56 us` (❌ *2.17x slower*)   | `182.71 us` (❌ *5.87x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.69 us` (❌ *2.90x slower*)   | `31.55 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.93 ns` (✅ **1.00x**)       | `88.80 ns` (❌ *1.81x slower*)    |
| **`from_big-endian_bits`**    | `48.90 ns` (✅ **1.00x**)       | `89.17 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.74 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.78 ns` (✅ **1.00x**) | `75.56 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `23.88 ns` (✅ **1.00x**) | `46.85 ns` (❌ *1.96x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

