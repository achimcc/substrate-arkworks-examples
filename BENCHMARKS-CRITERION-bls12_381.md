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
|        | `180.46 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.98x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.63 us` (❌ *3.24x slower*)     | `29.80 ns` (🚀 **37.56x faster**) | `179.26 ns` (🚀 **6.24x faster**)  | `19.44 ns` (🚀 **57.58x faster**) | `8.18 ns` (🚀 **136.81x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.15 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.20x slower*)     | `27.26 ns` (🚀 **42.24x faster**) | `168.83 ns` (🚀 **6.82x faster**)  | `14.76 ns` (🚀 **78.02x faster**) | `8.54 ns` (🚀 **134.80x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `818.79 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.19x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `844.27 ns` (✅ **1.00x**) | `2.66 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `563.00 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.91x slower*)     | `13.07 ns` (🚀 **43.07x faster**) | `101.54 ns` (🚀 **5.54x faster**)  | `7.62 ns` (🚀 **73.93x faster**)  | `5.46 ns` (🚀 **103.02x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.46 us` (✅ **1.00x**) | `868.13 us` (❌ *3.06x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.89 ns` (❌ *3.82x slower*)    | `104.74 ns` (❌ *17.50x slower*)   | `17.13 ns` (❌ *2.86x slower*)    | `5.98 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `223.50 ns` (❌ *5.77x slower*)   | `5.74 us` (❌ *148.04x slower*)    | `70.26 ns` (❌ *1.81x slower*)    | `38.74 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `174.10 ns` (❌ *4.91x slower*)   | `4.03 us` (❌ *113.75x slower*)    | `58.24 ns` (❌ *1.64x slower*)    | `35.47 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.86 us` (❌ *2.16x slower*)    | `23.03 us` (❌ *3.58x slower*)     | `13.56 us` (❌ *2.11x slower*)    | `6.43 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.32 ns` (❌ *6.00x slower*)   | `11.74 us` (❌ *142.78x slower*)   | `107.19 ns` (❌ *1.30x slower*)   | `82.25 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `474.36 ns` (❌ *5.88x slower*)   | `11.65 us` (❌ *144.59x slower*)   | `155.95 ns` (❌ *1.93x slower*)   | `80.61 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.74 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `141.68 ns` (✅ **1.00x**) | `191.38 ns` (❌ *1.35x slower*)   | `30.19 ns` (🚀 **4.69x faster**)    | `49.55 ns` (🚀 **2.86x faster**)    | `97.86 ns` (✅ **1.45x faster**)    | `631.75 ns` (❌ *4.46x slower*)    |
| **`serialize_uncompressed`**             | `179.53 ns` (✅ **1.00x**) | `267.70 ns` (❌ *1.49x slower*)   | `30.09 ns` (🚀 **5.97x faster**)    | `49.40 ns` (🚀 **3.63x faster**)    | `97.87 ns` (🚀 **1.83x faster**)    | `629.97 ns` (❌ *3.51x slower*)    |
| **`deserialize_compressed`**             | `118.18 us` (✅ **1.00x**) | `241.54 us` (❌ *2.04x slower*)   | `46.50 ns` (🚀 **2541.65x faster**) | `93.83 ns` (🚀 **1259.47x faster**) | `209.72 ns` (🚀 **563.49x faster**) | `1.27 us` (🚀 **92.95x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.17 us` (✅ **1.00x**)  | `122.92 us` (❌ *3.40x slower*)   | `46.50 ns` (🚀 **777.85x faster**)  | `93.82 ns` (🚀 **385.51x faster**)  | `209.68 ns` (🚀 **172.49x faster**) | `1.26 us` (🚀 **28.60x faster**)   |
| **`deserialize_uncompressed`**           | `82.02 us` (✅ **1.00x**)  | `118.28 us` (❌ *1.44x slower*)   | `46.50 ns` (🚀 **1764.11x faster**) | `93.82 ns` (🚀 **874.23x faster**)  | `209.72 ns` (🚀 **391.12x faster**) | `1.26 us` (🚀 **64.89x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `199.89 ns` (✅ **1.00x**) | `413.57 ns` (❌ *2.07x slower*)   | `46.49 ns` (🚀 **4.30x faster**)    | `93.85 ns` (🚀 **2.13x faster**)    | `209.66 ns` (✅ **1.05x slower**)   | `1.26 us` (❌ *6.32x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.23 s` (✅ **1.00x**)  | `6.59 s` (❌ *2.96x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.06 us` (✅ **1.00x**) | `35.78 us` (❌ *1.62x slower*)   | `121.92 us` (❌ *5.53x slower*)    |
| **`legendre_for_qr`**    | `12.33 us` (✅ **1.00x**) | `35.70 us` (❌ *2.89x slower*)   | `35.86 us` (❌ *2.91x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.90 ns` (✅ **1.00x**) | `108.27 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `60.75 ns` (✅ **1.00x**) | `108.18 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.67 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.95 ns` (✅ **1.00x**) | `78.99 ns` (❌ *2.20x slower*)    |
| **`into_bigint`** | `21.63 ns` (✅ **1.00x**) | `41.41 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

