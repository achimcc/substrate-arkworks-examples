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
|        | `176.75 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.45x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.44 us` (❌ *3.91x slower*)   | `26.83 ns` (🚀 **42.38x faster**)  | `177.25 ns` (🚀 **6.41x faster**)  | `19.11 ns` (🚀 **59.49x faster**) | `8.27 ns` (🚀 **137.49x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.49 us` (❌ *3.83x slower*)   | `27.05 ns` (🚀 **43.40x faster**)  | `169.45 ns` (🚀 **6.93x faster**)  | `15.19 ns` (🚀 **77.28x faster**) | `8.73 ns` (🚀 **134.42x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `831.72 ns` (✅ **1.00x**) | `3.18 us` (❌ *3.82x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `856.67 ns` (✅ **1.00x**) | `3.22 us` (❌ *3.76x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `566.94 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.66x slower*)   | `12.80 ns` (🚀 **44.28x faster**)  | `101.63 ns` (🚀 **5.58x faster**)  | `7.48 ns` (🚀 **75.84x faster**)  | `9.10 ns` (🚀 **62.32x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.51 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.83 ns` (❌ *3.84x slower*)     | `101.93 ns` (❌ *17.13x slower*)   | `17.30 ns` (❌ *2.91x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `266.80 ns` (❌ *7.15x slower*)    | `6.64 us` (❌ *178.03x slower*)    | `69.93 ns` (❌ *1.88x slower*)    | `37.29 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `249.38 ns` (❌ *7.85x slower*)    | `4.67 us` (❌ *147.03x slower*)    | `58.86 ns` (❌ *1.85x slower*)    | `31.78 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.76 us` (❌ *2.18x slower*)     | `25.05 us` (❌ *3.97x slower*)     | `13.40 us` (❌ *2.12x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `570.54 ns` (❌ *10.74x slower*)   | `13.55 us` (❌ *255.24x slower*)   | `112.24 ns` (❌ *2.11x slower*)   | `53.10 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `554.50 ns` (❌ *6.95x slower*)    | `13.48 us` (❌ *169.00x slower*)   | `156.47 ns` (❌ *1.96x slower*)   | `79.76 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.55 ns` (✅ **1.00x**) | `212.60 ns` (❌ *1.44x slower*)   | `28.08 ns` (🚀 **5.25x faster**)    | `50.33 ns` (🚀 **2.93x faster**)    | `99.94 ns` (✅ **1.48x faster**)     | `626.60 ns` (❌ *4.25x slower*)    |
| **`serialize_uncompressed`**             | `198.63 ns` (✅ **1.00x**) | `319.69 ns` (❌ *1.61x slower*)   | `27.90 ns` (🚀 **7.12x faster**)    | `50.09 ns` (🚀 **3.97x faster**)    | `99.94 ns` (🚀 **1.99x faster**)     | `626.48 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `279.20 us` (✅ **1.00x**) | `966.16 us` (❌ *3.46x slower*)   | `44.93 ns` (🚀 **6214.48x faster**) | `93.22 ns` (🚀 **2995.23x faster**) | `207.45 ns` (🚀 **1345.90x faster**) | `1.25 us` (🚀 **223.13x faster**)  |
| **`deserialize_compressed_unchecked`**   | `64.67 us` (✅ **1.00x**)  | `173.06 us` (❌ *2.68x slower*)   | `44.93 ns` (🚀 **1439.50x faster**) | `93.45 ns` (🚀 **692.04x faster**)  | `207.53 ns` (🚀 **311.63x faster**)  | `1.25 us` (🚀 **51.68x faster**)   |
| **`deserialize_uncompressed`**           | `214.96 us` (✅ **1.00x**) | `791.73 us` (❌ *3.68x slower*)   | `44.89 ns` (🚀 **4789.13x faster**) | `93.48 ns` (🚀 **2299.55x faster**) | `207.38 ns` (🚀 **1036.55x faster**) | `1.25 us` (🚀 **171.75x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.28 ns` (✅ **1.00x**) | `471.18 ns` (❌ *2.09x slower*)   | `44.89 ns` (🚀 **5.02x faster**)    | `93.15 ns` (🚀 **2.42x faster**)    | `207.35 ns` (✅ **1.09x faster**)    | `1.25 us` (❌ *5.56x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.25 s` (✅ **1.00x**)  | `7.89 s` (❌ *3.51x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.63 us` (✅ **1.00x**) | `64.47 us` (❌ *2.33x slower*)   | `172.40 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `9.51 us` (✅ **1.00x**)  | `29.70 us` (❌ *3.12x slower*)   | `29.41 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.69 ns` (✅ **1.00x**) | `107.65 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `60.73 ns` (✅ **1.00x**) | `107.54 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.74 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.85 ns` (✅ **1.00x**) | `79.21 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.75 ns` (✅ **1.00x**) | `41.52 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

