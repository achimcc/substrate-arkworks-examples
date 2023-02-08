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
|        | `292.16 us` (✅ **1.00x**)        | `2.37 ms` (❌ *8.11x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.80 us` (✅ **1.00x**)   | `5.08 us` (❌ *2.82x slower*)   | `37.39 ns` (🚀 **48.20x faster**) | `227.74 ns` (🚀 **7.91x faster**)  | `24.80 ns` (🚀 **72.68x faster**) | `11.05 ns` (🚀 **163.13x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.87 us` (✅ **1.00x**)   | `5.17 us` (❌ *2.76x slower*)   | `35.65 ns` (🚀 **52.49x faster**) | `217.39 ns` (🚀 **8.61x faster**)  | `19.64 ns` (🚀 **95.27x faster**) | `11.29 ns` (🚀 **165.73x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.32 us` (✅ **1.00x**)   | `3.64 us` (❌ *2.75x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.33 us` (✅ **1.00x**)   | `3.73 us` (❌ *2.81x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `880.90 ns` (✅ **1.00x**) | `2.50 us` (❌ *2.84x slower*)   | `17.58 ns` (🚀 **50.11x faster**) | `132.16 ns` (🚀 **6.67x faster**)  | `9.83 ns` (🚀 **89.61x faster**)  | `6.53 ns` (🚀 **134.99x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `456.11 us` (✅ **1.00x**) | `1.25 ms` (❌ *2.73x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `27.92 ns` (❌ *3.24x slower*)    | `133.24 ns` (❌ *15.46x slower*)   | `21.70 ns` (❌ *2.52x slower*)    | `8.62 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `293.51 ns` (❌ *5.87x slower*)   | `7.36 us` (❌ *147.24x slower*)    | `90.09 ns` (❌ *1.80x slower*)    | `49.97 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `264.48 ns` (❌ *6.02x slower*)   | `5.28 us` (❌ *120.29x slower*)    | `77.01 ns` (❌ *1.75x slower*)    | `43.92 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.59 us` (❌ *2.29x slower*)    | `29.42 us` (❌ *4.06x slower*)     | `16.13 us` (❌ *2.23x slower*)    | `7.24 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `633.47 ns` (❌ *6.06x slower*)   | `14.84 us` (❌ *142.08x slower*)   | `139.02 ns` (❌ *1.33x slower*)   | `104.47 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `612.55 ns` (❌ *5.88x slower*)   | `14.98 us` (❌ *143.75x slower*)   | `203.51 ns` (❌ *1.95x slower*)   | `104.24 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.98 ns` (✅ **1.00x**)  | `12.64 ns` (❌ *1.58x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.52 ns` (✅ **1.00x**) | `14.90 ns` (❌ *1.42x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.54 ns` (✅ **1.00x**)  | `4.81 ns` (✅ **1.06x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.24 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `192.37 ns` (✅ **1.00x**) | `273.61 ns` (❌ *1.42x slower*)   | `38.20 ns` (🚀 **5.04x faster**)    | `61.36 ns` (🚀 **3.14x faster**)     | `120.01 ns` (✅ **1.60x faster**)   | `803.90 ns` (❌ *4.18x slower*)    |
| **`serialize_uncompressed`**             | `257.53 ns` (✅ **1.00x**) | `388.43 ns` (❌ *1.51x slower*)   | `38.88 ns` (🚀 **6.62x faster**)    | `61.39 ns` (🚀 **4.20x faster**)     | `119.14 ns` (🚀 **2.16x faster**)   | `770.99 ns` (❌ *2.99x slower*)    |
| **`deserialize_compressed`**             | `192.64 us` (✅ **1.00x**) | `373.75 us` (❌ *1.94x slower*)   | `64.69 ns` (🚀 **2977.88x faster**) | `122.20 ns` (🚀 **1576.41x faster**) | `285.72 ns` (🚀 **674.23x faster**) | `1.80 us` (🚀 **107.14x faster**)  |
| **`deserialize_compressed_unchecked`**   | `60.30 us` (✅ **1.00x**)  | `199.67 us` (❌ *3.31x slower*)   | `61.42 ns` (🚀 **981.71x faster**)  | `118.98 ns` (🚀 **506.79x faster**)  | `294.02 ns` (🚀 **205.08x faster**) | `1.80 us` (🚀 **33.55x faster**)   |
| **`deserialize_uncompressed`**           | `127.94 us` (✅ **1.00x**) | `174.41 us` (❌ *1.36x slower*)   | `61.19 ns` (🚀 **2090.83x faster**) | `119.92 ns` (🚀 **1066.88x faster**) | `288.70 ns` (🚀 **443.17x faster**) | `1.94 us` (🚀 **66.05x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `286.20 ns` (✅ **1.00x**) | `569.93 ns` (❌ *1.99x slower*)   | `61.91 ns` (🚀 **4.62x faster**)    | `119.86 ns` (🚀 **2.39x faster**)    | `299.23 ns` (✅ **1.05x slower**)   | `1.84 us` (❌ *6.44x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `3.56 s` (✅ **1.00x**)  | `9.28 s` (❌ *2.61x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `28.00 us` (✅ **1.00x**) | `57.99 us` (❌ *2.07x slower*)   | `195.81 us` (❌ *6.99x slower*)    |
| **`legendre_for_qr`**    | `15.90 us` (✅ **1.00x**) | `58.11 us` (❌ *3.65x slower*)   | `59.18 us` (❌ *3.72x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.78 ns` (✅ **1.00x**)  | `4.83 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `75.24 ns` (✅ **1.00x**) | `129.60 ns` (❌ *1.72x slower*)    |
| **`from_big-endian_bits`**    | `74.63 ns` (✅ **1.00x**) | `132.23 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.63 ns` (✅ **1.00x**)  | `5.11 ns` (✅ **1.10x slower**)    |
| **`equality`**                | `5.13 ns` (✅ **1.00x**)  | `5.85 ns` (❌ *1.14x slower*)      |
| **`is_zero`**                 | `4.43 ns` (✅ **1.00x**)  | `4.74 ns` (✅ **1.07x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `45.41 ns` (✅ **1.00x**) | `112.53 ns` (❌ *2.48x slower*)    |
| **`into_bigint`** | `28.65 ns` (✅ **1.00x**) | `51.38 ns` (❌ *1.79x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

