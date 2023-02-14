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
|        | `292.37 us` (✅ **1.00x**)        | `2.34 ms` (❌ *8.00x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.83 us` (✅ **1.00x**)   | `5.20 us` (❌ *2.84x slower*)   | `34.77 ns` (🚀 **52.70x faster**) | `230.51 ns` (🚀 **7.95x faster**)  | `24.78 ns` (🚀 **73.95x faster**) | `11.18 ns` (🚀 **163.95x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.86 us` (✅ **1.00x**)   | `5.00 us` (❌ *2.69x slower*)   | `35.17 ns` (🚀 **52.82x faster**) | `219.40 ns` (🚀 **8.47x faster**)  | `20.04 ns` (🚀 **92.68x faster**) | `11.91 ns` (🚀 **155.96x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.31 us` (✅ **1.00x**)   | `3.54 us` (❌ *2.72x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.34 us` (✅ **1.00x**)   | `3.54 us` (❌ *2.65x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `877.48 ns` (✅ **1.00x**) | `2.44 us` (❌ *2.78x slower*)   | `17.41 ns` (🚀 **50.39x faster**) | `131.14 ns` (🚀 **6.69x faster**)  | `9.91 ns` (🚀 **88.55x faster**)  | `6.73 ns` (🚀 **130.34x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `444.87 us` (✅ **1.00x**) | `1.23 ms` (❌ *2.76x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `28.75 ns` (❌ *3.47x slower*)    | `139.05 ns` (❌ *16.78x slower*)   | `22.51 ns` (❌ *2.72x slower*)    | `8.29 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `292.04 ns` (❌ *5.70x slower*)   | `7.40 us` (❌ *144.50x slower*)    | `89.86 ns` (❌ *1.75x slower*)    | `51.23 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `260.40 ns` (❌ *5.89x slower*)   | `5.19 us` (❌ *117.46x slower*)    | `76.25 ns` (❌ *1.73x slower*)    | `44.17 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.93 us` (❌ *2.31x slower*)    | `29.72 us` (❌ *4.06x slower*)     | `16.49 us` (❌ *2.25x slower*)    | `7.32 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `635.10 ns` (❌ *6.08x slower*)   | `15.44 us` (❌ *147.85x slower*)   | `142.62 ns` (❌ *1.37x slower*)   | `104.43 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `611.59 ns` (❌ *5.81x slower*)   | `15.15 us` (❌ *144.03x slower*)   | `226.15 ns` (❌ *2.15x slower*)   | `105.20 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.14 ns` (✅ **1.00x**)  | `10.89 ns` (❌ *1.34x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.71 ns` (✅ **1.00x**) | `14.66 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.61 ns` (✅ **1.00x**)  | `4.76 ns` (✅ **1.03x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.36 ns` (✅ **1.00x**)  | `4.33 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `196.62 ns` (✅ **1.00x**) | `269.57 ns` (❌ *1.37x slower*)   | `39.31 ns` (🚀 **5.00x faster**)    | `62.43 ns` (🚀 **3.15x faster**)     | `120.35 ns` (✅ **1.63x faster**)   | `799.48 ns` (❌ *4.07x slower*)    |
| **`serialize_uncompressed`**             | `245.40 ns` (✅ **1.00x**) | `370.54 ns` (❌ *1.51x slower*)   | `39.00 ns` (🚀 **6.29x faster**)    | `61.10 ns` (🚀 **4.02x faster**)     | `123.80 ns` (🚀 **1.98x faster**)   | `783.14 ns` (❌ *3.19x slower*)    |
| **`deserialize_compressed`**             | `186.43 us` (✅ **1.00x**) | `367.99 us` (❌ *1.97x slower*)   | `62.10 ns` (🚀 **3001.91x faster**) | `118.13 ns` (🚀 **1578.24x faster**) | `289.63 ns` (🚀 **643.68x faster**) | `1.96 us` (🚀 **95.32x faster**)   |
| **`deserialize_compressed_unchecked`**   | `59.79 us` (✅ **1.00x**)  | `191.35 us` (❌ *3.20x slower*)   | `61.73 ns` (🚀 **968.43x faster**)  | `118.21 ns` (🚀 **505.75x faster**)  | `297.85 ns` (🚀 **200.72x faster**) | `1.94 us` (🚀 **30.86x faster**)   |
| **`deserialize_uncompressed`**           | `129.44 us` (✅ **1.00x**) | `168.15 us` (❌ *1.30x slower*)   | `63.88 ns` (🚀 **2026.26x faster**) | `118.04 ns` (🚀 **1096.56x faster**) | `292.13 ns` (🚀 **443.08x faster**) | `1.92 us` (🚀 **67.53x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `283.83 ns` (✅ **1.00x**) | `581.35 ns` (❌ *2.05x slower*)   | `64.41 ns` (🚀 **4.41x faster**)    | `117.67 ns` (🚀 **2.41x faster**)    | `292.45 ns` (✅ **1.03x slower**)   | `1.94 us` (❌ *6.82x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `3.49 s` (✅ **1.00x**)  | `9.05 s` (❌ *2.59x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `28.22 us` (✅ **1.00x**) | `58.58 us` (❌ *2.08x slower*)   | `197.81 us` (❌ *7.01x slower*)    |
| **`legendre_for_qr`**    | `16.23 us` (✅ **1.00x**) | `58.80 us` (❌ *3.62x slower*)   | `60.17 us` (❌ *3.71x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.60 ns` (✅ **1.00x**)  | `4.93 ns` (✅ **1.07x slower**)    |
| **`from_little-endian_bits`** | `76.24 ns` (✅ **1.00x**) | `135.68 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `75.17 ns` (✅ **1.00x**) | `136.26 ns` (❌ *1.81x slower*)    |
| **`comparison`**              | `4.68 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.07x slower**)    |
| **`equality`**                | `5.10 ns` (✅ **1.00x**)  | `5.98 ns` (❌ *1.17x slower*)      |
| **`is_zero`**                 | `4.42 ns` (✅ **1.00x**)  | `4.82 ns` (✅ **1.09x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `46.36 ns` (✅ **1.00x**) | `107.37 ns` (❌ *2.32x slower*)    |
| **`into_bigint`** | `27.67 ns` (✅ **1.00x**) | `51.72 ns` (❌ *1.87x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

