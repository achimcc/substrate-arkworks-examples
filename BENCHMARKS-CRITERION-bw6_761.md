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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.15 us` (✅ **1.00x**) | `4.15 us` (✅ **1.00x slower**) | `89.81 ns` (🚀 **46.23x faster**) | `180.32 ns` (🚀 **23.03x faster**) | `30.27 ns` (🚀 **137.16x faster**) | `19.45 ns` (🚀 **213.43x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.21 us` (✅ **1.00x**) | `4.21 us` (✅ **1.00x slower**) | `84.43 ns` (🚀 **49.89x faster**) | `166.68 ns` (🚀 **25.27x faster**) | `29.39 ns` (🚀 **143.33x faster**) | `15.11 ns` (🚀 **278.82x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.92 us` (✅ **1.00x**) | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.96 us` (✅ **1.00x**) | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**) | `1.93 us` (✅ **1.00x slower**) | `67.94 ns` (🚀 **28.47x faster**) | `140.16 ns` (🚀 **13.80x faster**) | `21.26 ns` (🚀 **91.01x faster**)  | `11.17 ns` (🚀 **173.13x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.49 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `69.46 ns` (❌ *4.01x slower*)    | `123.70 ns` (❌ *7.15x slower*)    | `24.03 ns` (❌ *1.39x slower*)     | `17.31 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.16 us` (❌ *31.20x slower*)    | `6.92 us` (❌ *99.76x slower*)     | `270.63 ns` (❌ *3.90x slower*)    | `69.37 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.31x slower*)    | `4.89 us` (❌ *82.55x slower*)     | `218.05 ns` (❌ *3.68x slower*)    | `59.29 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.94 us` (❌ *3.82x slower*)    | `58.73 us` (❌ *4.40x slower*)     | `47.43 us` (❌ *3.56x slower*)     | `13.34 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.10x slower*)    | `14.07 us` (❌ *132.86x slower*)   | `401.01 ns` (❌ *3.79x slower*)    | `105.92 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.41 us` (❌ *27.77x slower*)    | `13.98 us` (❌ *88.08x slower*)    | `569.66 ns` (❌ *3.59x slower*)    | `158.66 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `18.38 ns` (❌ *2.35x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.74 ns` (✅ **1.00x**) | `21.63 ns` (❌ *2.01x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.72 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `468.70 ns` (✅ **1.00x**) | `468.44 ns` (✅ **1.00x faster**) | `50.30 ns` (🚀 **9.32x faster**)     | `157.02 ns` (🚀 **2.98x faster**)    | `466.01 ns` (✅ **1.01x faster**)    | `984.20 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `629.31 ns` (✅ **1.00x**) | `629.39 ns` (✅ **1.00x slower**) | `50.17 ns` (🚀 **12.54x faster**)    | `157.12 ns` (🚀 **4.01x faster**)    | `466.31 ns` (✅ **1.35x faster**)    | `984.08 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x slower**)   | `93.72 ns` (🚀 **14555.10x faster**) | `306.09 ns` (🚀 **4456.56x faster**) | `942.66 ns` (🚀 **1447.08x faster**) | `1.90 us` (🚀 **717.74x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.15 us` (✅ **1.00x**) | `252.12 us` (✅ **1.00x faster**) | `93.73 ns` (🚀 **2690.30x faster**)  | `306.06 ns` (🚀 **823.85x faster**)  | `942.68 ns` (🚀 **267.48x faster**)  | `1.90 us` (🚀 **132.71x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x slower**)   | `93.78 ns` (🚀 **11865.85x faster**) | `306.17 ns` (🚀 **3634.55x faster**) | `942.85 ns` (🚀 **1180.24x faster**) | `1.90 us` (🚀 **585.48x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `698.02 ns` (✅ **1.00x**) | `697.06 ns` (✅ **1.00x faster**) | `93.80 ns` (🚀 **7.44x faster**)     | `306.12 ns` (🚀 **2.28x faster**)    | `943.11 ns` (❌ *1.35x slower*)      | `1.90 us` (❌ *2.72x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `10.97 s` (✅ **1.00x**) | `10.98 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.91 us` (✅ **1.00x**) | `250.87 us` (❌ *3.86x slower*)   | `5.99 ms` (❌ *92.23x slower*)     |
| **`legendre_for_qr`**    | `29.53 us` (✅ **1.00x**) | `251.03 us` (❌ *8.50x slower*)   | `257.05 us` (❌ *8.70x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.68 ns` (✅ **1.00x**) | `211.51 ns` (❌ *1.96x slower*)    |
| **`from_big-endian_bits`**    | `107.50 ns` (✅ **1.00x**) | `211.44 ns` (❌ *1.97x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)   | `4.20 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)   | `5.07 ns` (✅ **1.09x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.35 ns` (✅ **1.00x**) | `278.36 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.56 ns` (✅ **1.00x**) | `143.65 ns` (❌ *3.46x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`           |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------ |
|        | `16.61 ns` (✅ **1.00x**)              | `854.40 us` (❌ *51437.06x slower*)    | `3.04 ms` (❌ *183140.24x slower*)   | `3.68 ms` (❌ *221305.23x slower*)           | `7.56 ms` (❌ *455379.05x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

