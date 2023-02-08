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
|        | `202.37 us` (✅ **1.00x**)        | `1.79 ms` (❌ *8.84x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `3.93 us` (❌ *3.23x slower*)     | `23.19 ns` (🚀 **52.48x faster**) | `191.29 ns` (🚀 **6.36x faster**)  | `12.69 ns` (🚀 **95.94x faster**) | `8.67 ns` (🚀 **140.42x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `3.99 us` (❌ *3.18x slower*)     | `23.44 ns` (🚀 **53.48x faster**) | `159.42 ns` (🚀 **7.86x faster**)  | `12.85 ns` (🚀 **97.51x faster**) | `8.79 ns` (🚀 **142.58x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `880.65 ns` (✅ **1.00x**) | `2.82 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `910.03 ns` (✅ **1.00x**) | `2.86 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `581.56 ns` (✅ **1.00x**) | `1.79 us` (❌ *3.08x slower*)     | `12.56 ns` (🚀 **46.28x faster**) | `72.08 ns` (🚀 **8.07x faster**)   | `7.24 ns` (🚀 **80.30x faster**)  | `5.88 ns` (🚀 **98.94x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `320.87 us` (✅ **1.00x**) | `956.97 us` (❌ *2.98x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.16 ns` (❌ *3.59x slower*)    | `100.01 ns` (❌ *16.19x slower*)   | `18.29 ns` (❌ *2.96x slower*)    | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `239.25 ns` (❌ *5.21x slower*)   | `6.18 us` (❌ *134.48x slower*)    | `76.41 ns` (❌ *1.66x slower*)    | `45.96 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `173.20 ns` (❌ *4.56x slower*)   | `4.34 us` (❌ *114.12x slower*)    | `65.17 ns` (❌ *1.71x slower*)    | `38.01 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.22 us` (❌ *2.13x slower*)    | `25.39 us` (❌ *3.56x slower*)     | `14.89 us` (❌ *2.09x slower*)    | `7.13 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.44 ns` (❌ *6.21x slower*)   | `12.65 us` (❌ *151.25x slower*)   | `115.22 ns` (❌ *1.38x slower*)   | `83.64 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.45 ns` (❌ *5.81x slower*)   | `12.59 us` (❌ *143.64x slower*)   | `163.15 ns` (❌ *1.86x slower*)   | `87.62 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `10.30 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.81 ns` (✅ **1.00x**) | `4.80 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.53 ns` (✅ **1.00x**) | `202.58 ns` (❌ *1.35x slower*)   | `32.62 ns` (🚀 **4.61x faster**)    | `56.09 ns` (🚀 **2.68x faster**)    | `109.33 ns` (✅ **1.38x faster**)   | `698.80 ns` (❌ *4.64x slower*)    |
| **`serialize_uncompressed`**             | `192.39 ns` (✅ **1.00x**) | `282.46 ns` (❌ *1.47x slower*)   | `31.89 ns` (🚀 **6.03x faster**)    | `55.25 ns` (🚀 **3.48x faster**)    | `109.35 ns` (✅ **1.76x faster**)   | `698.34 ns` (❌ *3.63x slower*)    |
| **`deserialize_compressed`**             | `131.53 us` (✅ **1.00x**) | `264.81 us` (❌ *2.01x slower*)   | `52.38 ns` (🚀 **2511.05x faster**) | `93.79 ns` (🚀 **1402.35x faster**) | `215.87 ns` (🚀 **609.28x faster**) | `1.27 us` (🚀 **103.52x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.82 us` (✅ **1.00x**)  | `132.58 us` (❌ *3.42x slower*)   | `52.39 ns` (🚀 **740.99x faster**)  | `93.91 ns` (🚀 **413.40x faster**)  | `215.92 ns` (🚀 **179.80x faster**) | `1.27 us` (🚀 **30.56x faster**)   |
| **`deserialize_uncompressed`**           | `92.64 us` (✅ **1.00x**)  | `131.95 us` (❌ *1.42x slower*)   | `52.35 ns` (🚀 **1769.61x faster**) | `93.56 ns` (🚀 **990.08x faster**)  | `215.84 ns` (🚀 **429.18x faster**) | `1.27 us` (🚀 **72.87x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `201.93 ns` (✅ **1.00x**) | `407.23 ns` (❌ *2.02x slower*)   | `52.36 ns` (🚀 **3.86x faster**)    | `93.76 ns` (🚀 **2.15x faster**)    | `215.83 ns` (✅ **1.07x slower**)   | `1.27 us` (❌ *6.29x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `7.00 s` (❌ *3.02x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.41 us` (✅ **1.00x**) | `38.30 us` (❌ *1.51x slower*)   | `131.49 us` (❌ *5.17x slower*)    |
| **`legendre_for_qr`**    | `14.39 us` (✅ **1.00x**) | `38.46 us` (❌ *2.67x slower*)   | `39.68 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.55 ns` (✅ **1.00x**) | `88.61 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `48.51 ns` (✅ **1.00x**) | `88.51 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.89 ns` (✅ **1.00x**) | `76.52 ns` (❌ *1.87x slower*)    |
| **`into_bigint`** | `22.45 ns` (✅ **1.00x**) | `47.92 ns` (❌ *2.13x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

