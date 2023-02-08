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
|        | `210.00 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.75x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.24 us` (✅ **1.00x**)          | `4.56 us` (❌ *3.69x slower*)     | `23.06 ns` (🚀 **53.68x faster**) | `190.40 ns` (🚀 **6.50x faster**)  | `12.48 ns` (🚀 **99.17x faster**)  | `8.71 ns` (🚀 **142.07x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.61 us` (❌ *3.58x slower*)     | `23.24 ns` (🚀 **55.43x faster**) | `159.63 ns` (🚀 **8.07x faster**)  | `12.78 ns` (🚀 **100.83x faster**) | `8.80 ns` (🚀 **146.37x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `903.27 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.66x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `934.39 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.58x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `609.41 ns` (✅ **1.00x**)        | `2.26 us` (❌ *3.70x slower*)     | `12.35 ns` (🚀 **49.33x faster**) | `67.33 ns` (🚀 **9.05x faster**)   | `7.13 ns` (🚀 **85.44x faster**)   | `5.92 ns` (🚀 **102.88x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.84 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.45x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.38 ns` (❌ *3.63x slower*)    | `95.21 ns` (❌ *15.46x slower*)    | `18.31 ns` (❌ *2.97x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `268.07 ns` (❌ *6.21x slower*)   | `7.18 us` (❌ *166.29x slower*)    | `75.99 ns` (❌ *1.76x slower*)     | `43.17 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `239.94 ns` (❌ *6.57x slower*)   | `5.05 us` (❌ *138.22x slower*)    | `66.44 ns` (❌ *1.82x slower*)     | `36.52 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.09 us` (❌ *2.14x slower*)    | `27.53 us` (❌ *3.90x slower*)     | `14.75 us` (❌ *2.09x slower*)     | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `577.83 ns` (❌ *9.41x slower*)   | `14.64 us` (❌ *238.40x slower*)   | `117.67 ns` (❌ *1.92x slower*)    | `61.40 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `565.92 ns` (❌ *6.18x slower*)   | `14.54 us` (❌ *158.91x slower*)   | `162.49 ns` (❌ *1.78x slower*)    | `91.50 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**)        | `10.52 ns` (❌ *1.22x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `158.70 ns` (✅ **1.00x**)        | `219.51 ns` (❌ *1.38x slower*)   | `30.79 ns` (🚀 **5.15x faster**)    | `55.42 ns` (🚀 **2.86x faster**)    | `109.99 ns` (✅ **1.44x faster**)    | `706.02 ns` (❌ *4.45x slower*)    |
| **`serialize_uncompressed`**             | `208.67 ns` (✅ **1.00x**)        | `337.77 ns` (❌ *1.62x slower*)   | `32.72 ns` (🚀 **6.38x faster**)    | `55.42 ns` (🚀 **3.77x faster**)    | `110.07 ns` (🚀 **1.90x faster**)    | `706.82 ns` (❌ *3.39x slower*)    |
| **`deserialize_compressed`**             | `314.46 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.37x slower*)     | `51.63 ns` (🚀 **6090.69x faster**) | `92.75 ns` (🚀 **3390.59x faster**) | `209.56 ns` (🚀 **1500.56x faster**) | `1.27 us` (🚀 **248.22x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.48 us` (✅ **1.00x**)         | `182.30 us` (❌ *2.70x slower*)   | `51.61 ns` (🚀 **1307.65x faster**) | `92.76 ns` (🚀 **727.49x faster**)  | `209.73 ns` (🚀 **321.77x faster**)  | `1.27 us` (🚀 **53.27x faster**)   |
| **`deserialize_uncompressed`**           | `247.17 us` (✅ **1.00x**)        | `875.41 us` (❌ *3.54x slower*)   | `51.60 ns` (🚀 **4790.35x faster**) | `92.97 ns` (🚀 **2658.63x faster**) | `209.83 ns` (🚀 **1177.99x faster**) | `1.27 us` (🚀 **194.85x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `239.75 ns` (✅ **1.00x**)        | `464.34 ns` (❌ *1.94x slower*)   | `51.58 ns` (🚀 **4.65x faster**)    | `93.10 ns` (🚀 **2.58x faster**)    | `209.94 ns` (✅ **1.14x faster**)    | `1.27 us` (❌ *5.30x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.32 s` (❌ *3.53x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.13 us` (✅ **1.00x**) | `66.93 us` (❌ *2.15x slower*)   | `181.42 us` (❌ *5.83x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.62 us` (❌ *2.89x slower*)   | `31.53 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `50.33 ns` (✅ **1.00x**)       | `84.34 ns` (❌ *1.68x slower*)    |
| **`from_big-endian_bits`**    | `50.24 ns` (✅ **1.00x**)       | `84.22 ns` (❌ *1.68x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.38 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.59 ns` (✅ **1.00x**) | `75.11 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `23.79 ns` (✅ **1.00x**) | `46.88 ns` (❌ *1.97x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

