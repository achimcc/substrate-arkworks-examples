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
|        | `182.00 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.90x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.21 us` (❌ *2.87x slower*)     | `26.94 ns` (🚀 **41.56x faster**) | `175.30 ns` (🚀 **6.39x faster**)  | `19.31 ns` (🚀 **58.01x faster**) | `8.21 ns` (🚀 **136.33x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.19x slower*)     | `27.92 ns` (🚀 **41.37x faster**) | `175.43 ns` (🚀 **6.58x faster**)  | `15.22 ns` (🚀 **75.91x faster**) | `8.62 ns` (🚀 **133.98x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `819.76 ns` (✅ **1.00x**) | `2.62 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `842.88 ns` (✅ **1.00x**) | `2.65 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `562.43 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.92x slower*)     | `14.51 ns` (🚀 **38.76x faster**) | `87.71 ns` (🚀 **6.41x faster**)   | `11.52 ns` (🚀 **48.84x faster**) | `5.42 ns` (🚀 **103.76x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.41 us` (✅ **1.00x**) | `869.34 us` (❌ *3.07x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `23.37 ns` (❌ *3.93x slower*)    | `103.89 ns` (❌ *17.47x slower*)   | `17.17 ns` (❌ *2.89x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `196.79 ns` (❌ *5.07x slower*)   | `5.78 us` (❌ *148.86x slower*)    | `70.29 ns` (❌ *1.81x slower*)    | `38.83 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `173.85 ns` (❌ *4.90x slower*)   | `4.05 us` (❌ *114.01x slower*)    | `58.11 ns` (❌ *1.64x slower*)    | `35.51 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.90 us` (❌ *2.13x slower*)    | `23.11 us` (❌ *3.53x slower*)     | `13.61 us` (❌ *2.08x slower*)    | `6.54 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.55 ns` (❌ *6.06x slower*)   | `11.80 us` (❌ *144.80x slower*)   | `106.13 ns` (❌ *1.30x slower*)   | `81.48 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `475.31 ns` (❌ *5.88x slower*)   | `11.72 us` (❌ *144.91x slower*)   | `155.49 ns` (❌ *1.92x slower*)   | `80.88 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.72 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `141.38 ns` (✅ **1.00x**) | `192.95 ns` (❌ *1.36x slower*)   | `29.89 ns` (🚀 **4.73x faster**)    | `49.61 ns` (🚀 **2.85x faster**)    | `97.65 ns` (✅ **1.45x faster**)    | `628.20 ns` (❌ *4.44x slower*)    |
| **`serialize_uncompressed`**             | `178.98 ns` (✅ **1.00x**) | `267.51 ns` (❌ *1.49x slower*)   | `29.83 ns` (🚀 **6.00x faster**)    | `50.39 ns` (🚀 **3.55x faster**)    | `97.67 ns` (🚀 **1.83x faster**)    | `627.41 ns` (❌ *3.51x slower*)    |
| **`deserialize_compressed`**             | `118.31 us` (✅ **1.00x**) | `241.73 us` (❌ *2.04x slower*)   | `46.78 ns` (🚀 **2529.23x faster**) | `95.91 ns` (🚀 **1233.59x faster**) | `206.49 ns` (🚀 **572.96x faster**) | `1.27 us` (🚀 **92.89x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.48 us` (✅ **1.00x**)  | `123.59 us` (❌ *3.39x slower*)   | `46.88 ns` (🚀 **778.12x faster**)  | `95.33 ns` (🚀 **382.67x faster**)  | `205.13 ns` (🚀 **177.84x faster**) | `1.27 us` (🚀 **28.66x faster**)   |
| **`deserialize_uncompressed`**           | `81.76 us` (✅ **1.00x**)  | `117.99 us` (❌ *1.44x slower*)   | `46.64 ns` (🚀 **1752.90x faster**) | `95.32 ns` (🚀 **857.79x faster**)  | `205.26 ns` (🚀 **398.33x faster**) | `1.27 us` (🚀 **64.20x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `201.43 ns` (✅ **1.00x**) | `415.22 ns` (❌ *2.06x slower*)   | `46.85 ns` (🚀 **4.30x faster**)    | `84.58 ns` (🚀 **2.38x faster**)    | `205.23 ns` (✅ **1.02x slower**)   | `1.27 us` (❌ *6.32x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.26 s` (✅ **1.00x**)  | `6.63 s` (❌ *2.94x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.14 us` (✅ **1.00x**) | `36.18 us` (❌ *1.63x slower*)   | `122.78 us` (❌ *5.54x slower*)    |
| **`legendre_for_qr`**    | `12.50 us` (✅ **1.00x**) | `35.85 us` (❌ *2.87x slower*)   | `35.95 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.47 ns` (✅ **1.00x**) | `107.96 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `60.55 ns` (✅ **1.00x**) | `107.91 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `3.96 ns` (✅ **1.00x**)  | `4.21 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.02x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.09 ns` (✅ **1.00x**) | `78.81 ns` (❌ *2.18x slower*)    |
| **`into_bigint`** | `22.13 ns` (✅ **1.00x**) | `36.52 ns` (❌ *1.65x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

