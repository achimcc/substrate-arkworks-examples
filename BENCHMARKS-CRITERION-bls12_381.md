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
|        | `180.56 us` (✅ **1.00x**)        | `1.63 ms` (❌ *9.00x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.13 us` (✅ **1.00x**)   | `3.64 us` (❌ *3.24x slower*)     | `28.17 ns` (🚀 **39.94x faster**) | `180.15 ns` (🚀 **6.25x faster**)  | `19.25 ns` (🚀 **58.44x faster**) | `8.19 ns` (🚀 **137.38x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.69 us` (❌ *3.18x slower*)     | `27.30 ns` (🚀 **42.52x faster**) | `170.36 ns` (🚀 **6.81x faster**)  | `14.90 ns` (🚀 **77.93x faster**) | `8.58 ns` (🚀 **135.28x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `820.95 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.18x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `858.48 ns` (✅ **1.00x**) | `2.65 us` (❌ *3.09x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `565.71 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.90x slower*)     | `13.10 ns` (🚀 **43.19x faster**) | `100.29 ns` (🚀 **5.64x faster**)  | `7.61 ns` (🚀 **74.34x faster**)  | `5.49 ns` (🚀 **103.09x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.73 us` (✅ **1.00x**) | `869.56 us` (❌ *3.05x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.77 ns` (❌ *3.93x slower*)    | `101.66 ns` (❌ *17.55x slower*)   | `16.76 ns` (❌ *2.89x slower*)    | `5.79 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `224.19 ns` (❌ *5.78x slower*)   | `5.75 us` (❌ *148.04x slower*)    | `70.53 ns` (❌ *1.82x slower*)    | `38.81 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `176.39 ns` (❌ *4.96x slower*)   | `4.05 us` (❌ *113.80x slower*)    | `58.10 ns` (❌ *1.63x slower*)    | `35.56 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `14.09 us` (❌ *2.18x slower*)    | `23.28 us` (❌ *3.60x slower*)     | `13.80 us` (❌ *2.13x slower*)    | `6.47 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.23 ns` (❌ *6.01x slower*)   | `11.77 us` (❌ *143.52x slower*)   | `106.90 ns` (❌ *1.30x slower*)   | `82.01 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `474.13 ns` (❌ *5.88x slower*)   | `11.69 us` (❌ *145.03x slower*)   | `156.64 ns` (❌ *1.94x slower*)   | `80.62 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.91 ns` (❌ *1.21x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.57 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `141.61 ns` (✅ **1.00x**) | `192.17 ns` (❌ *1.36x slower*)   | `30.16 ns` (🚀 **4.69x faster**)    | `49.59 ns` (🚀 **2.86x faster**)    | `97.97 ns` (✅ **1.45x faster**)    | `632.18 ns` (❌ *4.46x slower*)    |
| **`serialize_uncompressed`**             | `179.52 ns` (✅ **1.00x**) | `268.93 ns` (❌ *1.50x slower*)   | `30.06 ns` (🚀 **5.97x faster**)    | `49.62 ns` (🚀 **3.62x faster**)    | `97.99 ns` (🚀 **1.83x faster**)    | `628.94 ns` (❌ *3.50x slower*)    |
| **`deserialize_compressed`**             | `118.20 us` (✅ **1.00x**) | `241.89 us` (❌ *2.05x slower*)   | `46.57 ns` (🚀 **2538.00x faster**) | `95.05 ns` (🚀 **1243.51x faster**) | `207.49 ns` (🚀 **569.66x faster**) | `1.26 us` (🚀 **93.69x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.11 us` (✅ **1.00x**)  | `123.26 us` (❌ *3.41x slower*)   | `46.56 ns` (🚀 **775.51x faster**)  | `94.69 ns` (🚀 **381.35x faster**)  | `207.50 ns` (🚀 **174.03x faster**) | `1.26 us` (🚀 **28.63x faster**)   |
| **`deserialize_uncompressed`**           | `82.15 us` (✅ **1.00x**)  | `118.19 us` (❌ *1.44x slower*)   | `46.50 ns` (🚀 **1766.49x faster**) | `94.69 ns` (🚀 **867.54x faster**)  | `207.64 ns` (🚀 **395.63x faster**) | `1.26 us` (🚀 **65.10x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `201.04 ns` (✅ **1.00x**) | `417.28 ns` (❌ *2.08x slower*)   | `46.52 ns` (🚀 **4.32x faster**)    | `94.69 ns` (🚀 **2.12x faster**)    | `208.84 ns` (✅ **1.04x slower**)   | `1.26 us` (❌ *6.28x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.28 s` (✅ **1.00x**)  | `6.70 s` (❌ *2.93x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.23 us` (✅ **1.00x**) | `35.70 us` (❌ *1.61x slower*)   | `122.36 us` (❌ *5.50x slower*)    |
| **`legendre_for_qr`**    | `12.34 us` (✅ **1.00x**) | `35.96 us` (❌ *2.91x slower*)   | `36.10 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.75 ns` (✅ **1.00x**) | `109.95 ns` (❌ *1.81x slower*)    |
| **`from_big-endian_bits`**    | `60.69 ns` (✅ **1.00x**) | `110.00 ns` (❌ *1.81x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.74 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.02 ns` (✅ **1.00x**) | `78.84 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.68 ns` (✅ **1.00x**) | `41.43 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

