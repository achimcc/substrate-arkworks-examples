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
    - [pairing_for_bw6_761optimized](#pairing_for_bw6_761optimized)

## Benchmark Results

### sample_bw6_761_optimized

|        | `g1projectivebw6_761_elements`          | `g2projectivebw6_761_elements`           |
|:-------|:----------------------------------------|:---------------------------------------- |
|        | `1.75 ms` (✅ **1.00x**)                 | `1.74 ms` (✅ **1.01x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.15 us` (✅ **1.00x slower**) | `90.52 ns` (🚀 **45.76x faster**) | `182.95 ns` (🚀 **22.64x faster**) | `30.66 ns` (🚀 **135.07x faster**) | `19.47 ns` (🚀 **212.67x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.19 us` (✅ **1.00x**)        | `4.19 us` (✅ **1.00x slower**) | `84.84 ns` (🚀 **49.38x faster**) | `170.94 ns` (🚀 **24.51x faster**) | `29.27 ns` (🚀 **143.13x faster**) | `15.32 ns` (🚀 **273.49x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.92 us` (✅ **1.00x**)        | `2.93 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.95 us` (✅ **1.00x**)        | `2.95 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.93 us` (✅ **1.00x**)        | `1.93 us` (✅ **1.00x slower**) | `68.94 ns` (🚀 **27.99x faster**) | `140.19 ns` (🚀 **13.76x faster**) | `21.00 ns` (🚀 **91.87x faster**)  | `11.09 ns` (🚀 **173.92x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.55 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `68.87 ns` (❌ *4.11x slower*)    | `122.11 ns` (❌ *7.28x slower*)    | `23.98 ns` (❌ *1.43x slower*)     | `16.77 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.16 us` (❌ *31.19x slower*)    | `6.91 us` (❌ *99.64x slower*)     | `271.26 ns` (❌ *3.91x slower*)    | `69.39 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.56 us` (❌ *26.30x slower*)    | `4.85 us` (❌ *81.91x slower*)     | `218.29 ns` (❌ *3.69x slower*)    | `59.20 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.00 us` (❌ *3.79x slower*)    | `58.71 us` (❌ *4.37x slower*)     | `47.42 us` (❌ *3.53x slower*)     | `13.45 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.47 us` (❌ *42.17x slower*)    | `14.06 us` (❌ *132.71x slower*)   | `401.40 ns` (❌ *3.79x slower*)    | `105.94 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.40 us` (❌ *27.98x slower*)    | `13.96 us` (❌ *88.72x slower*)    | `570.04 ns` (❌ *3.62x slower*)    | `157.39 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)        | `15.73 ns` (❌ *2.01x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.74 ns` (✅ **1.00x**)       | `21.27 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `471.65 ns` (✅ **1.00x**)      | `469.28 ns` (✅ **1.01x faster**) | `50.30 ns` (🚀 **9.38x faster**)     | `157.55 ns` (🚀 **2.99x faster**)    | `466.56 ns` (✅ **1.01x faster**)    | `990.72 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `628.71 ns` (✅ **1.00x**)      | `635.21 ns` (✅ **1.01x slower**) | `50.07 ns` (🚀 **12.56x faster**)    | `157.33 ns` (🚀 **4.00x faster**)    | `466.57 ns` (✅ **1.35x faster**)    | `990.33 ns` (❌ *1.58x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.37 ms` (✅ **1.00x faster**)   | `93.63 ns` (🚀 **14594.62x faster**) | `307.96 ns` (🚀 **4436.98x faster**) | `941.01 ns` (🚀 **1452.09x faster**) | `1.89 us` (🚀 **724.09x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.12 us` (✅ **1.00x**)      | `252.29 us` (✅ **1.00x slower**) | `93.64 ns` (🚀 **2692.35x faster**)  | `307.61 ns` (🚀 **819.61x faster**)  | `945.31 ns` (🚀 **266.71x faster**)  | `1.89 us` (🚀 **133.62x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)        | `1.11 ms` (✅ **1.00x faster**)   | `93.62 ns` (🚀 **11906.07x faster**) | `306.28 ns` (🚀 **3639.33x faster**) | `943.85 ns` (🚀 **1180.94x faster**) | `1.89 us` (🚀 **589.87x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `673.87 ns` (✅ **1.00x**)      | `673.38 ns` (✅ **1.00x faster**) | `93.62 ns` (🚀 **7.20x faster**)     | `306.26 ns` (🚀 **2.20x faster**)    | `943.93 ns` (❌ *1.40x slower*)      | `1.90 us` (❌ *2.82x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `11.26 s` (✅ **1.00x**)        | `11.30 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.88 us` (✅ **1.00x**) | `250.82 us` (❌ *3.87x slower*)   | `5.99 ms` (❌ *92.26x slower*)     |
| **`legendre_for_qr`**    | `29.14 us` (✅ **1.00x**) | `251.38 us` (❌ *8.63x slower*)   | `256.97 us` (❌ *8.82x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `109.57 ns` (✅ **1.00x**)      | `211.58 ns` (❌ *1.93x slower*)    |
| **`from_big-endian_bits`**    | `109.50 ns` (✅ **1.00x**)      | `210.60 ns` (❌ *1.92x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)        | `4.20 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.43 ns` (✅ **1.00x**) | `277.66 ns` (❌ *3.50x slower*)    |
| **`into_bigint`** | `41.50 ns` (✅ **1.00x**) | `144.47 ns` (❌ *3.48x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `16.64 ns` (✅ **1.00x**)                       | `11.23 ns` (✅ **1.48x faster**)                | `3.90 ms` (❌ *234305.86x slower*)           | `3.68 ms` (❌ *221314.27x slower*)                    | `7.57 ms` (❌ *455071.81x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

