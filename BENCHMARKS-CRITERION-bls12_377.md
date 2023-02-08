# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377](#sample_bls12_377)
    - [arithmetic_for_bls12_377](#arithmetic_for_bls12_377)
    - [serialization_for_bls12_377](#serialization_for_bls12_377)
    - [msm_for_bls12_377](#msm_for_bls12_377)
    - [squareroot_for_bls12_377](#squareroot_for_bls12_377)
    - [bitwise_operations_for_bls12_377](#bitwise_operations_for_bls12_377)
    - [conversions_for_bls12_377](#conversions_for_bls12_377)

## Benchmark Results

### sample_bls12_377

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `192.70 us` (✅ **1.00x**)        | `2.02 ms` (❌ *10.47x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.21 us` (✅ **1.00x**)   | `4.76 us` (❌ *3.94x slower*)   | `23.42 ns` (🚀 **51.63x faster**) | `189.67 ns` (🚀 **6.38x faster**)  | `12.51 ns` (🚀 **96.68x faster**) | `8.71 ns` (🚀 **138.74x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.82 us` (❌ *3.84x slower*)   | `23.33 ns` (🚀 **53.84x faster**) | `160.95 ns` (🚀 **7.80x faster**)  | `12.70 ns` (🚀 **98.92x faster**) | `8.80 ns` (🚀 **142.74x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `882.91 ns` (✅ **1.00x**) | `3.40 us` (❌ *3.85x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `914.98 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.76x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `601.69 ns` (✅ **1.00x**) | `2.25 us` (❌ *3.73x slower*)   | `12.38 ns` (🚀 **48.60x faster**) | `69.59 ns` (🚀 **8.65x faster**)   | `7.13 ns` (🚀 **84.41x faster**)  | `5.92 ns` (🚀 **101.64x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `316.60 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.66x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `23.39 ns` (❌ *3.80x slower*)    | `95.86 ns` (❌ *15.57x slower*)    | `18.67 ns` (❌ *3.03x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `268.15 ns` (❌ *6.01x slower*)   | `7.09 us` (❌ *158.86x slower*)    | `75.78 ns` (❌ *1.70x slower*)    | `44.64 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `233.41 ns` (❌ *6.49x slower*)   | `5.00 us` (❌ *139.10x slower*)    | `66.82 ns` (❌ *1.86x slower*)    | `35.97 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.11 us` (❌ *2.14x slower*)    | `27.46 us` (❌ *3.89x slower*)     | `14.81 us` (❌ *2.10x slower*)    | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `579.32 ns` (❌ *9.38x slower*)   | `14.53 us` (❌ *235.18x slower*)   | `118.26 ns` (❌ *1.91x slower*)   | `61.79 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `568.13 ns` (❌ *6.39x slower*)   | `14.46 us` (❌ *162.56x slower*)   | `162.43 ns` (❌ *1.83x slower*)   | `88.93 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.66 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**) | `10.40 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `156.14 ns` (✅ **1.00x**) | `219.44 ns` (❌ *1.41x slower*)   | `31.28 ns` (🚀 **4.99x faster**)    | `57.97 ns` (🚀 **2.69x faster**)    | `110.32 ns` (✅ **1.42x faster**)    | `698.24 ns` (❌ *4.47x slower*)    |
| **`serialize_uncompressed`**             | `212.44 ns` (✅ **1.00x**) | `330.65 ns` (❌ *1.56x slower*)   | `30.36 ns` (🚀 **7.00x faster**)    | `55.84 ns` (🚀 **3.80x faster**)    | `110.48 ns` (🚀 **1.92x faster**)    | `698.14 ns` (❌ *3.29x slower*)    |
| **`deserialize_compressed`**             | `309.91 us` (✅ **1.00x**) | `1.05 ms` (❌ *3.40x slower*)     | `51.95 ns` (🚀 **5965.97x faster**) | `93.09 ns` (🚀 **3329.17x faster**) | `212.56 ns` (🚀 **1457.98x faster**) | `1.30 us` (🚀 **238.05x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.79 us` (✅ **1.00x**)  | `183.10 us` (❌ *2.70x slower*)   | `52.04 ns` (🚀 **1302.70x faster**) | `92.73 ns` (🚀 **731.04x faster**)  | `212.32 ns` (🚀 **319.29x faster**)  | `1.30 us` (🚀 **52.06x faster**)   |
| **`deserialize_uncompressed`**           | `242.11 us` (✅ **1.00x**) | `868.24 us` (❌ *3.59x slower*)   | `51.97 ns` (🚀 **4658.29x faster**) | `92.65 ns` (🚀 **2613.29x faster**) | `212.24 ns` (🚀 **1140.73x faster**) | `1.30 us` (🚀 **186.04x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `230.70 ns` (✅ **1.00x**) | `468.30 ns` (❌ *2.03x slower*)   | `51.96 ns` (🚀 **4.44x faster**)    | `92.53 ns` (🚀 **2.49x faster**)    | `212.31 ns` (✅ **1.09x faster**)    | `1.30 us` (❌ *5.64x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `8.22 s` (❌ *3.54x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.13 us` (✅ **1.00x**) | `67.42 us` (❌ *2.17x slower*)   | `182.12 us` (❌ *5.85x slower*)    |
| **`legendre_for_qr`**    | `10.93 us` (✅ **1.00x**) | `31.71 us` (❌ *2.90x slower*)   | `31.57 us` (❌ *2.89x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.37 ns` (✅ **1.00x**) | `83.56 ns` (❌ *1.69x slower*)    |
| **`from_big-endian_bits`**    | `49.29 ns` (✅ **1.00x**) | `83.48 ns` (❌ *1.69x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.19 ns` (✅ **1.06x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.70 ns` (✅ **1.00x**) | `75.33 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.52 ns` (✅ **1.00x**) | `46.89 ns` (❌ *2.08x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

