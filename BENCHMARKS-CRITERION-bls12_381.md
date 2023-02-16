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
|        | `202.59 us` (✅ **1.00x**)        | `1.79 ms` (❌ *8.86x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `3.97 us` (❌ *3.25x slower*)     | `23.20 ns` (🚀 **52.53x faster**) | `180.97 ns` (🚀 **6.73x faster**)  | `12.66 ns` (🚀 **96.26x faster**) | `8.68 ns` (🚀 **140.47x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.01 us` (❌ *3.18x slower*)     | `23.41 ns` (🚀 **53.91x faster**) | `158.26 ns` (🚀 **7.97x faster**)  | `12.83 ns` (🚀 **98.33x faster**) | `8.77 ns` (🚀 **143.92x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `885.02 ns` (✅ **1.00x**) | `2.84 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `911.35 ns` (✅ **1.00x**) | `2.89 us` (❌ *3.17x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `580.00 ns` (✅ **1.00x**) | `1.80 us` (❌ *3.11x slower*)     | `12.46 ns` (🚀 **46.54x faster**) | `74.24 ns` (🚀 **7.81x faster**)   | `7.24 ns` (🚀 **80.08x faster**)  | `5.91 ns` (🚀 **98.15x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `320.36 us` (✅ **1.00x**) | `962.29 us` (❌ *3.00x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.68 ns` (❌ *3.70x slower*)    | `95.62 ns` (❌ *15.60x slower*)    | `18.10 ns` (❌ *2.95x slower*)    | `6.13 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `239.44 ns` (❌ *5.21x slower*)   | `6.15 us` (❌ *133.96x slower*)    | `76.16 ns` (❌ *1.66x slower*)    | `45.94 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.55 ns` (❌ *4.68x slower*)   | `4.34 us` (❌ *115.47x slower*)    | `65.42 ns` (❌ *1.74x slower*)    | `37.55 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.20 us` (❌ *2.13x slower*)    | `25.41 us` (❌ *3.56x slower*)     | `14.89 us` (❌ *2.09x slower*)    | `7.13 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `518.65 ns` (❌ *6.20x slower*)   | `12.61 us` (❌ *150.75x slower*)   | `115.48 ns` (❌ *1.38x slower*)   | `83.64 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.05 ns` (❌ *5.83x slower*)   | `12.62 us` (❌ *144.40x slower*)   | `163.27 ns` (❌ *1.87x slower*)   | `87.39 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.65 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.76 ns` (✅ **1.00x**) | `10.39 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.29 ns` (✅ **1.00x**) | `202.87 ns` (❌ *1.35x slower*)   | `31.92 ns` (🚀 **4.71x faster**)    | `55.15 ns` (🚀 **2.73x faster**)    | `109.35 ns` (✅ **1.37x faster**)   | `704.75 ns` (❌ *4.69x slower*)    |
| **`serialize_uncompressed`**             | `192.25 ns` (✅ **1.00x**) | `282.15 ns` (❌ *1.47x slower*)   | `31.99 ns` (🚀 **6.01x faster**)    | `55.37 ns` (🚀 **3.47x faster**)    | `109.32 ns` (✅ **1.76x faster**)   | `699.13 ns` (❌ *3.64x slower*)    |
| **`deserialize_compressed`**             | `131.72 us` (✅ **1.00x**) | `264.37 us` (❌ *2.01x slower*)   | `51.95 ns` (🚀 **2535.59x faster**) | `93.63 ns` (🚀 **1406.78x faster**) | `217.22 ns` (🚀 **606.38x faster**) | `1.34 us` (🚀 **98.38x faster**)   |
| **`deserialize_compressed_unchecked`**   | `38.79 us` (✅ **1.00x**)  | `132.68 us` (❌ *3.42x slower*)   | `52.39 ns` (🚀 **740.26x faster**)  | `93.67 ns` (🚀 **414.04x faster**)  | `217.18 ns` (🚀 **178.58x faster**) | `1.34 us` (🚀 **28.98x faster**)   |
| **`deserialize_uncompressed`**           | `92.81 us` (✅ **1.00x**)  | `131.55 us` (❌ *1.42x slower*)   | `52.09 ns` (🚀 **1781.59x faster**) | `93.68 ns` (🚀 **990.76x faster**)  | `217.13 ns` (🚀 **427.43x faster**) | `1.34 us` (🚀 **69.34x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `196.71 ns` (✅ **1.00x**) | `398.27 ns` (❌ *2.02x slower*)   | `51.91 ns` (🚀 **3.79x faster**)    | `93.73 ns` (🚀 **2.10x faster**)    | `217.40 ns` (✅ **1.11x slower**)   | `1.34 us` (❌ *6.80x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `7.02 s` (❌ *3.02x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.37 us` (✅ **1.00x**) | `38.30 us` (❌ *1.51x slower*)   | `131.65 us` (❌ *5.19x slower*)    |
| **`legendre_for_qr`**    | `14.34 us` (✅ **1.00x**) | `38.48 us` (❌ *2.68x slower*)   | `38.62 us` (❌ *2.69x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.86 ns` (✅ **1.00x**) | `88.90 ns` (❌ *1.82x slower*)    |
| **`from_big-endian_bits`**    | `48.83 ns` (✅ **1.00x**) | `89.11 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.06 ns` (✅ **1.00x**) | `76.60 ns` (❌ *1.87x slower*)    |
| **`into_bigint`** | `22.36 ns` (✅ **1.00x**) | `47.90 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

