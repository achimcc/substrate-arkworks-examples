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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.15 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x faster**) | `89.80 ns` (🚀 **46.21x faster**) | `178.96 ns` (🚀 **23.19x faster**) | `30.77 ns` (🚀 **134.84x faster**) | `19.09 ns` (🚀 **217.41x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.20 us` (✅ **1.00x**) | `4.20 us` (✅ **1.00x faster**) | `84.35 ns` (🚀 **49.79x faster**) | `166.10 ns` (🚀 **25.28x faster**) | `28.74 ns` (🚀 **146.13x faster**) | `18.19 ns` (🚀 **230.90x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.95 us` (✅ **1.00x**) | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**) | `1.93 us` (✅ **1.00x faster**) | `69.06 ns` (🚀 **27.96x faster**) | `139.54 ns` (🚀 **13.84x faster**) | `24.19 ns` (🚀 **79.82x faster**)  | `11.17 ns` (🚀 **172.80x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.49 ms` (✅ **1.01x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `67.99 ns` (❌ *4.06x slower*)    | `122.18 ns` (❌ *7.29x slower*)    | `23.96 ns` (❌ *1.43x slower*)     | `16.75 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.17 us` (❌ *31.25x slower*)    | `6.91 us` (❌ *99.56x slower*)     | `271.21 ns` (❌ *3.91x slower*)    | `69.36 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.29x slower*)    | `4.87 us` (❌ *82.18x slower*)     | `216.80 ns` (❌ *3.66x slower*)    | `59.31 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `51.27 us` (❌ *3.92x slower*)    | `59.04 us` (❌ *4.52x slower*)     | `47.74 us` (❌ *3.65x slower*)     | `13.07 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.12x slower*)    | `14.07 us` (❌ *132.93x slower*)   | `400.77 ns` (❌ *3.79x slower*)    | `105.87 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.40 us` (❌ *27.97x slower*)    | `13.98 us` (❌ *88.82x slower*)    | `568.73 ns` (❌ *3.61x slower*)    | `157.34 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.66 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.63 ns` (✅ **1.00x**) | `21.00 ns` (❌ *1.98x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.40 ns` (✅ **1.00x**) | `469.61 ns` (✅ **1.00x slower**) | `50.30 ns` (🚀 **9.31x faster**)     | `157.45 ns` (🚀 **2.97x faster**)    | `465.77 ns` (✅ **1.01x faster**)    | `990.39 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `629.63 ns` (✅ **1.00x**) | `629.12 ns` (✅ **1.00x faster**) | `50.20 ns` (🚀 **12.54x faster**)    | `157.56 ns` (🚀 **4.00x faster**)    | `465.99 ns` (✅ **1.35x faster**)    | `989.74 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x faster**)   | `93.14 ns` (🚀 **14639.80x faster**) | `306.17 ns` (🚀 **4453.62x faster**) | `941.57 ns` (🚀 **1448.17x faster**) | `1.90 us` (🚀 **716.50x faster**)  |
| **`deserialize_compressed_unchecked`**   | `251.79 us` (✅ **1.00x**) | `251.77 us` (✅ **1.00x faster**) | `93.14 ns` (🚀 **2703.46x faster**)  | `306.17 ns` (🚀 **822.39x faster**)  | `941.41 ns` (🚀 **267.46x faster**)  | `1.90 us` (🚀 **132.29x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x slower**)   | `93.08 ns` (🚀 **11930.30x faster**) | `306.13 ns` (🚀 **3627.42x faster**) | `943.82 ns` (🚀 **1176.56x faster**) | `1.90 us` (🚀 **583.26x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `671.83 ns` (✅ **1.00x**) | `671.89 ns` (✅ **1.00x slower**) | `93.07 ns` (🚀 **7.22x faster**)     | `306.08 ns` (🚀 **2.19x faster**)    | `948.08 ns` (❌ *1.41x slower*)      | `1.90 us` (❌ *2.83x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.01 s` (✅ **1.00x**) | `11.04 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.70 us` (✅ **1.00x**) | `250.49 us` (❌ *3.87x slower*)   | `5.98 ms` (❌ *92.44x slower*)     |
| **`legendre_for_qr`**    | `29.18 us` (✅ **1.00x**) | `251.48 us` (❌ *8.62x slower*)   | `256.75 us` (❌ *8.80x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.99 ns` (✅ **1.00x**) | `216.43 ns` (❌ *2.00x slower*)    |
| **`from_big-endian_bits`**    | `107.96 ns` (✅ **1.00x**) | `216.51 ns` (❌ *2.01x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)   | `4.20 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)   | `4.62 ns` (✅ **1.01x faster**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.36 ns` (✅ **1.00x**) | `278.22 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.48 ns` (✅ **1.00x**) | `143.41 ns` (❌ *3.46x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`           |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------ |
|        | `16.61 ns` (✅ **1.00x**)              | `854.33 us` (❌ *51446.31x slower*)    | `3.04 ms` (❌ *182929.93x slower*)   | `3.67 ms` (❌ *221180.73x slower*)           | `7.56 ms` (❌ *455388.81x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

