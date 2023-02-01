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
|        | `204.29 us` (✅ **1.00x**)        | `1.81 ms` (❌ *8.85x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.23 us` (✅ **1.00x**)   | `3.97 us` (❌ *3.23x slower*)     | `23.34 ns` (🚀 **52.65x faster**) | `196.52 ns` (🚀 **6.25x faster**)  | `12.68 ns` (🚀 **96.93x faster**) | `8.67 ns` (🚀 **141.77x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `4.02 us` (❌ *3.15x slower*)     | `23.31 ns` (🚀 **54.70x faster**) | `161.76 ns` (🚀 **7.88x faster**)  | `13.08 ns` (🚀 **97.47x faster**) | `8.77 ns` (🚀 **145.34x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `883.29 ns` (✅ **1.00x**) | `2.84 us` (❌ *3.22x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `908.28 ns` (✅ **1.00x**) | `2.88 us` (❌ *3.17x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `601.25 ns` (✅ **1.00x**) | `1.81 us` (❌ *3.01x slower*)     | `12.69 ns` (🚀 **47.37x faster**) | `72.17 ns` (🚀 **8.33x faster**)   | `7.24 ns` (🚀 **83.05x faster**)  | `5.92 ns` (🚀 **101.56x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `326.31 us` (✅ **1.00x**) | `967.24 us` (❌ *2.96x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.19 ns` (❌ *3.60x slower*)    | `95.65 ns` (❌ *15.54x slower*)    | `18.45 ns` (❌ *3.00x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `239.62 ns` (❌ *5.20x slower*)   | `6.17 us` (❌ *133.76x slower*)    | `77.53 ns` (❌ *1.68x slower*)    | `46.11 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `179.83 ns` (❌ *4.79x slower*)   | `4.34 us` (❌ *115.48x slower*)    | `65.95 ns` (❌ *1.76x slower*)    | `37.54 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.29 us` (❌ *2.07x slower*)    | `25.46 us` (❌ *3.45x slower*)     | `14.95 us` (❌ *2.03x slower*)    | `7.38 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.39 ns` (❌ *6.08x slower*)   | `12.65 us` (❌ *147.96x slower*)   | `116.66 ns` (❌ *1.36x slower*)   | `85.49 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.45 ns` (❌ *5.80x slower*)   | `12.56 us` (❌ *142.92x slower*)   | `164.11 ns` (❌ *1.87x slower*)   | `87.86 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.60 ns` (✅ **1.00x**) | `8.66 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.67 ns` (✅ **1.00x**) | `10.47 ns` (❌ *1.21x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.37 ns` (✅ **1.00x**) | `202.31 ns` (❌ *1.34x slower*)   | `32.20 ns` (🚀 **4.70x faster**)    | `56.78 ns` (🚀 **2.67x faster**)    | `109.29 ns` (✅ **1.39x faster**)   | `701.03 ns` (❌ *4.63x slower*)    |
| **`serialize_uncompressed`**             | `190.97 ns` (✅ **1.00x**) | `282.80 ns` (❌ *1.48x slower*)   | `32.49 ns` (🚀 **5.88x faster**)    | `55.68 ns` (🚀 **3.43x faster**)    | `109.25 ns` (✅ **1.75x faster**)   | `699.18 ns` (❌ *3.66x slower*)    |
| **`deserialize_compressed`**             | `133.17 us` (✅ **1.00x**) | `266.88 us` (❌ *2.00x slower*)   | `51.90 ns` (🚀 **2566.09x faster**) | `94.77 ns` (🚀 **1405.26x faster**) | `215.26 ns` (🚀 **618.67x faster**) | `1.35 us` (🚀 **98.68x faster**)   |
| **`deserialize_compressed_unchecked`**   | `39.38 us` (✅ **1.00x**)  | `133.94 us` (❌ *3.40x slower*)   | `51.89 ns` (🚀 **758.89x faster**)  | `94.86 ns` (🚀 **415.13x faster**)  | `214.70 ns` (🚀 **183.41x faster**) | `1.35 us` (🚀 **29.08x faster**)   |
| **`deserialize_uncompressed`**           | `93.77 us` (✅ **1.00x**)  | `132.67 us` (❌ *1.41x slower*)   | `51.75 ns` (🚀 **1812.04x faster**) | `94.84 ns` (🚀 **988.68x faster**)  | `215.19 ns` (🚀 **435.73x faster**) | `1.35 us` (🚀 **69.32x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `192.54 ns` (✅ **1.00x**) | `401.96 ns` (❌ *2.09x slower*)   | `51.80 ns` (🚀 **3.72x faster**)    | `94.77 ns` (🚀 **2.03x faster**)    | `214.78 ns` (❌ *1.12x slower*)     | `1.35 us` (❌ *7.01x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `7.00 s` (❌ *3.02x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.31 us` (✅ **1.00x**) | `38.85 us` (❌ *1.54x slower*)   | `132.95 us` (❌ *5.25x slower*)    |
| **`legendre_for_qr`**    | `14.39 us` (✅ **1.00x**) | `38.63 us` (❌ *2.68x slower*)   | `38.79 us` (❌ *2.69x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.45 ns` (✅ **1.00x**) | `90.36 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `49.28 ns` (✅ **1.00x**) | `90.35 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)  | `5.08 ns` (✅ **1.04x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.05 ns` (✅ **1.00x**) | `75.93 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.46 ns` (✅ **1.00x**) | `48.10 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

