# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381](#sample_bls12_381)
    - [arithmetic_for_bls12_381](#arithmetic_for_bls12_381)
    - [serialization_for_bls12_381](#serialization_for_bls12_381)
    - [msm_for_bls12_381](#msm_for_bls12_381)
    - [squareroot_for_bls12_381](#squareroot_for_bls12_381)
    - [bitwise_operations_for_bls12_381](#bitwise_operations_for_bls12_381)
    - [conversions_for_bls12_381](#conversions_for_bls12_381)

## Benchmark Results

### sample_bls12_381

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `202.36 us` (✅ **1.00x**)        | `1.79 ms` (❌ *8.87x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `3.97 us` (❌ *3.27x slower*)     | `23.50 ns` (🚀 **51.74x faster**) | `180.94 ns` (🚀 **6.72x faster**)  | `12.67 ns` (🚀 **95.96x faster**) | `8.68 ns` (🚀 **140.09x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.02 us` (❌ *3.18x slower*)     | `23.59 ns` (🚀 **53.51x faster**) | `160.03 ns` (🚀 **7.89x faster**)  | `12.85 ns` (🚀 **98.24x faster**) | `8.77 ns` (🚀 **143.91x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `886.08 ns` (✅ **1.00x**) | `2.83 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `910.86 ns` (✅ **1.00x**) | `2.88 us` (❌ *3.17x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `578.68 ns` (✅ **1.00x**) | `1.80 us` (❌ *3.11x slower*)     | `12.54 ns` (🚀 **46.13x faster**) | `74.19 ns` (🚀 **7.80x faster**)   | `7.23 ns` (🚀 **80.07x faster**)  | `5.90 ns` (🚀 **98.13x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `320.45 us` (✅ **1.00x**) | `961.88 us` (❌ *3.00x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.70 ns` (❌ *3.70x slower*)    | `94.09 ns` (❌ *15.35x slower*)    | `18.18 ns` (❌ *2.97x slower*)    | `6.13 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `238.91 ns` (❌ *5.20x slower*)   | `6.15 us` (❌ *134.01x slower*)    | `76.23 ns` (❌ *1.66x slower*)    | `45.93 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `176.26 ns` (❌ *4.69x slower*)   | `4.33 us` (❌ *115.22x slower*)    | `65.45 ns` (❌ *1.74x slower*)    | `37.58 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.21 us` (❌ *2.13x slower*)    | `25.45 us` (❌ *3.57x slower*)     | `14.92 us` (❌ *2.09x slower*)    | `7.13 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `517.78 ns` (❌ *6.19x slower*)   | `12.61 us` (❌ *150.80x slower*)   | `115.34 ns` (❌ *1.38x slower*)   | `83.64 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.21 ns` (❌ *5.83x slower*)   | `12.54 us` (❌ *143.49x slower*)   | `163.33 ns` (❌ *1.87x slower*)   | `87.36 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.65 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**) | `10.42 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**) | `4.50 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.68 ns` (✅ **1.00x**) | `202.80 ns` (❌ *1.35x slower*)   | `32.10 ns` (🚀 **4.69x faster**)    | `55.22 ns` (🚀 **2.73x faster**)    | `109.31 ns` (✅ **1.38x faster**)   | `703.71 ns` (❌ *4.67x slower*)    |
| **`serialize_uncompressed`**             | `192.11 ns` (✅ **1.00x**) | `283.39 ns` (❌ *1.48x slower*)   | `31.96 ns` (🚀 **6.01x faster**)    | `55.31 ns` (🚀 **3.47x faster**)    | `109.35 ns` (✅ **1.76x faster**)   | `700.14 ns` (❌ *3.64x slower*)    |
| **`deserialize_compressed`**             | `131.70 us` (✅ **1.00x**) | `264.12 us` (❌ *2.01x slower*)   | `52.15 ns` (🚀 **2525.29x faster**) | `94.82 ns` (🚀 **1388.97x faster**) | `224.57 ns` (🚀 **586.45x faster**) | `1.34 us` (🚀 **98.15x faster**)   |
| **`deserialize_compressed_unchecked`**   | `38.77 us` (✅ **1.00x**)  | `132.72 us` (❌ *3.42x slower*)   | `52.17 ns` (🚀 **743.19x faster**)  | `95.33 ns` (🚀 **406.69x faster**)  | `224.62 ns` (🚀 **172.60x faster**) | `1.34 us` (🚀 **28.89x faster**)   |
| **`deserialize_uncompressed`**           | `93.09 us` (✅ **1.00x**)  | `131.23 us` (❌ *1.41x slower*)   | `51.92 ns` (🚀 **1792.82x faster**) | `94.66 ns` (🚀 **983.34x faster**)  | `224.19 ns` (🚀 **415.23x faster**) | `1.34 us` (🚀 **69.45x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `196.52 ns` (✅ **1.00x**) | `399.91 ns` (❌ *2.03x slower*)   | `51.90 ns` (🚀 **3.79x faster**)    | `94.72 ns` (🚀 **2.07x faster**)    | `224.20 ns` (❌ *1.14x slower*)     | `1.34 us` (❌ *6.83x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `6.99 s` (❌ *3.01x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.39 us` (✅ **1.00x**) | `38.31 us` (❌ *1.51x slower*)   | `131.70 us` (❌ *5.19x slower*)    |
| **`legendre_for_qr`**    | `14.36 us` (✅ **1.00x**) | `38.49 us` (❌ *2.68x slower*)   | `38.61 us` (❌ *2.69x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.86 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.03x slower**)   |
| **`from_little-endian_bits`** | `48.83 ns` (✅ **1.00x**) | `89.39 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `48.83 ns` (✅ **1.00x**) | `89.48 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.06 ns` (✅ **1.00x**) | `76.44 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.32 ns` (✅ **1.00x**) | `47.91 ns` (❌ *2.15x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

