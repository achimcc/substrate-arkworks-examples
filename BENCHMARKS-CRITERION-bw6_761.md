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
|        | `2.03 ms` (✅ **1.00x**)          | `2.02 ms` (✅ **1.00x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                             | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.98 us` (✅ **1.00x**) | `4.99 us` (✅ **1.00x slower**) | `110.72 ns` (🚀 **45.01x faster**) | `216.44 ns` (🚀 **23.03x faster**) | `36.06 ns` (🚀 **138.22x faster**) | `22.91 ns` (🚀 **217.57x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `5.04 us` (✅ **1.00x**) | `5.04 us` (✅ **1.00x faster**) | `100.04 ns` (🚀 **50.43x faster**) | `190.36 ns` (🚀 **26.50x faster**) | `33.65 ns` (🚀 **149.93x faster**) | `18.67 ns` (🚀 **270.25x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.51 us` (✅ **1.00x**) | `3.51 us` (✅ **1.00x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.55 us` (✅ **1.00x**) | `3.56 us` (✅ **1.00x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.32 us` (✅ **1.00x**) | `2.32 us` (✅ **1.00x faster**) | `84.41 ns` (🚀 **27.50x faster**)  | `176.38 ns` (🚀 **13.16x faster**) | `26.32 ns` (🚀 **88.18x faster**)  | `13.42 ns` (🚀 **172.98x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.80 ms` (✅ **1.00x**) | `1.80 ms` (✅ **1.00x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `85.85 ns` (❌ *4.18x slower*)     | `148.18 ns` (❌ *7.21x slower*)    | `29.30 ns` (❌ *1.43x slower*)     | `20.55 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.68 us` (❌ *31.71x slower*)     | `8.25 us` (❌ *97.74x slower*)     | `324.21 ns` (❌ *3.84x slower*)    | `84.43 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.87 us` (❌ *26.40x slower*)     | `5.82 us` (❌ *82.09x slower*)     | `259.09 ns` (❌ *3.65x slower*)    | `70.91 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `60.77 us` (❌ *3.78x slower*)     | `69.56 us` (❌ *4.32x slower*)     | `56.08 us` (❌ *3.48x slower*)     | `16.09 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.35 us` (❌ *42.38x slower*)     | `16.82 us` (❌ *133.19x slower*)   | `481.71 ns` (❌ *3.81x slower*)    | `126.32 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.29 us` (❌ *27.93x slower*)     | `16.70 us` (❌ *88.24x slower*)    | `681.04 ns` (❌ *3.60x slower*)    | `189.23 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `9.40 ns` (✅ **1.00x**)  | `18.82 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `12.75 ns` (✅ **1.00x**) | `25.33 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.85 ns` (✅ **1.00x**)  | `4.91 ns` (✅ **1.01x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.50 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                 | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `562.24 ns` (✅ **1.00x**) | `560.84 ns` (✅ **1.00x faster**) | `60.36 ns` (🚀 **9.32x faster**)      | `187.82 ns` (🚀 **2.99x faster**)    | `569.41 ns` (✅ **1.01x slower**)  | `1.18 us` (❌ *2.10x slower*)      |
| **`serialize_uncompressed`**             | `755.01 ns` (✅ **1.00x**) | `754.65 ns` (✅ **1.00x faster**) | `60.11 ns` (🚀 **12.56x faster**)     | `188.71 ns` (🚀 **4.00x faster**)    | `557.78 ns` (✅ **1.35x faster**)  | `1.18 us` (❌ *1.57x slower*)      |
| **`deserialize_compressed`**             | `1.64 ms` (✅ **1.00x**)   | `1.64 ms` (✅ **1.00x faster**)   | `113.17 ns` (🚀 **14460.21x faster**) | `365.31 ns` (🚀 **4479.63x faster**) | `1.13 us` (🚀 **1448.56x faster**) | `2.28 us` (🚀 **716.78x faster**)  |
| **`deserialize_compressed_unchecked`**   | `302.59 us` (✅ **1.00x**) | `302.62 us` (✅ **1.00x slower**) | `112.53 ns` (🚀 **2688.95x faster**)  | `366.83 ns` (🚀 **824.87x faster**)  | `1.13 us` (🚀 **267.95x faster**)  | `2.28 us` (🚀 **132.57x faster**)  |
| **`deserialize_uncompressed`**           | `1.34 ms` (✅ **1.00x**)   | `1.35 ms` (✅ **1.01x slower**)   | `112.52 ns` (🚀 **11868.24x faster**) | `367.08 ns` (🚀 **3637.96x faster**) | `1.13 us` (🚀 **1177.58x faster**) | `2.28 us` (🚀 **584.61x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `834.71 ns` (✅ **1.00x**) | `837.25 ns` (✅ **1.00x slower**) | `112.46 ns` (🚀 **7.42x faster**)     | `365.47 ns` (🚀 **2.28x faster**)    | `1.14 us` (❌ *1.37x slower*)      | `2.28 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `13.26 s` (✅ **1.00x**) | `13.28 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `77.68 us` (✅ **1.00x**) | `301.19 us` (❌ *3.88x slower*)   | `7.20 ms` (❌ *92.72x slower*)     |
| **`legendre_for_qr`**    | `35.10 us` (✅ **1.00x**) | `301.29 us` (❌ *8.58x slower*)   | `308.52 us` (❌ *8.79x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)   | `5.10 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `129.73 ns` (✅ **1.00x**) | `261.64 ns` (❌ *2.02x slower*)    |
| **`from_big-endian_bits`**    | `129.53 ns` (✅ **1.00x**) | `261.84 ns` (❌ *2.02x slower*)    |
| **`comparison`**              | `5.04 ns` (✅ **1.00x**)   | `5.04 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `5.65 ns` (✅ **1.00x**)   | `6.14 ns` (✅ **1.09x slower**)    |
| **`is_zero`**                 | `4.81 ns` (✅ **1.00x**)   | `4.94 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `94.24 ns` (✅ **1.00x**) | `334.20 ns` (❌ *3.55x slower*)    |
| **`into_bigint`** | `49.81 ns` (✅ **1.00x**) | `172.77 ns` (❌ *3.47x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

