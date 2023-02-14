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
|        | `193.38 us` (✅ **1.00x**)                 | `1.87 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.28 us` (❌ *3.79x slower*)     | `26.57 ns` (🚀 **42.55x faster**)  | `177.41 ns` (🚀 **6.37x faster**)  | `19.06 ns` (🚀 **59.32x faster**) | `8.30 ns` (🚀 **136.18x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.17 us` (✅ **1.00x**)          | `4.32 us` (❌ *3.71x slower*)     | `27.32 ns` (🚀 **42.68x faster**)  | `172.55 ns` (🚀 **6.76x faster**)  | `18.76 ns` (🚀 **62.15x faster**) | `8.59 ns` (🚀 **135.74x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `826.09 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.73x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `853.54 ns` (✅ **1.00x**)        | `3.12 us` (❌ *3.65x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `571.94 ns` (✅ **1.00x**)        | `2.05 us` (❌ *3.59x slower*)     | `12.82 ns` (🚀 **44.63x faster**)  | `104.40 ns` (🚀 **5.48x faster**)  | `11.05 ns` (🚀 **51.74x faster**) | `5.30 ns` (🚀 **108.01x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `300.38 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.59x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.61 ns` (❌ *3.80x slower*)     | `101.62 ns` (❌ *17.08x slower*)   | `16.75 ns` (❌ *2.81x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.69 ns` (❌ *7.18x slower*)    | `6.66 us` (❌ *178.69x slower*)    | `69.35 ns` (❌ *1.86x slower*)    | `37.27 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `249.37 ns` (❌ *7.83x slower*)    | `4.69 us` (❌ *147.26x slower*)    | `59.03 ns` (❌ *1.85x slower*)    | `31.84 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.71 us` (❌ *2.17x slower*)     | `24.98 us` (❌ *3.96x slower*)     | `13.35 us` (❌ *2.12x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `569.44 ns` (❌ *10.74x slower*)   | `13.56 us` (❌ *255.64x slower*)   | `111.72 ns` (❌ *2.11x slower*)   | `53.04 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `553.51 ns` (❌ *6.73x slower*)    | `13.48 us` (❌ *163.95x slower*)   | `155.96 ns` (❌ *1.90x slower*)   | `82.23 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.75 ns` (✅ **1.00x**)        | `211.66 ns` (❌ *1.40x slower*)   | `27.87 ns` (🚀 **5.41x faster**)    | `50.28 ns` (🚀 **3.00x faster**)    | `100.10 ns` (✅ **1.51x faster**)    | `626.55 ns` (❌ *4.16x slower*)    |
| **`serialize_uncompressed`**             | `198.01 ns` (✅ **1.00x**)        | `326.48 ns` (❌ *1.65x slower*)   | `27.79 ns` (🚀 **7.13x faster**)    | `50.13 ns` (🚀 **3.95x faster**)    | `100.09 ns` (🚀 **1.98x faster**)    | `626.51 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `282.37 us` (✅ **1.00x**)        | `972.41 us` (❌ *3.44x slower*)   | `44.97 ns` (🚀 **6279.26x faster**) | `93.20 ns` (🚀 **3029.80x faster**) | `205.88 ns` (🚀 **1371.53x faster**) | `1.25 us` (🚀 **225.25x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.08 us` (✅ **1.00x**)         | `173.52 us` (❌ *2.67x slower*)   | `44.95 ns` (🚀 **1447.98x faster**) | `93.19 ns` (🚀 **698.36x faster**)  | `206.00 ns` (🚀 **315.93x faster**)  | `1.25 us` (🚀 **51.91x faster**)   |
| **`deserialize_uncompressed`**           | `217.41 us` (✅ **1.00x**)        | `795.88 us` (❌ *3.66x slower*)   | `44.88 ns` (🚀 **4843.88x faster**) | `93.17 ns` (🚀 **2333.40x faster**) | `205.88 ns` (🚀 **1056.00x faster**) | `1.26 us` (🚀 **173.23x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.73 ns` (✅ **1.00x**)        | `477.36 ns` (❌ *2.11x slower*)   | `44.88 ns` (🚀 **5.05x faster**)    | `93.17 ns` (🚀 **2.43x faster**)    | `205.93 ns` (✅ **1.10x faster**)    | `1.26 us` (❌ *5.54x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.25 s` (✅ **1.00x**)           | `7.90 s` (❌ *3.52x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.65 us` (✅ **1.00x**) | `64.72 us` (❌ *2.34x slower*)   | `173.11 us` (❌ *6.26x slower*)    |
| **`legendre_for_qr`**    | `9.57 us` (✅ **1.00x**)  | `29.06 us` (❌ *3.04x slower*)   | `29.55 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.63 ns` (✅ **1.00x**)       | `107.59 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `60.60 ns` (✅ **1.00x**)       | `107.78 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.85 ns` (✅ **1.00x**) | `79.50 ns` (❌ *2.22x slower*)    |
| **`into_bigint`** | `21.71 ns` (✅ **1.00x**) | `41.53 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

