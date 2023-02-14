# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761_optimized](#sample_bw6_761_optimized)
    - [arithmetic_for_bw6_761_optimized](#arithmetic_for_bw6_761_optimized)
    - [serialization_for_bw6_761_optimized](#serialization_for_bw6_761_optimized)
    - [msm_for_bw6_761_optimized](#msm_for_bw6_761_optimized)
    - [squareroot_for_bw6_761_optimized](#squareroot_for_bw6_761_optimized)
    - [bitwise_operations_for_bw6_761_optimized](#bitwise_operations_for_bw6_761_optimized)
    - [conversions_for_bw6_761_optimized](#conversions_for_bw6_761_optimized)

## Benchmark Results

### sample_bw6_761_optimized

|        | `g1projectivebw6_761_elements`          | `g2projectivebw6_761_elements`           |
|:-------|:----------------------------------------|:---------------------------------------- |
|        | `2.02 ms` (✅ **1.00x**)                 | `2.01 ms` (✅ **1.00x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x faster**) | `76.20 ns` (🚀 **63.23x faster**) | `158.84 ns` (🚀 **30.33x faster**) | `27.77 ns` (🚀 **173.52x faster**) | `12.63 ns` (🚀 **381.57x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.88 us` (✅ **1.00x**)        | `4.88 us` (✅ **1.00x slower**) | `77.13 ns` (🚀 **63.28x faster**) | `152.25 ns` (🚀 **32.06x faster**) | `25.94 ns` (🚀 **188.18x faster**) | `13.43 ns` (🚀 **363.31x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.40 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.21 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `54.76 ns` (🚀 **40.41x faster**) | `117.20 ns` (🚀 **18.88x faster**) | `19.06 ns` (🚀 **116.10x faster**) | `7.16 ns` (🚀 **308.85x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.79 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `71.13 ns` (❌ *3.89x slower*)    | `119.96 ns` (❌ *6.55x slower*)    | `22.26 ns` (❌ *1.22x slower*)     | `18.30 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.47 us` (❌ *32.49x slower*)    | `7.91 us` (❌ *104.17x slower*)    | `313.57 ns` (❌ *4.13x slower*)    | `75.98 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.87x slower*)    | `5.52 us` (❌ *83.16x slower*)     | `244.68 ns` (❌ *3.69x slower*)    | `66.38 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.77 us` (❌ *3.56x slower*)    | `60.84 us` (❌ *4.18x slower*)     | `47.73 us` (❌ *3.28x slower*)     | `14.56 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.44x slower*)    | `16.09 us` (❌ *136.62x slower*)   | `418.16 ns` (❌ *3.55x slower*)    | `117.80 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.81x slower*)    | `16.03 us` (❌ *98.12x slower*)    | `648.30 ns` (❌ *3.97x slower*)    | `163.42 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)        | `17.21 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**)       | `21.81 ns` (❌ *2.11x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.88 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `512.87 ns` (✅ **1.00x**)      | `514.96 ns` (✅ **1.00x slower**) | `58.21 ns` (🚀 **8.81x faster**)     | `171.75 ns` (🚀 **2.99x faster**)    | `516.10 ns` (✅ **1.01x slower**)  | `1.09 us` (❌ *2.12x slower*)      |
| **`serialize_uncompressed`**             | `695.54 ns` (✅ **1.00x**)      | `697.17 ns` (✅ **1.00x slower**) | `56.12 ns` (🚀 **12.39x faster**)    | `169.86 ns` (🚀 **4.09x faster**)    | `516.10 ns` (✅ **1.35x faster**)  | `1.09 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x slower**)   | `92.44 ns` (🚀 **17180.77x faster**) | `341.86 ns` (🚀 **4645.75x faster**) | `1.04 us` (🚀 **1525.21x faster**) | `2.10 us` (🚀 **757.05x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.87 us` (✅ **1.00x**)      | `291.86 us` (✅ **1.00x faster**) | `92.40 ns` (🚀 **3158.68x faster**)  | `341.90 ns` (🚀 **853.67x faster**)  | `1.04 us` (🚀 **280.29x faster**)  | `2.10 us` (🚀 **139.09x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `92.22 ns` (🚀 **14060.21x faster**) | `341.84 ns` (🚀 **3793.04x faster**) | `1.04 us` (🚀 **1245.31x faster**) | `2.10 us` (🚀 **618.01x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `763.95 ns` (✅ **1.00x**)      | `770.28 ns` (✅ **1.01x slower**) | `92.26 ns` (🚀 **8.28x faster**)     | `341.79 ns` (🚀 **2.24x faster**)    | `1.04 us` (❌ *1.36x slower*)      | `2.10 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.42 s` (✅ **1.00x**)        | `12.48 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.27 us` (✅ **1.00x**) | `290.38 us` (❌ *4.32x slower*)   | `6.97 ms` (❌ *103.55x slower*)    |
| **`legendre_for_qr`**    | `32.30 us` (✅ **1.00x**) | `292.53 us` (❌ *9.06x slower*)   | `297.03 us` (❌ *9.20x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.43 ns` (✅ **1.00x**)       | `165.32 ns` (❌ *1.98x slower*)    |
| **`from_big-endian_bits`**    | `83.67 ns` (✅ **1.00x**)       | `165.35 ns` (❌ *1.98x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.68 ns` (✅ **1.00x**)        | `5.76 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.27 ns` (✅ **1.00x**) | `309.81 ns` (❌ *4.12x slower*)    |
| **`into_bigint`** | `47.04 ns` (✅ **1.00x**) | `159.34 ns` (❌ *3.39x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

