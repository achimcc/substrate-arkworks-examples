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
|        | `1.75 ms` (✅ **1.00x**)                 | `1.74 ms` (✅ **1.01x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x slower**) | `92.35 ns` (🚀 **44.79x faster**) | `181.11 ns` (🚀 **22.84x faster**) | `30.88 ns` (🚀 **133.97x faster**) | `19.31 ns` (🚀 **214.23x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.19 us` (✅ **1.00x**)        | `4.19 us` (✅ **1.00x slower**) | `87.73 ns` (🚀 **47.75x faster**) | `168.72 ns` (🚀 **24.83x faster**) | `28.87 ns` (🚀 **145.12x faster**) | `15.23 ns` (🚀 **274.98x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.93 us` (✅ **1.00x**)        | `2.93 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.96 us` (✅ **1.00x**)        | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.93 us` (✅ **1.00x**)        | `1.93 us` (✅ **1.00x slower**) | `72.35 ns` (🚀 **26.72x faster**) | `145.09 ns` (🚀 **13.32x faster**) | `24.14 ns` (🚀 **80.09x faster**)  | `11.19 ns` (🚀 **172.77x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.55 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.91 ns` (❌ *4.23x slower*)    | `123.74 ns` (❌ *7.38x slower*)    | `24.27 ns` (❌ *1.45x slower*)     | `16.77 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.18 us` (❌ *31.44x slower*)    | `6.91 us` (❌ *99.62x slower*)     | `270.45 ns` (❌ *3.90x slower*)    | `69.41 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.57 us` (❌ *26.61x slower*)    | `4.88 us` (❌ *82.51x slower*)     | `216.87 ns` (❌ *3.67x slower*)    | `59.16 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.77 us` (❌ *3.90x slower*)    | `58.49 us` (❌ *4.49x slower*)     | `47.23 us` (❌ *3.63x slower*)     | `13.02 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.49 us` (❌ *42.34x slower*)    | `14.10 us` (❌ *133.13x slower*)   | `401.38 ns` (❌ *3.79x slower*)    | `105.93 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.42 us` (❌ *28.29x slower*)    | `13.99 us` (❌ *89.44x slower*)    | `569.14 ns` (❌ *3.64x slower*)    | `156.36 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.84 ns` (✅ **1.00x**)        | `15.66 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.74 ns` (✅ **1.00x**)       | `21.21 ns` (❌ *1.97x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.99 ns` (✅ **1.00x**)      | `469.09 ns` (✅ **1.00x slower**) | `50.32 ns` (🚀 **9.32x faster**)     | `156.19 ns` (🚀 **3.00x faster**)    | `464.20 ns` (✅ **1.01x faster**)    | `979.41 ns` (❌ *2.09x slower*)    |
| **`serialize_uncompressed`**             | `629.89 ns` (✅ **1.00x**)      | `628.73 ns` (✅ **1.00x faster**) | `50.68 ns` (🚀 **12.43x faster**)    | `156.92 ns` (🚀 **4.01x faster**)    | `464.16 ns` (✅ **1.36x faster**)    | `983.01 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.37 ms` (✅ **1.00x faster**)   | `93.87 ns` (🚀 **14578.23x faster**) | `307.04 ns` (🚀 **4456.80x faster**) | `942.36 ns` (🚀 **1452.13x faster**) | `1.90 us` (🚀 **719.21x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.66 us` (✅ **1.00x**)      | `252.64 us` (✅ **1.00x faster**) | `93.22 ns` (🚀 **2710.43x faster**)  | `305.67 ns` (🚀 **826.58x faster**)  | `941.74 ns` (🚀 **268.29x faster**)  | `1.90 us` (🚀 **132.78x faster**)  |
| **`deserialize_uncompressed`**           | `1.12 ms` (✅ **1.00x**)        | `1.11 ms` (✅ **1.00x faster**)   | `93.31 ns` (🚀 **11950.45x faster**) | `305.87 ns` (🚀 **3645.82x faster**) | `946.42 ns` (🚀 **1178.28x faster**) | `1.90 us` (🚀 **585.73x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `694.86 ns` (✅ **1.00x**)      | `692.64 ns` (✅ **1.00x faster**) | `93.33 ns` (🚀 **7.44x faster**)     | `306.67 ns` (🚀 **2.27x faster**)    | `942.19 ns` (❌ *1.36x slower*)      | `1.90 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `11.02 s` (✅ **1.00x**)        | `11.01 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `65.62 us` (✅ **1.00x**) | `251.42 us` (❌ *3.83x slower*)   | `6.00 ms` (❌ *91.38x slower*)     |
| **`legendre_for_qr`**    | `29.37 us` (✅ **1.00x**) | `251.17 us` (❌ *8.55x slower*)   | `256.69 us` (❌ *8.74x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `109.34 ns` (✅ **1.00x**)      | `213.37 ns` (❌ *1.95x slower*)    |
| **`from_big-endian_bits`**    | `109.38 ns` (✅ **1.00x**)      | `213.28 ns` (❌ *1.95x slower*)    |
| **`comparison`**              | `4.21 ns` (✅ **1.00x**)        | `4.16 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)        | `5.05 ns` (✅ **1.08x slower**)    |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.41 ns` (✅ **1.00x**) | `278.27 ns` (❌ *3.50x slower*)    |
| **`into_bigint`** | `41.54 ns` (✅ **1.00x**) | `142.35 ns` (❌ *3.43x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

