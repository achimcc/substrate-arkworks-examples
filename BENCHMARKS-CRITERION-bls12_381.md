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
|        | `203.72 us` (✅ **1.00x**)        | `1.81 ms` (❌ *8.87x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `3.98 us` (❌ *3.27x slower*)     | `23.32 ns` (🚀 **52.22x faster**) | `178.70 ns` (🚀 **6.81x faster**)  | `12.67 ns` (🚀 **96.11x faster**) | `8.68 ns` (🚀 **140.29x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.04 us` (❌ *3.20x slower*)     | `23.34 ns` (🚀 **54.18x faster**) | `157.09 ns` (🚀 **8.05x faster**)  | `12.84 ns` (🚀 **98.45x faster**) | `8.78 ns` (🚀 **143.96x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `880.77 ns` (✅ **1.00x**) | `2.86 us` (❌ *3.25x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `921.16 ns` (✅ **1.00x**) | `2.89 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `572.60 ns` (✅ **1.00x**) | `1.81 us` (❌ *3.15x slower*)     | `12.46 ns` (🚀 **45.95x faster**) | `67.41 ns` (🚀 **8.49x faster**)   | `7.20 ns` (🚀 **79.58x faster**)  | `6.12 ns` (🚀 **93.62x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `322.15 us` (✅ **1.00x**) | `964.49 us` (❌ *2.99x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.27 ns` (❌ *3.61x slower*)    | `93.30 ns` (❌ *15.11x slower*)    | `18.27 ns` (❌ *2.96x slower*)    | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `243.92 ns` (❌ *5.31x slower*)   | `6.25 us` (❌ *136.00x slower*)    | `76.37 ns` (❌ *1.66x slower*)    | `45.95 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `174.85 ns` (❌ *4.67x slower*)   | `4.40 us` (❌ *117.68x slower*)    | `64.95 ns` (❌ *1.74x slower*)    | `37.43 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.06 us` (❌ *2.11x slower*)    | `25.42 us` (❌ *3.56x slower*)     | `14.75 us` (❌ *2.07x slower*)    | `7.14 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `527.74 ns` (❌ *6.31x slower*)   | `12.80 us` (❌ *153.00x slower*)   | `116.60 ns` (❌ *1.39x slower*)   | `83.67 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `518.34 ns` (❌ *5.93x slower*)   | `12.73 us` (❌ *145.67x slower*)   | `164.20 ns` (❌ *1.88x slower*)   | `87.42 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.35 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.78 ns` (✅ **1.00x**) | `4.80 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.64 ns` (✅ **1.00x**) | `203.02 ns` (❌ *1.35x slower*)   | `32.50 ns` (🚀 **4.63x faster**)    | `57.07 ns` (🚀 **2.64x faster**)    | `110.57 ns` (✅ **1.36x faster**)   | `700.21 ns` (❌ *4.65x slower*)    |
| **`serialize_uncompressed`**             | `191.51 ns` (✅ **1.00x**) | `282.76 ns` (❌ *1.48x slower*)   | `32.95 ns` (🚀 **5.81x faster**)    | `55.71 ns` (🚀 **3.44x faster**)    | `110.54 ns` (✅ **1.73x faster**)   | `700.53 ns` (❌ *3.66x slower*)    |
| **`deserialize_compressed`**             | `131.65 us` (✅ **1.00x**) | `267.91 us` (❌ *2.04x slower*)   | `64.64 ns` (🚀 **2036.57x faster**) | `93.63 ns` (🚀 **1406.08x faster**) | `214.87 ns` (🚀 **612.67x faster**) | `1.29 us` (🚀 **102.15x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.82 us` (✅ **1.00x**)  | `135.67 us` (❌ *3.41x slower*)   | `64.66 ns` (🚀 **615.85x faster**)  | `93.82 ns` (🚀 **424.46x faster**)  | `214.85 ns` (🚀 **185.35x faster**) | `1.30 us` (🚀 **30.54x faster**)   |
| **`deserialize_uncompressed`**           | `91.92 us` (✅ **1.00x**)  | `131.99 us` (❌ *1.44x slower*)   | `64.54 ns` (🚀 **1424.24x faster**) | `93.59 ns` (🚀 **982.13x faster**)  | `214.22 ns` (🚀 **429.07x faster**) | `1.29 us` (🚀 **71.32x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `192.42 ns` (✅ **1.00x**) | `400.33 ns` (❌ *2.08x slower*)   | `64.58 ns` (🚀 **2.98x faster**)    | `93.53 ns` (🚀 **2.06x faster**)    | `214.27 ns` (❌ *1.11x slower*)     | `1.29 us` (❌ *6.70x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `7.06 s` (❌ *3.04x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.45 us` (✅ **1.00x**) | `39.51 us` (❌ *1.55x slower*)   | `134.65 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.34 us` (✅ **1.00x**) | `39.73 us` (❌ *2.77x slower*)   | `39.67 us` (❌ *2.77x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.45 ns` (✅ **1.00x**) | `84.77 ns` (❌ *1.75x slower*)    |
| **`from_big-endian_bits`**    | `48.44 ns` (✅ **1.00x**) | `84.66 ns` (❌ *1.75x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.15 ns` (✅ **1.00x**) | `75.84 ns` (❌ *1.76x slower*)    |
| **`into_bigint`** | `22.41 ns` (✅ **1.00x**) | `48.07 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

