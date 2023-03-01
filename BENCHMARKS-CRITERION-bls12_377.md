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
|        | `176.67 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.46x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `3.83 us` (❌ *3.37x slower*)     | `26.96 ns` (🚀 **42.19x faster**)  | `176.04 ns` (🚀 **6.46x faster**)  | `18.96 ns` (🚀 **60.01x faster**) | `8.27 ns` (🚀 **137.56x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.49 us` (❌ *3.83x slower*)     | `27.37 ns` (🚀 **42.86x faster**)  | `168.93 ns` (🚀 **6.94x faster**)  | `15.02 ns` (🚀 **78.12x faster**) | `8.72 ns` (🚀 **134.47x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `832.90 ns` (✅ **1.00x**) | `3.17 us` (❌ *3.81x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `857.14 ns` (✅ **1.00x**) | `2.77 us` (❌ *3.24x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `567.50 ns` (✅ **1.00x**) | `1.81 us` (❌ *3.19x slower*)     | `12.76 ns` (🚀 **44.46x faster**)  | `101.64 ns` (🚀 **5.58x faster**)  | `7.48 ns` (🚀 **75.87x faster**)  | `8.02 ns` (🚀 **70.77x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.73 us` (✅ **1.00x**) | `928.77 us` (❌ *3.27x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.63 ns` (❌ *4.40x slower*)     | `101.30 ns` (❌ *19.71x slower*)   | `16.76 ns` (❌ *3.26x slower*)    | `5.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `266.70 ns` (❌ *7.15x slower*)    | `6.65 us` (❌ *178.31x slower*)    | `69.72 ns` (❌ *1.87x slower*)    | `37.31 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `248.42 ns` (❌ *8.85x slower*)    | `4.68 us` (❌ *166.92x slower*)    | `58.90 ns` (❌ *2.10x slower*)    | `28.06 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.77 us` (❌ *2.49x slower*)     | `25.10 us` (❌ *4.55x slower*)     | `13.46 us` (❌ *2.44x slower*)    | `5.52 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `571.83 ns` (❌ *12.42x slower*)   | `13.54 us` (❌ *294.01x slower*)   | `111.60 ns` (❌ *2.42x slower*)   | `46.05 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `551.61 ns` (❌ *8.01x slower*)    | `13.46 us` (❌ *195.48x slower*)   | `156.84 ns` (❌ *2.28x slower*)   | `68.87 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `5.73 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.37x slower*)    | `N/A`                     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `6.86 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.55x slower*)   | `N/A`                     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.15 ns` (✅ **1.00x**) | `184.92 ns` (❌ *1.26x slower*)   | `28.11 ns` (🚀 **5.24x faster**)    | `50.30 ns` (🚀 **2.93x faster**)    | `99.66 ns` (✅ **1.48x faster**)     | `626.72 ns` (❌ *4.26x slower*)    |
| **`serialize_uncompressed`**             | `198.22 ns` (✅ **1.00x**) | `318.15 ns` (❌ *1.61x slower*)   | `24.54 ns` (🚀 **8.08x faster**)    | `50.09 ns` (🚀 **3.96x faster**)    | `99.50 ns` (🚀 **1.99x faster**)     | `626.67 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `279.67 us` (✅ **1.00x**) | `966.70 us` (❌ *3.46x slower*)   | `45.00 ns` (🚀 **6214.42x faster**) | `93.12 ns` (🚀 **3003.44x faster**) | `205.77 ns` (🚀 **1359.14x faster**) | `1.25 us` (🚀 **223.53x faster**)  |
| **`deserialize_compressed_unchecked`**   | `64.85 us` (✅ **1.00x**)  | `150.14 us` (❌ *2.32x slower*)   | `45.00 ns` (🚀 **1440.97x faster**) | `93.13 ns` (🚀 **696.35x faster**)  | `205.93 ns` (🚀 **314.91x faster**)  | `1.25 us` (🚀 **51.83x faster**)   |
| **`deserialize_uncompressed`**           | `215.05 us` (✅ **1.00x**) | `792.08 us` (❌ *3.68x slower*)   | `44.93 ns` (🚀 **4786.52x faster**) | `93.10 ns` (🚀 **2309.82x faster**) | `206.07 ns` (🚀 **1043.61x faster**) | `1.25 us` (🚀 **171.83x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `227.97 ns` (✅ **1.00x**) | `418.95 ns` (❌ *1.84x slower*)   | `39.56 ns` (🚀 **5.76x faster**)    | `93.08 ns` (🚀 **2.45x faster**)    | `206.05 ns` (✅ **1.11x faster**)    | `1.25 us` (❌ *5.49x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.14 s` (✅ **1.00x**)  | `7.94 s` (❌ *3.71x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.67 us` (✅ **1.00x**) | `64.33 us` (❌ *2.33x slower*)   | `172.31 us` (❌ *6.23x slower*)    |
| **`legendre_for_qr`**    | `8.37 us` (✅ **1.00x**)  | `29.22 us` (❌ *3.49x slower*)   | `29.42 us` (❌ *3.52x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.74 ns` (✅ **1.00x**) | `110.89 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `60.79 ns` (✅ **1.00x**) | `110.49 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.86 ns` (✅ **1.00x**) | `79.09 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.72 ns` (✅ **1.00x**) | `41.52 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

