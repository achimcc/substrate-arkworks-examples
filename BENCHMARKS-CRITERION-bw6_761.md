# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761](#sample_bw6_761)
    - [arithmetic_for_bw6_761](#arithmetic_for_bw6_761)
    - [serialization_for_bw6_761](#serialization_for_bw6_761)
    - [msm_for_bw6_761](#msm_for_bw6_761)
    - [squareroot_for_bw6_761](#squareroot_for_bw6_761)
    - [bitwise_operations_for_bw6_761](#bitwise_operations_for_bw6_761)
    - [conversions_for_bw6_761](#conversions_for_bw6_761)

## Benchmark Results

### sample_bw6_761

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `1.70 ms` (✅ **1.00x**)          | `1.68 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.15 us` (✅ **1.00x**) | `4.16 us` (✅ **1.00x slower**) | `89.78 ns` (🚀 **46.23x faster**) | `180.46 ns` (🚀 **23.00x faster**) | `29.91 ns` (🚀 **138.78x faster**) | `19.04 ns` (🚀 **218.02x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.20 us` (✅ **1.00x**) | `4.20 us` (✅ **1.00x slower**) | `81.03 ns` (🚀 **51.78x faster**) | `156.81 ns` (🚀 **26.75x faster**) | `27.47 ns` (🚀 **152.70x faster**) | `15.50 ns` (🚀 **270.61x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.95 us` (✅ **1.00x**) | `2.95 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**) | `1.93 us` (✅ **1.00x slower**) | `70.38 ns` (🚀 **27.43x faster**) | `144.02 ns` (🚀 **13.40x faster**) | `22.07 ns` (🚀 **87.48x faster**)  | `11.16 ns` (🚀 **172.98x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.50 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `68.05 ns` (❌ *4.06x slower*)    | `122.60 ns` (❌ *7.32x slower*)    | `24.11 ns` (❌ *1.44x slower*)     | `16.76 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.16 us` (❌ *30.52x slower*)    | `6.87 us` (❌ *97.12x slower*)     | `271.20 ns` (❌ *3.83x slower*)    | `70.78 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.41x slower*)    | `4.83 us` (❌ *81.82x slower*)     | `216.79 ns` (❌ *3.67x slower*)    | `59.02 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.16 us` (❌ *3.74x slower*)    | `57.87 us` (❌ *4.32x slower*)     | `46.64 us` (❌ *3.48x slower*)     | `13.41 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.45 us` (❌ *42.04x slower*)    | `14.00 us` (❌ *132.29x slower*)   | `400.85 ns` (❌ *3.79x slower*)    | `105.85 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.39 us` (❌ *27.81x slower*)    | `13.94 us` (❌ *88.27x slower*)    | `568.17 ns` (❌ *3.60x slower*)    | `157.87 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.68 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**) | `21.22 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.30 ns` (✅ **1.00x**) | `467.04 ns` (✅ **1.00x faster**) | `50.25 ns` (🚀 **9.32x faster**)     | `157.28 ns` (🚀 **2.98x faster**)    | `466.24 ns` (✅ **1.00x faster**)    | `983.88 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `628.81 ns` (✅ **1.00x**) | `628.91 ns` (✅ **1.00x slower**) | `50.04 ns` (🚀 **12.57x faster**)    | `157.98 ns` (🚀 **3.98x faster**)    | `466.22 ns` (✅ **1.35x faster**)    | `983.84 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x faster**)   | `93.58 ns` (🚀 **14565.47x faster**) | `305.80 ns` (🚀 **4457.08x faster**) | `940.70 ns` (🚀 **1448.89x faster**) | `1.90 us` (🚀 **716.08x faster**)  |
| **`deserialize_compressed_unchecked`**   | `251.90 us` (✅ **1.00x**) | `251.89 us` (✅ **1.00x faster**) | `93.62 ns` (🚀 **2690.84x faster**)  | `305.99 ns` (🚀 **823.24x faster**)  | `940.56 ns` (🚀 **267.82x faster**)  | `1.90 us` (🚀 **132.36x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x slower**)   | `93.57 ns` (🚀 **11876.83x faster**) | `305.92 ns` (🚀 **3632.56x faster**) | `941.19 ns` (🚀 **1180.71x faster**) | `1.91 us` (🚀 **583.18x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `692.75 ns` (✅ **1.00x**) | `695.40 ns` (✅ **1.00x slower**) | `93.77 ns` (🚀 **7.39x faster**)     | `305.88 ns` (🚀 **2.26x faster**)    | `941.37 ns` (❌ *1.36x slower*)      | `1.90 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.03 s` (✅ **1.00x**) | `11.03 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.72 us` (✅ **1.00x**) | `250.70 us` (❌ *3.87x slower*)   | `5.98 ms` (❌ *92.43x slower*)     |
| **`legendre_for_qr`**    | `29.24 us` (✅ **1.00x**) | `250.89 us` (❌ *8.58x slower*)   | `256.80 us` (❌ *8.78x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.74 ns` (✅ **1.00x**) | `218.07 ns` (❌ *2.02x slower*)    |
| **`from_big-endian_bits`**    | `107.85 ns` (✅ **1.00x**) | `217.83 ns` (❌ *2.02x slower*)    |
| **`comparison`**              | `4.21 ns` (✅ **1.00x**)   | `4.19 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)   | `4.63 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.63 ns` (✅ **1.00x**) | `278.50 ns` (❌ *3.50x slower*)    |
| **`into_bigint`** | `41.47 ns` (✅ **1.00x**) | `141.85 ns` (❌ *3.42x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

