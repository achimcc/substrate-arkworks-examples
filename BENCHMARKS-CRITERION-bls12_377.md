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
|        | `192.05 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.56x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `4.77 us` (❌ *3.92x slower*)   | `23.26 ns` (🚀 **52.27x faster**) | `196.68 ns` (🚀 **6.18x faster**)  | `12.52 ns` (🚀 **97.08x faster**) | `8.72 ns` (🚀 **139.43x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.82 us` (❌ *3.83x slower*)   | `23.27 ns` (🚀 **54.07x faster**) | `158.73 ns` (🚀 **7.93x faster**)  | `12.70 ns` (🚀 **99.12x faster**) | `8.82 ns` (🚀 **142.73x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `889.75 ns` (✅ **1.00x**) | `3.42 us` (❌ *3.84x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `918.79 ns` (✅ **1.00x**) | `3.45 us` (❌ *3.75x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `584.61 ns` (✅ **1.00x**) | `2.26 us` (❌ *3.86x slower*)   | `12.36 ns` (🚀 **47.29x faster**) | `67.23 ns` (🚀 **8.70x faster**)   | `7.14 ns` (🚀 **81.87x faster**)  | `5.83 ns` (🚀 **100.24x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `316.12 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.67x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.66 ns` (❌ *3.68x slower*)    | `95.25 ns` (❌ *15.45x slower*)    | `18.17 ns` (❌ *2.95x slower*)    | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `268.58 ns` (❌ *6.13x slower*)   | `7.13 us` (❌ *162.73x slower*)    | `75.85 ns` (❌ *1.73x slower*)    | `43.80 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `233.10 ns` (❌ *6.55x slower*)   | `5.01 us` (❌ *140.79x slower*)    | `66.71 ns` (❌ *1.87x slower*)    | `35.60 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.16 us` (❌ *2.15x slower*)    | `27.48 us` (❌ *3.90x slower*)     | `14.85 us` (❌ *2.11x slower*)    | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `581.93 ns` (❌ *9.48x slower*)   | `14.57 us` (❌ *237.37x slower*)   | `118.59 ns` (❌ *1.93x slower*)   | `61.37 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `567.18 ns` (❌ *6.36x slower*)   | `14.48 us` (❌ *162.33x slower*)   | `163.61 ns` (❌ *1.83x slower*)   | `89.20 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.67 ns` (✅ **1.00x**) | `10.29 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.51 ns` (✅ **1.00x**) | `4.52 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `155.43 ns` (✅ **1.00x**) | `220.55 ns` (❌ *1.42x slower*)   | `30.36 ns` (🚀 **5.12x faster**)    | `55.69 ns` (🚀 **2.79x faster**)    | `108.89 ns` (✅ **1.43x faster**)    | `693.44 ns` (❌ *4.46x slower*)    |
| **`serialize_uncompressed`**             | `211.32 ns` (✅ **1.00x**) | `334.61 ns` (❌ *1.58x slower*)   | `32.21 ns` (🚀 **6.56x faster**)    | `55.50 ns` (🚀 **3.81x faster**)    | `109.04 ns` (🚀 **1.94x faster**)    | `694.27 ns` (❌ *3.29x slower*)    |
| **`deserialize_compressed`**             | `308.83 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.43x slower*)     | `52.38 ns` (🚀 **5896.33x faster**) | `94.10 ns` (🚀 **3281.76x faster**) | `220.35 ns` (🚀 **1401.52x faster**) | `1.28 us` (🚀 **240.74x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.86 us` (✅ **1.00x**)  | `183.27 us` (❌ *2.70x slower*)   | `52.37 ns` (🚀 **1295.80x faster**) | `94.12 ns` (🚀 **721.02x faster**)  | `220.63 ns` (🚀 **307.58x faster**)  | `1.28 us` (🚀 **52.93x faster**)   |
| **`deserialize_uncompressed`**           | `241.05 us` (✅ **1.00x**) | `872.41 us` (❌ *3.62x slower*)   | `52.18 ns` (🚀 **4619.74x faster**) | `93.95 ns` (🚀 **2565.60x faster**) | `220.04 ns` (🚀 **1095.45x faster**) | `1.28 us` (🚀 **188.06x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `228.49 ns` (✅ **1.00x**) | `478.36 ns` (❌ *2.09x slower*)   | `52.15 ns` (🚀 **4.38x faster**)    | `93.92 ns` (🚀 **2.43x faster**)    | `220.17 ns` (✅ **1.04x faster**)    | `1.28 us` (❌ *5.61x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `8.32 s` (❌ *3.58x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.30 us` (✅ **1.00x**) | `67.47 us` (❌ *2.16x slower*)   | `182.23 us` (❌ *5.82x slower*)    |
| **`legendre_for_qr`**    | `10.96 us` (✅ **1.00x**) | `31.73 us` (❌ *2.90x slower*)   | `31.58 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.96 ns` (✅ **1.00x**) | `83.23 ns` (❌ *1.70x slower*)    |
| **`from_big-endian_bits`**    | `48.96 ns` (✅ **1.00x**) | `83.30 ns` (❌ *1.70x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.81 ns` (✅ **1.00x**) | `75.09 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `23.33 ns` (✅ **1.00x**) | `46.92 ns` (❌ *2.01x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

