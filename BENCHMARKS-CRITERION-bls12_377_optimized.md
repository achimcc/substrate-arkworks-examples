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
|        | `211.67 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.51 us` (❌ *3.62x slower*)     | `22.91 ns` (🚀 **54.40x faster**) | `193.18 ns` (🚀 **6.45x faster**)  | `12.51 ns` (🚀 **99.68x faster**)  | `8.71 ns` (🚀 **143.07x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.58 us` (❌ *3.55x slower*)     | `23.34 ns` (🚀 **55.24x faster**) | `161.22 ns` (🚀 **8.00x faster**)  | `12.75 ns` (🚀 **101.09x faster**) | `8.81 ns` (🚀 **146.40x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `916.24 ns` (✅ **1.00x**)        | `3.27 us` (❌ *3.57x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `927.50 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.57x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `583.75 ns` (✅ **1.00x**)        | `2.22 us` (❌ *3.81x slower*)     | `12.33 ns` (🚀 **47.33x faster**) | `67.66 ns` (🚀 **8.63x faster**)   | `7.14 ns` (🚀 **81.76x faster**)   | `5.86 ns` (🚀 **99.63x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `339.40 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.47x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.87 ns` (❌ *3.72x slower*)    | `97.29 ns` (❌ *15.81x slower*)    | `18.48 ns` (❌ *3.00x slower*)     | `6.15 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `266.45 ns` (❌ *6.00x slower*)   | `7.09 us` (❌ *159.67x slower*)    | `76.00 ns` (❌ *1.71x slower*)     | `44.42 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `232.92 ns` (❌ *6.31x slower*)   | `5.00 us` (❌ *135.29x slower*)    | `66.83 ns` (❌ *1.81x slower*)     | `36.94 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.10 us` (❌ *2.14x slower*)    | `27.48 us` (❌ *3.89x slower*)     | `14.75 us` (❌ *2.09x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `576.96 ns` (❌ *9.33x slower*)   | `14.54 us` (❌ *235.15x slower*)   | `118.01 ns` (❌ *1.91x slower*)    | `61.83 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `564.29 ns` (❌ *6.17x slower*)   | `14.45 us` (❌ *158.01x slower*)   | `162.77 ns` (❌ *1.78x slower*)    | `91.43 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.51 ns` (❌ *1.22x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.70 ns` (✅ **1.00x**)        | `4.70 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `159.61 ns` (✅ **1.00x**)        | `222.11 ns` (❌ *1.39x slower*)   | `31.21 ns` (🚀 **5.11x faster**)    | `56.87 ns` (🚀 **2.81x faster**)    | `110.13 ns` (✅ **1.45x faster**)    | `704.82 ns` (❌ *4.42x slower*)    |
| **`serialize_uncompressed`**             | `209.83 ns` (✅ **1.00x**)        | `346.75 ns` (❌ *1.65x slower*)   | `31.31 ns` (🚀 **6.70x faster**)    | `56.10 ns` (🚀 **3.74x faster**)    | `109.64 ns` (🚀 **1.91x faster**)    | `704.02 ns` (❌ *3.36x slower*)    |
| **`deserialize_compressed`**             | `315.89 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.36x slower*)     | `51.80 ns` (🚀 **6098.33x faster**) | `93.76 ns` (🚀 **3369.06x faster**) | `212.62 ns` (🚀 **1485.69x faster**) | `1.34 us` (🚀 **235.96x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.98 us` (✅ **1.00x**)         | `184.87 us` (❌ *2.72x slower*)   | `51.90 ns` (🚀 **1309.92x faster**) | `93.89 ns` (🚀 **724.12x faster**)  | `213.12 ns` (🚀 **319.00x faster**)  | `1.34 us` (🚀 **50.78x faster**)   |
| **`deserialize_uncompressed`**           | `247.93 us` (✅ **1.00x**)        | `875.88 us` (❌ *3.53x slower*)   | `51.73 ns` (🚀 **4793.26x faster**) | `93.82 ns` (🚀 **2642.54x faster**) | `212.16 ns` (🚀 **1168.61x faster**) | `1.34 us` (🚀 **185.18x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `229.61 ns` (✅ **1.00x**)        | `474.54 ns` (❌ *2.07x slower*)   | `51.91 ns` (🚀 **4.42x faster**)    | `93.90 ns` (🚀 **2.45x faster**)    | `212.14 ns` (✅ **1.08x faster**)    | `1.34 us` (❌ *5.83x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.37 s` (✅ **1.00x**)           | `8.30 s` (❌ *3.51x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.11 us` (✅ **1.00x**) | `67.58 us` (❌ *2.17x slower*)   | `182.76 us` (❌ *5.87x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.68 us` (❌ *2.89x slower*)   | `31.56 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.09 ns` (✅ **1.00x**)       | `88.75 ns` (❌ *1.81x slower*)    |
| **`from_big-endian_bits`**    | `49.06 ns` (✅ **1.00x**)       | `88.81 ns` (❌ *1.81x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.72 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.78 ns` (✅ **1.00x**) | `75.33 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `23.88 ns` (✅ **1.00x**) | `46.87 ns` (❌ *1.96x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

