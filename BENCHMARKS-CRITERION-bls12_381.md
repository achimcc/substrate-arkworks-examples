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
|        | `213.56 us` (✅ **1.00x**)        | `1.90 ms` (❌ *8.89x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.32 us` (✅ **1.00x**)   | `4.35 us` (❌ *3.28x slower*)   | `33.06 ns` (🚀 **40.02x faster**) | `210.39 ns` (🚀 **6.29x faster**)  | `22.82 ns` (🚀 **57.97x faster**) | `9.55 ns` (🚀 **138.54x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.39 us` (✅ **1.00x**)   | `4.36 us` (❌ *3.14x slower*)   | `31.82 ns` (🚀 **43.61x faster**) | `200.70 ns` (🚀 **6.91x faster**)  | `17.73 ns` (🚀 **78.28x faster**) | `10.39 ns` (🚀 **133.55x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `969.54 ns` (✅ **1.00x**) | `3.07 us` (❌ *3.17x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `997.49 ns` (✅ **1.00x**) | `3.21 us` (❌ *3.22x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `662.25 ns` (✅ **1.00x**) | `1.95 us` (❌ *2.94x slower*)   | `15.33 ns` (🚀 **43.20x faster**) | `129.17 ns` (🚀 **5.13x faster**)  | `8.95 ns` (🚀 **74.02x faster**)  | `6.38 ns` (🚀 **103.81x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `333.09 us` (✅ **1.00x**) | `1.03 ms` (❌ *3.09x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `27.34 ns` (❌ *3.90x slower*)    | `121.52 ns` (❌ *17.35x slower*)   | `20.21 ns` (❌ *2.89x slower*)    | `7.00 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `263.42 ns` (❌ *5.84x slower*)   | `6.78 us` (❌ *150.23x slower*)    | `82.80 ns` (❌ *1.84x slower*)    | `45.11 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `205.23 ns` (❌ *4.90x slower*)   | `4.76 us` (❌ *113.52x slower*)    | `68.53 ns` (❌ *1.63x slower*)    | `41.92 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.16 us` (❌ *2.38x slower*)    | `27.58 us` (❌ *4.06x slower*)     | `16.28 us` (❌ *2.40x slower*)    | `6.79 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `582.54 ns` (❌ *6.52x slower*)   | `13.89 us` (❌ *155.50x slower*)   | `129.16 ns` (❌ *1.45x slower*)   | `89.34 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `568.34 ns` (❌ *6.87x slower*)   | `13.82 us` (❌ *167.02x slower*)   | `184.89 ns` (❌ *2.23x slower*)   | `82.75 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `6.65 ns` (✅ **1.00x**) | `9.32 ns` (❌ *1.40x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `7.99 ns` (✅ **1.00x**) | `12.75 ns` (❌ *1.60x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `3.97 ns` (✅ **1.00x**) | `4.77 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.43 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `168.39 ns` (✅ **1.00x**) | `226.52 ns` (❌ *1.35x slower*)   | `30.45 ns` (🚀 **5.53x faster**)    | `60.94 ns` (🚀 **2.76x faster**)     | `119.89 ns` (✅ **1.40x faster**)   | `740.25 ns` (❌ *4.40x slower*)    |
| **`serialize_uncompressed`**             | `212.15 ns` (✅ **1.00x**) | `323.12 ns` (❌ *1.52x slower*)   | `30.78 ns` (🚀 **6.89x faster**)    | `58.09 ns` (🚀 **3.65x faster**)     | `116.15 ns` (🚀 **1.83x faster**)   | `740.78 ns` (❌ *3.49x slower*)    |
| **`deserialize_compressed`**             | `142.61 us` (✅ **1.00x**) | `288.54 us` (❌ *2.02x slower*)   | `48.65 ns` (🚀 **2931.18x faster**) | `113.89 ns` (🚀 **1252.13x faster**) | `245.47 ns` (🚀 **580.97x faster**) | `1.52 us` (🚀 **93.86x faster**)   |
| **`deserialize_compressed_unchecked`**   | `42.26 us` (✅ **1.00x**)  | `145.98 us` (❌ *3.45x slower*)   | `48.67 ns` (🚀 **868.44x faster**)  | `113.89 ns` (🚀 **371.09x faster**)  | `245.10 ns` (🚀 **172.44x faster**) | `1.50 us` (🚀 **28.13x faster**)   |
| **`deserialize_uncompressed`**           | `96.20 us` (✅ **1.00x**)  | `143.38 us` (❌ *1.49x slower*)   | `47.81 ns` (🚀 **2012.13x faster**) | `114.64 ns` (🚀 **839.16x faster**)  | `245.68 ns` (🚀 **391.57x faster**) | `1.51 us` (🚀 **63.69x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `235.40 ns` (✅ **1.00x**) | `487.81 ns` (❌ *2.07x slower*)   | `50.45 ns` (🚀 **4.67x faster**)    | `113.67 ns` (🚀 **2.07x faster**)    | `246.51 ns` (✅ **1.05x slower**)   | `1.51 us` (❌ *6.44x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.64 s` (✅ **1.00x**)  | `7.78 s` (❌ *2.95x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.97 us` (✅ **1.00x**) | `42.44 us` (❌ *1.85x slower*)   | `143.32 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `13.09 us` (✅ **1.00x**) | `42.53 us` (❌ *3.25x slower*)   | `42.56 us` (❌ *3.25x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.23 ns` (✅ **1.00x**)  | `4.93 ns` (❌ *1.17x slower*)      |
| **`from_little-endian_bits`** | `63.90 ns` (✅ **1.00x**) | `126.40 ns` (❌ *1.98x slower*)    |
| **`from_big-endian_bits`**    | `61.85 ns` (✅ **1.00x**) | `127.42 ns` (❌ *2.06x slower*)    |
| **`comparison`**              | `4.40 ns` (✅ **1.00x**)  | `5.11 ns` (❌ *1.16x slower*)      |
| **`equality`**                | `5.00 ns` (✅ **1.00x**)  | `5.50 ns` (✅ **1.10x slower**)    |
| **`is_zero`**                 | `4.74 ns` (✅ **1.00x**)  | `4.72 ns` (✅ **1.01x faster**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.21 ns` (✅ **1.00x**) | `95.71 ns` (❌ *2.16x slower*)    |
| **`into_bigint`** | `25.54 ns` (✅ **1.00x**) | `49.11 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

