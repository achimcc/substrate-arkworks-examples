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
|        | `181.24 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.94x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.62 us` (❌ *3.24x slower*)     | `27.60 ns` (🚀 **40.53x faster**) | `180.01 ns` (🚀 **6.22x faster**)  | `19.47 ns` (🚀 **57.46x faster**) | `8.21 ns` (🚀 **136.33x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.18x slower*)     | `27.44 ns` (🚀 **42.15x faster**) | `168.92 ns` (🚀 **6.85x faster**)  | `15.11 ns` (🚀 **76.54x faster**) | `8.55 ns` (🚀 **135.18x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `816.32 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.19x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `839.79 ns` (✅ **1.00x**) | `2.64 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `560.92 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.93x slower*)     | `13.08 ns` (🚀 **42.89x faster**) | `99.26 ns` (🚀 **5.65x faster**)   | `7.64 ns` (🚀 **73.42x faster**)  | `5.40 ns` (🚀 **103.82x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `282.91 us` (✅ **1.00x**) | `866.42 us` (❌ *3.06x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.50 ns` (❌ *3.78x slower*)    | `101.34 ns` (❌ *17.04x slower*)   | `16.77 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `224.76 ns` (❌ *5.80x slower*)   | `5.75 us` (❌ *148.44x slower*)    | `70.30 ns` (❌ *1.81x slower*)    | `38.74 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `174.74 ns` (❌ *4.91x slower*)   | `4.04 us` (❌ *113.51x slower*)    | `58.49 ns` (❌ *1.64x slower*)    | `35.57 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.86 us` (❌ *2.15x slower*)    | `23.08 us` (❌ *3.58x slower*)     | `13.58 us` (❌ *2.10x slower*)    | `6.45 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `495.21 ns` (❌ *6.01x slower*)   | `11.74 us` (❌ *142.48x slower*)   | `107.13 ns` (❌ *1.30x slower*)   | `82.43 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `476.11 ns` (❌ *5.87x slower*)   | `11.66 us` (❌ *143.71x slower*)   | `156.26 ns` (❌ *1.93x slower*)   | `81.12 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.74 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `140.06 ns` (✅ **1.00x**) | `191.60 ns` (❌ *1.37x slower*)   | `30.11 ns` (🚀 **4.65x faster**)    | `49.61 ns` (🚀 **2.82x faster**)    | `97.97 ns` (✅ **1.43x faster**)    | `631.33 ns` (❌ *4.51x slower*)    |
| **`serialize_uncompressed`**             | `178.78 ns` (✅ **1.00x**) | `266.76 ns` (❌ *1.49x slower*)   | `30.04 ns` (🚀 **5.95x faster**)    | `49.58 ns` (🚀 **3.61x faster**)    | `97.99 ns` (🚀 **1.82x faster**)    | `631.40 ns` (❌ *3.53x slower*)    |
| **`deserialize_compressed`**             | `117.81 us` (✅ **1.00x**) | `242.13 us` (❌ *2.06x slower*)   | `44.83 ns` (🚀 **2627.84x faster**) | `95.11 ns` (🚀 **1238.58x faster**) | `205.79 ns` (🚀 **572.46x faster**) | `1.27 us` (🚀 **92.77x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.34 us` (✅ **1.00x**)  | `123.22 us` (❌ *3.39x slower*)   | `44.79 ns` (🚀 **811.25x faster**)  | `95.23 ns` (🚀 **381.60x faster**)  | `205.81 ns` (🚀 **176.57x faster**) | `1.27 us` (🚀 **28.62x faster**)   |
| **`deserialize_uncompressed`**           | `81.57 us` (✅ **1.00x**)  | `118.45 us` (❌ *1.45x slower*)   | `44.76 ns` (🚀 **1822.61x faster**) | `95.81 ns` (🚀 **851.37x faster**)  | `205.83 ns` (🚀 **396.30x faster**) | `1.27 us` (🚀 **64.23x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `201.49 ns` (✅ **1.00x**) | `413.23 ns` (❌ *2.05x slower*)   | `44.76 ns` (🚀 **4.50x faster**)    | `95.10 ns` (🚀 **2.12x faster**)    | `205.85 ns` (✅ **1.02x slower**)   | `1.27 us` (❌ *6.30x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.22 s` (✅ **1.00x**)  | `6.61 s` (❌ *2.97x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.13 us` (✅ **1.00x**) | `35.95 us` (❌ *1.62x slower*)   | `122.38 us` (❌ *5.53x slower*)    |
| **`legendre_for_qr`**    | `12.50 us` (✅ **1.00x**) | `35.81 us` (❌ *2.86x slower*)   | `36.07 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.75 ns` (✅ **1.00x**) | `108.35 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `60.76 ns` (✅ **1.00x**) | `108.28 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.99 ns` (✅ **1.00x**) | `78.77 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.57 ns` (✅ **1.00x**) | `41.45 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

