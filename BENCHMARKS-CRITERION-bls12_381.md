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
|        | `222.30 us` (✅ **1.00x**)        | `1.94 ms` (❌ *8.75x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.42 us` (✅ **1.00x**)   | `4.36 us` (❌ *3.07x slower*)   | `34.65 ns` (🚀 **40.92x faster**) | `215.18 ns` (🚀 **6.59x faster**)  | `24.34 ns` (🚀 **58.25x faster**) | `9.78 ns` (🚀 **144.97x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.47 us` (✅ **1.00x**)   | `4.42 us` (❌ *3.01x slower*)   | `32.84 ns` (🚀 **44.62x faster**) | `204.04 ns` (🚀 **7.18x faster**)  | `18.00 ns` (🚀 **81.41x faster**) | `10.19 ns` (🚀 **143.83x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `976.83 ns` (✅ **1.00x**) | `3.28 us` (❌ *3.36x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.01 us` (✅ **1.00x**)   | `3.16 us` (❌ *3.15x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `671.67 ns` (✅ **1.00x**) | `1.97 us` (❌ *2.94x slower*)   | `16.22 ns` (🚀 **41.41x faster**) | `122.57 ns` (🚀 **5.48x faster**)  | `9.10 ns` (🚀 **73.78x faster**)  | `6.42 ns` (🚀 **104.65x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `339.22 us` (✅ **1.00x**) | `1.04 ms` (❌ *3.06x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `26.92 ns` (❌ *3.81x slower*)    | `124.94 ns` (❌ *17.69x slower*)   | `20.08 ns` (❌ *2.84x slower*)    | `7.06 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `268.83 ns` (❌ *5.84x slower*)   | `6.86 us` (❌ *148.94x slower*)    | `84.15 ns` (❌ *1.83x slower*)    | `46.07 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `208.68 ns` (❌ *4.94x slower*)   | `4.81 us` (❌ *113.96x slower*)    | `69.72 ns` (❌ *1.65x slower*)    | `42.23 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.47 us` (❌ *2.14x slower*)    | `27.55 us` (❌ *3.58x slower*)     | `16.10 us` (❌ *2.09x slower*)    | `7.70 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `590.93 ns` (❌ *6.03x slower*)   | `14.06 us` (❌ *143.45x slower*)   | `127.69 ns` (❌ *1.30x slower*)   | `98.01 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `570.28 ns` (❌ *5.91x slower*)   | `14.00 us` (❌ *145.04x slower*)   | `187.22 ns` (❌ *1.94x slower*)   | `96.55 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**) | `9.40 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.36 ns` (✅ **1.00x**) | `13.00 ns` (❌ *1.39x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.67 ns` (✅ **1.00x**) | `4.82 ns` (✅ **1.03x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.49 ns` (✅ **1.00x**) | `4.48 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `167.80 ns` (✅ **1.00x**) | `242.62 ns` (❌ *1.45x slower*)   | `35.45 ns` (🚀 **4.73x faster**)    | `59.36 ns` (🚀 **2.83x faster**)     | `115.99 ns` (✅ **1.45x faster**)   | `799.77 ns` (❌ *4.77x slower*)    |
| **`serialize_uncompressed`**             | `214.40 ns` (✅ **1.00x**) | `318.84 ns` (❌ *1.49x slower*)   | `35.27 ns` (🚀 **6.08x faster**)    | `59.27 ns` (🚀 **3.62x faster**)     | `119.09 ns` (🚀 **1.80x faster**)   | `747.26 ns` (❌ *3.49x slower*)    |
| **`deserialize_compressed`**             | `146.29 us` (✅ **1.00x**) | `299.84 us` (❌ *2.05x slower*)   | `55.28 ns` (🚀 **2646.28x faster**) | `114.33 ns` (🚀 **1279.50x faster**) | `246.73 ns` (🚀 **592.91x faster**) | `1.52 us` (🚀 **96.38x faster**)   |
| **`deserialize_compressed_unchecked`**   | `44.33 us` (✅ **1.00x**)  | `147.72 us` (❌ *3.33x slower*)   | `55.38 ns` (🚀 **800.53x faster**)  | `114.51 ns` (🚀 **387.17x faster**)  | `244.84 ns` (🚀 **181.07x faster**) | `1.51 us` (🚀 **29.36x faster**)   |
| **`deserialize_uncompressed`**           | `97.69 us` (✅ **1.00x**)  | `142.15 us` (❌ *1.46x slower*)   | `55.24 ns` (🚀 **1768.38x faster**) | `112.32 ns` (🚀 **869.73x faster**)  | `254.57 ns` (🚀 **383.73x faster**) | `1.52 us` (🚀 **64.32x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `240.80 ns` (✅ **1.00x**) | `496.11 ns` (❌ *2.06x slower*)   | `55.19 ns` (🚀 **4.36x faster**)    | `112.99 ns` (🚀 **2.13x faster**)    | `247.37 ns` (✅ **1.03x slower**)   | `1.52 us` (❌ *6.32x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.67 s` (✅ **1.00x**)  | `7.96 s` (❌ *2.98x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `26.38 us` (✅ **1.00x**) | `42.92 us` (❌ *1.63x slower*)   | `146.32 us` (❌ *5.55x slower*)    |
| **`legendre_for_qr`**    | `14.93 us` (✅ **1.00x**) | `42.78 us` (❌ *2.87x slower*)   | `43.08 us` (❌ *2.89x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.76 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.06x slower**)    |
| **`from_little-endian_bits`** | `75.95 ns` (✅ **1.00x**) | `129.49 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `73.04 ns` (✅ **1.00x**) | `129.56 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.92 ns` (✅ **1.00x**)  | `5.47 ns` (❌ *1.11x slower*)      |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.60 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `4.66 ns` (✅ **1.00x**)  | `4.80 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.81 ns` (✅ **1.00x**) | `94.20 ns` (❌ *2.20x slower*)    |
| **`into_bigint`** | `25.99 ns` (✅ **1.00x**) | `49.57 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

