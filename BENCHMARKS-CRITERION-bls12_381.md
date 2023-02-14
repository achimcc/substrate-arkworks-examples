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
|        | `202.32 us` (✅ **1.00x**)        | `1.80 ms` (❌ *8.88x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `3.96 us` (❌ *3.25x slower*)     | `23.50 ns` (🚀 **51.80x faster**) | `181.11 ns` (🚀 **6.72x faster**)  | `12.66 ns` (🚀 **96.14x faster**) | `8.69 ns` (🚀 **140.16x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.02 us` (❌ *3.19x slower*)     | `23.41 ns` (🚀 **53.84x faster**) | `158.22 ns` (🚀 **7.97x faster**)  | `12.84 ns` (🚀 **98.20x faster**) | `8.78 ns` (🚀 **143.58x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `881.72 ns` (✅ **1.00x**) | `2.83 us` (❌ *3.21x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `909.81 ns` (✅ **1.00x**) | `2.89 us` (❌ *3.17x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `577.85 ns` (✅ **1.00x**) | `1.80 us` (❌ *3.12x slower*)     | `12.45 ns` (🚀 **46.40x faster**) | `71.28 ns` (🚀 **8.11x faster**)   | `7.23 ns` (🚀 **79.95x faster**)  | `5.89 ns` (🚀 **98.15x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `320.45 us` (✅ **1.00x**) | `961.56 us` (❌ *3.00x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `23.46 ns` (❌ *3.83x slower*)    | `92.97 ns` (❌ *15.16x slower*)    | `18.90 ns` (❌ *3.08x slower*)    | `6.13 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `238.73 ns` (❌ *5.20x slower*)   | `6.14 us` (❌ *133.62x slower*)    | `76.29 ns` (❌ *1.66x slower*)    | `45.92 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.88 ns` (❌ *4.68x slower*)   | `4.35 us` (❌ *115.54x slower*)    | `65.39 ns` (❌ *1.74x slower*)    | `37.62 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.20 us` (❌ *2.13x slower*)    | `25.59 us` (❌ *3.58x slower*)     | `14.90 us` (❌ *2.09x slower*)    | `7.14 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.39 ns` (❌ *6.21x slower*)   | `12.68 us` (❌ *151.62x slower*)   | `115.34 ns` (❌ *1.38x slower*)   | `83.63 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.57 ns` (❌ *5.83x slower*)   | `12.54 us` (❌ *143.43x slower*)   | `163.44 ns` (❌ *1.87x slower*)   | `87.40 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.65 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.67 ns` (✅ **1.00x**) | `10.39 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.51 ns` (✅ **1.00x**) | `4.51 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.73 ns` (✅ **1.00x**) | `202.46 ns` (❌ *1.34x slower*)   | `31.90 ns` (🚀 **4.73x faster**)    | `55.21 ns` (🚀 **2.73x faster**)    | `109.38 ns` (✅ **1.38x faster**)   | `707.53 ns` (❌ *4.69x slower*)    |
| **`serialize_uncompressed`**             | `191.98 ns` (✅ **1.00x**) | `282.10 ns` (❌ *1.47x slower*)   | `32.07 ns` (🚀 **5.99x faster**)    | `55.25 ns` (🚀 **3.47x faster**)    | `109.32 ns` (✅ **1.76x faster**)   | `698.57 ns` (❌ *3.64x slower*)    |
| **`deserialize_compressed`**             | `131.69 us` (✅ **1.00x**) | `263.99 us` (❌ *2.00x slower*)   | `52.20 ns` (🚀 **2522.47x faster**) | `93.64 ns` (🚀 **1406.25x faster**) | `217.68 ns` (🚀 **604.95x faster**) | `1.35 us` (🚀 **97.77x faster**)   |
| **`deserialize_compressed_unchecked`**   | `38.77 us` (✅ **1.00x**)  | `132.65 us` (❌ *3.42x slower*)   | `52.22 ns` (🚀 **742.41x faster**)  | `93.70 ns` (🚀 **413.71x faster**)  | `217.71 ns` (🚀 **178.06x faster**) | `1.35 us` (🚀 **28.81x faster**)   |
| **`deserialize_uncompressed`**           | `92.77 us` (✅ **1.00x**)  | `131.17 us` (❌ *1.41x slower*)   | `52.14 ns` (🚀 **1779.34x faster**) | `93.48 ns` (🚀 **992.44x faster**)  | `217.97 ns` (🚀 **425.60x faster**) | `1.35 us` (🚀 **68.92x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `196.39 ns` (✅ **1.00x**) | `398.40 ns` (❌ *2.03x slower*)   | `52.12 ns` (🚀 **3.77x faster**)    | `93.55 ns` (🚀 **2.10x faster**)    | `218.24 ns` (❌ *1.11x slower*)     | `1.35 us` (❌ *6.85x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `7.00 s` (❌ *3.01x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.35 us` (✅ **1.00x**) | `38.29 us` (❌ *1.51x slower*)   | `131.66 us` (❌ *5.19x slower*)    |
| **`legendre_for_qr`**    | `14.36 us` (✅ **1.00x**) | `38.48 us` (❌ *2.68x slower*)   | `38.62 us` (❌ *2.69x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.84 ns` (✅ **1.00x**) | `91.02 ns` (❌ *1.86x slower*)    |
| **`from_big-endian_bits`**    | `48.79 ns` (✅ **1.00x**) | `91.05 ns` (❌ *1.87x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `5.14 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.06 ns` (✅ **1.00x**) | `76.45 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.31 ns` (✅ **1.00x**) | `47.91 ns` (❌ *2.15x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

