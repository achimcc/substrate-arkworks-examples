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
|        | `202.40 us` (✅ **1.00x**)        | `1.80 ms` (❌ *8.87x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.21 us` (✅ **1.00x**)   | `3.96 us` (❌ *3.26x slower*)     | `23.18 ns` (🚀 **52.40x faster**) | `180.47 ns` (🚀 **6.73x faster**)  | `12.67 ns` (🚀 **95.88x faster**) | `8.68 ns` (🚀 **139.93x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.02 us` (❌ *3.18x slower*)     | `23.33 ns` (🚀 **54.12x faster**) | `158.74 ns` (🚀 **7.95x faster**)  | `12.84 ns` (🚀 **98.34x faster**) | `8.78 ns` (🚀 **143.81x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `882.45 ns` (✅ **1.00x**) | `2.83 us` (❌ *3.21x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `909.69 ns` (✅ **1.00x**) | `2.89 us` (❌ *3.18x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `579.60 ns` (✅ **1.00x**) | `1.80 us` (❌ *3.10x slower*)     | `12.49 ns` (🚀 **46.39x faster**) | `67.25 ns` (🚀 **8.62x faster**)   | `7.23 ns` (🚀 **80.14x faster**)  | `5.89 ns` (🚀 **98.33x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `320.52 us` (✅ **1.00x**) | `961.64 us` (❌ *3.00x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `23.09 ns` (❌ *3.76x slower*)    | `95.97 ns` (❌ *15.63x slower*)    | `18.18 ns` (❌ *2.96x slower*)    | `6.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `238.71 ns` (❌ *5.19x slower*)   | `6.14 us` (❌ *133.62x slower*)    | `76.11 ns` (❌ *1.66x slower*)    | `45.95 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.87 ns` (❌ *4.68x slower*)   | `4.34 us` (❌ *115.49x slower*)    | `65.41 ns` (❌ *1.74x slower*)    | `37.57 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.20 us` (❌ *2.13x slower*)    | `25.44 us` (❌ *3.57x slower*)     | `14.89 us` (❌ *2.09x slower*)    | `7.13 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.25 ns` (❌ *6.21x slower*)   | `12.59 us` (❌ *150.54x slower*)   | `115.34 ns` (❌ *1.38x slower*)   | `83.61 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.51 ns` (❌ *5.83x slower*)   | `12.53 us` (❌ *143.26x slower*)   | `163.08 ns` (❌ *1.87x slower*)   | `87.43 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.65 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.40 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.30 ns` (✅ **1.00x**) | `202.93 ns` (❌ *1.35x slower*)   | `31.89 ns` (🚀 **4.71x faster**)    | `55.19 ns` (🚀 **2.72x faster**)    | `109.44 ns` (✅ **1.37x faster**)   | `708.24 ns` (❌ *4.71x slower*)    |
| **`serialize_uncompressed`**             | `191.98 ns` (✅ **1.00x**) | `282.28 ns` (❌ *1.47x slower*)   | `32.36 ns` (🚀 **5.93x faster**)    | `55.26 ns` (🚀 **3.47x faster**)    | `109.32 ns` (✅ **1.76x faster**)   | `701.01 ns` (❌ *3.65x slower*)    |
| **`deserialize_compressed`**             | `131.78 us` (✅ **1.00x**) | `264.12 us` (❌ *2.00x slower*)   | `52.24 ns` (🚀 **2522.56x faster**) | `94.13 ns` (🚀 **1399.98x faster**) | `218.08 ns` (🚀 **604.29x faster**) | `1.36 us` (🚀 **96.70x faster**)   |
| **`deserialize_compressed_unchecked`**   | `38.77 us` (✅ **1.00x**)  | `132.63 us` (❌ *3.42x slower*)   | `52.24 ns` (🚀 **742.16x faster**)  | `94.04 ns` (🚀 **412.30x faster**)  | `217.98 ns` (🚀 **177.87x faster**) | `1.36 us` (🚀 **28.45x faster**)   |
| **`deserialize_uncompressed`**           | `92.77 us` (✅ **1.00x**)  | `131.30 us` (❌ *1.42x slower*)   | `52.17 ns` (🚀 **1778.28x faster**) | `94.02 ns` (🚀 **986.64x faster**)  | `217.27 ns` (🚀 **426.96x faster**) | `1.36 us` (🚀 **68.07x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `196.77 ns` (✅ **1.00x**) | `397.63 ns` (❌ *2.02x slower*)   | `52.13 ns` (🚀 **3.77x faster**)    | `93.93 ns` (🚀 **2.09x faster**)    | `217.22 ns` (✅ **1.10x slower**)   | `1.36 us` (❌ *6.92x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `6.99 s` (❌ *3.01x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.37 us` (✅ **1.00x**) | `38.28 us` (❌ *1.51x slower*)   | `131.66 us` (❌ *5.19x slower*)    |
| **`legendre_for_qr`**    | `14.35 us` (✅ **1.00x**) | `38.47 us` (❌ *2.68x slower*)   | `38.61 us` (❌ *2.69x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.86 ns` (✅ **1.00x**) | `89.79 ns` (❌ *1.84x slower*)    |
| **`from_big-endian_bits`**    | `48.87 ns` (✅ **1.00x**) | `89.33 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.07 ns` (✅ **1.00x**) | `76.39 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.31 ns` (✅ **1.00x**) | `47.90 ns` (❌ *2.15x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

