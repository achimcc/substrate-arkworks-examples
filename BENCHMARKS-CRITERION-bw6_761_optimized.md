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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x slower**) | `90.13 ns` (🚀 **45.93x faster**) | `182.18 ns` (🚀 **22.72x faster**) | `30.20 ns` (🚀 **137.07x faster**) | `19.51 ns` (🚀 **212.22x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.19 us` (✅ **1.00x**)        | `4.20 us` (✅ **1.00x slower**) | `85.34 ns` (🚀 **49.12x faster**) | `168.34 ns` (🚀 **24.90x faster**) | `29.54 ns` (🚀 **141.91x faster**) | `15.13 ns` (🚀 **277.05x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.91 us` (✅ **1.00x**)        | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.97 us` (✅ **1.00x**)        | `2.97 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.93 us` (✅ **1.00x**)        | `1.93 us` (✅ **1.00x slower**) | `68.40 ns` (🚀 **28.20x faster**) | `139.22 ns` (🚀 **13.86x faster**) | `21.85 ns` (🚀 **88.27x faster**)  | `11.14 ns` (🚀 **173.15x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.55 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `77.27 ns` (❌ *4.47x slower*)    | `125.56 ns` (❌ *7.26x slower*)    | `24.57 ns` (❌ *1.42x slower*)     | `17.30 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.16 us` (❌ *31.16x slower*)    | `6.88 us` (❌ *99.14x slower*)     | `271.86 ns` (❌ *3.92x slower*)    | `69.37 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.55 us` (❌ *26.20x slower*)    | `4.85 us` (❌ *81.83x slower*)     | `217.41 ns` (❌ *3.67x slower*)    | `59.23 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.75 us` (❌ *3.78x slower*)    | `58.48 us` (❌ *4.36x slower*)     | `47.23 us` (❌ *3.52x slower*)     | `13.43 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.44 us` (❌ *41.94x slower*)    | `14.00 us` (❌ *132.26x slower*)   | `400.92 ns` (❌ *3.79x slower*)    | `105.89 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.39 us` (❌ *27.67x slower*)    | `13.89 us` (❌ *87.49x slower*)    | `568.28 ns` (❌ *3.58x slower*)    | `158.80 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.84 ns` (✅ **1.00x**)        | `15.70 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**)       | `21.23 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.15 ns` (✅ **1.00x**)      | `468.45 ns` (✅ **1.00x slower**) | `50.91 ns` (🚀 **9.20x faster**)     | `157.60 ns` (🚀 **2.97x faster**)    | `466.36 ns` (✅ **1.00x faster**)    | `984.64 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `628.99 ns` (✅ **1.00x**)      | `630.26 ns` (✅ **1.00x slower**) | `50.67 ns` (🚀 **12.41x faster**)    | `157.34 ns` (🚀 **4.00x faster**)    | `466.35 ns` (✅ **1.35x faster**)    | `985.32 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.36 ms` (✅ **1.00x faster**)   | `95.51 ns` (🚀 **14297.04x faster**) | `306.44 ns` (🚀 **4456.04x faster**) | `942.25 ns` (🚀 **1449.18x faster**) | `1.90 us` (🚀 **719.01x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.50 us` (✅ **1.00x**)      | `252.31 us` (✅ **1.00x faster**) | `93.86 ns` (🚀 **2690.07x faster**)  | `304.98 ns` (🚀 **827.93x faster**)  | `946.80 ns` (🚀 **266.69x faster**)  | `1.91 us` (🚀 **132.28x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)        | `1.11 ms` (✅ **1.00x faster**)   | `93.76 ns` (🚀 **11876.57x faster**) | `306.56 ns` (🚀 **3632.25x faster**) | `942.67 ns` (🚀 **1181.24x faster**) | `1.90 us` (🚀 **586.21x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `697.19 ns` (✅ **1.00x**)      | `698.37 ns` (✅ **1.00x slower**) | `93.76 ns` (🚀 **7.44x faster**)     | `305.20 ns` (🚀 **2.28x faster**)    | `942.55 ns` (❌ *1.35x slower*)      | `1.91 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `11.04 s` (✅ **1.00x**)        | `11.05 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.95 us` (✅ **1.00x**) | `251.33 us` (❌ *3.87x slower*)   | `5.98 ms` (❌ *92.04x slower*)     |
| **`legendre_for_qr`**    | `29.63 us` (✅ **1.00x**) | `251.06 us` (❌ *8.47x slower*)   | `257.54 us` (❌ *8.69x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `109.51 ns` (✅ **1.00x**)      | `214.82 ns` (❌ *1.96x slower*)    |
| **`from_big-endian_bits`**    | `109.51 ns` (✅ **1.00x**)      | `215.23 ns` (❌ *1.97x slower*)    |
| **`comparison`**              | `4.22 ns` (✅ **1.00x**)        | `4.20 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)        | `4.73 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.25 ns` (✅ **1.00x**) | `280.14 ns` (❌ *3.53x slower*)    |
| **`into_bigint`** | `41.53 ns` (✅ **1.00x**) | `142.04 ns` (❌ *3.42x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

