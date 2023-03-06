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
    - [pairing_for_bw6_761](#pairing_for_bw6_761)

## Benchmark Results

### sample_bw6_761

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `1.70 ms` (✅ **1.00x**)          | `1.69 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.15 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x slower**) | `90.00 ns` (🚀 **46.14x faster**) | `182.67 ns` (🚀 **22.73x faster**) | `30.27 ns` (🚀 **137.19x faster**) | `19.46 ns` (🚀 **213.36x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.20 us` (✅ **1.00x**) | `4.20 us` (✅ **1.00x slower**) | `85.11 ns` (🚀 **49.39x faster**) | `168.96 ns` (🚀 **24.88x faster**) | `29.13 ns` (🚀 **144.27x faster**) | `14.76 ns` (🚀 **284.75x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.96 us` (✅ **1.00x**) | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**) | `1.94 us` (✅ **1.00x slower**) | `68.29 ns` (🚀 **28.33x faster**) | `139.63 ns` (🚀 **13.86x faster**) | `23.48 ns` (🚀 **82.42x faster**)  | `11.19 ns` (🚀 **172.97x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.49 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.65 ns` (❌ *4.27x slower*)    | `123.85 ns` (❌ *7.39x slower*)    | `24.00 ns` (❌ *1.43x slower*)     | `16.76 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.16 us` (❌ *31.21x slower*)    | `6.92 us` (❌ *99.81x slower*)     | `269.56 ns` (❌ *3.89x slower*)    | `69.37 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.41x slower*)    | `4.88 us` (❌ *82.60x slower*)     | `217.00 ns` (❌ *3.67x slower*)    | `59.07 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.01 us` (❌ *3.80x slower*)    | `58.80 us` (❌ *4.39x slower*)     | `47.49 us` (❌ *3.54x slower*)     | `13.41 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.11x slower*)    | `14.08 us` (❌ *132.92x slower*)   | `401.12 ns` (❌ *3.79x slower*)    | `105.90 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.41 us` (❌ *27.78x slower*)    | `13.97 us` (❌ *88.07x slower*)    | `568.73 ns` (❌ *3.59x slower*)    | `158.61 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.63 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.63 ns` (✅ **1.00x**) | `20.99 ns` (❌ *1.98x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.68 ns` (✅ **1.00x**) | `468.59 ns` (✅ **1.00x faster**) | `50.30 ns` (🚀 **9.32x faster**)     | `156.34 ns` (🚀 **3.00x faster**)    | `466.17 ns` (✅ **1.01x faster**)    | `983.95 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `629.63 ns` (✅ **1.00x**) | `629.44 ns` (✅ **1.00x faster**) | `50.17 ns` (🚀 **12.55x faster**)    | `156.30 ns` (🚀 **4.03x faster**)    | `466.07 ns` (✅ **1.35x faster**)    | `983.77 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x slower**)   | `93.01 ns` (🚀 **14663.24x faster**) | `306.09 ns` (🚀 **4455.52x faster**) | `941.86 ns` (🚀 **1447.99x faster**) | `1.90 us` (🚀 **716.71x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.01 us` (✅ **1.00x**) | `252.00 us` (✅ **1.00x faster**) | `93.00 ns` (🚀 **2709.73x faster**)  | `306.10 ns` (🚀 **823.28x faster**)  | `941.78 ns` (🚀 **267.59x faster**)  | `1.90 us` (🚀 **132.42x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x slower**)   | `93.03 ns` (🚀 **11959.73x faster**) | `307.08 ns` (🚀 **3623.16x faster**) | `942.91 ns` (🚀 **1179.97x faster**) | `1.91 us` (🚀 **582.60x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `692.62 ns` (✅ **1.00x**) | `695.38 ns` (✅ **1.00x slower**) | `93.05 ns` (🚀 **7.44x faster**)     | `305.76 ns` (🚀 **2.27x faster**)    | `942.83 ns` (❌ *1.36x slower*)      | `1.90 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.02 s` (✅ **1.00x**) | `11.08 s` (✅ **1.01x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `65.01 us` (✅ **1.00x**) | `250.70 us` (❌ *3.86x slower*)   | `5.99 ms` (❌ *92.07x slower*)     |
| **`legendre_for_qr`**    | `29.45 us` (✅ **1.00x**) | `251.06 us` (❌ *8.52x slower*)   | `257.13 us` (❌ *8.73x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.57 ns` (✅ **1.00x**) | `211.10 ns` (❌ *1.96x slower*)    |
| **`from_big-endian_bits`**    | `107.59 ns` (✅ **1.00x**) | `210.85 ns` (❌ *1.96x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)   | `4.19 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)   | `5.03 ns` (✅ **1.08x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.26 ns` (✅ **1.00x**) | `279.08 ns` (❌ *3.52x slower*)    |
| **`into_bigint`** | `41.52 ns` (✅ **1.00x**) | `142.00 ns` (❌ *3.42x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`           |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------ |
|        | `16.60 ns` (✅ **1.00x**)              | `855.05 us` (❌ *51506.76x slower*)    | `3.05 ms` (❌ *183474.37x slower*)   | `3.68 ms` (❌ *221417.78x slower*)           | `7.56 ms` (❌ *455337.25x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

