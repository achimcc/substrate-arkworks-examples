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
|        | `208.53 us` (✅ **1.00x**)        | `2.17 ms` (❌ *10.40x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.34 us` (✅ **1.00x**)   | `5.29 us` (❌ *3.95x slower*)   | `32.89 ns` (🚀 **40.67x faster**)  | `215.16 ns` (🚀 **6.22x faster**)  | `19.45 ns` (🚀 **68.78x faster**) | `8.71 ns` (🚀 **153.57x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.39 us` (✅ **1.00x**)   | `5.27 us` (❌ *3.80x slower*)   | `33.42 ns` (🚀 **41.47x faster**)  | `198.99 ns` (🚀 **6.96x faster**)  | `15.57 ns` (🚀 **89.02x faster**) | `10.04 ns` (🚀 **137.96x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `979.67 ns` (✅ **1.00x**) | `3.75 us` (❌ *3.83x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.02 us` (✅ **1.00x**)   | `3.79 us` (❌ *3.71x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `670.14 ns` (✅ **1.00x**) | `2.44 us` (❌ *3.65x slower*)   | `15.22 ns` (🚀 **44.02x faster**)  | `119.56 ns` (🚀 **5.60x faster**)  | `7.68 ns` (🚀 **87.29x faster**)  | `9.45 ns` (🚀 **70.89x faster**)    |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `345.77 us` (✅ **1.00x**) | `1.26 ms` (❌ *3.65x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `27.66 ns` (❌ *4.55x slower*)     | `145.60 ns` (❌ *23.98x slower*)   | `19.12 ns` (❌ *3.15x slower*)    | `6.07 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `312.77 ns` (❌ *8.04x slower*)    | `8.05 us` (❌ *206.95x slower*)    | `70.45 ns` (❌ *1.81x slower*)    | `38.91 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `291.04 ns` (❌ *8.70x slower*)    | `5.48 us` (❌ *163.74x slower*)    | `60.36 ns` (❌ *1.81x slower*)    | `33.44 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.19 us` (❌ *2.52x slower*)     | `29.57 us` (❌ *4.60x slower*)     | `13.80 us` (❌ *2.15x slower*)    | `6.43 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `670.25 ns` (❌ *12.40x slower*)   | `15.87 us` (❌ *293.58x slower*)   | `112.75 ns` (❌ *2.09x slower*)   | `54.05 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `649.49 ns` (❌ *7.92x slower*)    | `15.94 us` (❌ *194.28x slower*)   | `159.05 ns` (❌ *1.94x slower*)   | `82.04 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `6.71 ns` (✅ **1.00x**) | `8.85 ns` (❌ *1.32x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `8.30 ns` (✅ **1.00x**) | `11.27 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `3.97 ns` (✅ **1.00x**) | `4.18 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.32 ns` (✅ **1.00x**) | `3.85 ns` (✅ **1.12x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `173.68 ns` (✅ **1.00x**) | `250.73 ns` (❌ *1.44x slower*)   | `28.50 ns` (🚀 **6.09x faster**)    | `56.34 ns` (🚀 **3.08x faster**)    | `116.16 ns` (✅ **1.50x faster**)    | `748.73 ns` (❌ *4.31x slower*)    |
| **`serialize_uncompressed`**             | `232.90 ns` (✅ **1.00x**) | `383.21 ns` (❌ *1.65x slower*)   | `28.63 ns` (🚀 **8.14x faster**)    | `51.46 ns` (🚀 **4.53x faster**)    | `120.01 ns` (🚀 **1.94x faster**)    | `737.04 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `329.06 us` (✅ **1.00x**) | `1.14 ms` (❌ *3.48x slower*)     | `49.34 ns` (🚀 **6669.02x faster**) | `97.76 ns` (🚀 **3365.97x faster**) | `243.79 ns` (🚀 **1349.75x faster**) | `1.49 us` (🚀 **220.92x faster**)  |
| **`deserialize_compressed_unchecked`**   | `77.64 us` (✅ **1.00x**)  | `203.02 us` (❌ *2.61x slower*)   | `47.70 ns` (🚀 **1627.62x faster**) | `95.78 ns` (🚀 **810.59x faster**)  | `247.86 ns` (🚀 **313.24x faster**)  | `1.46 us` (🚀 **53.36x faster**)   |
| **`deserialize_uncompressed`**           | `253.32 us` (✅ **1.00x**) | `941.18 us` (❌ *3.72x slower*)   | `47.60 ns` (🚀 **5321.65x faster**) | `97.50 ns` (🚀 **2598.15x faster**) | `243.58 ns` (🚀 **1039.97x faster**) | `1.47 us` (🚀 **171.96x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `265.52 ns` (✅ **1.00x**) | `566.92 ns` (❌ *2.14x slower*)   | `48.93 ns` (🚀 **5.43x faster**)    | `94.78 ns` (🚀 **2.80x faster**)    | `245.57 ns` (✅ **1.08x faster**)    | `1.46 us` (❌ *5.49x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.67 s` (✅ **1.00x**)  | `8.36 s` (❌ *3.13x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `28.16 us` (✅ **1.00x**) | `68.02 us` (❌ *2.42x slower*)   | `205.52 us` (❌ *7.30x slower*)    |
| **`legendre_for_qr`**    | `9.77 us` (✅ **1.00x**)  | `30.13 us` (❌ *3.08x slower*)   | `34.47 us` (❌ *3.53x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.06 ns` (✅ **1.00x**)  | `4.33 ns` (✅ **1.07x slower**)    |
| **`from_little-endian_bits`** | `62.18 ns` (✅ **1.00x**) | `114.95 ns` (❌ *1.85x slower*)    |
| **`from_big-endian_bits`**    | `62.19 ns` (✅ **1.00x**) | `112.08 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.33 ns` (✅ **1.00x**)  | `4.42 ns` (✅ **1.02x slower**)    |
| **`equality`**                | `4.59 ns` (✅ **1.00x**)  | `4.80 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.99 ns` (✅ **1.00x**)  | `4.23 ns` (✅ **1.06x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.49 ns` (✅ **1.00x**) | `81.10 ns` (❌ *2.22x slower*)    |
| **`into_bigint`** | `22.28 ns` (✅ **1.00x**) | `48.03 ns` (❌ *2.16x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

