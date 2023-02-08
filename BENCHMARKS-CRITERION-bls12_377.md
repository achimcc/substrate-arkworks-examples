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
|        | `213.47 us` (✅ **1.00x**)        | `2.21 ms` (❌ *10.35x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.37 us` (✅ **1.00x**)   | `5.29 us` (❌ *3.86x slower*)   | `31.44 ns` (🚀 **43.60x faster**)  | `211.13 ns` (🚀 **6.49x faster**)  | `22.70 ns` (🚀 **60.38x faster**) | `9.85 ns` (🚀 **139.22x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.41 us` (✅ **1.00x**)   | `5.65 us` (❌ *4.01x slower*)   | `32.45 ns` (🚀 **43.43x faster**)  | `203.87 ns` (🚀 **6.91x faster**)  | `17.77 ns` (🚀 **79.30x faster**) | `10.53 ns` (🚀 **133.80x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `1.00 us` (✅ **1.00x**)   | `3.78 us` (❌ *3.77x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.03 us` (✅ **1.00x**)   | `3.83 us` (❌ *3.73x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `697.00 ns` (✅ **1.00x**) | `2.48 us` (❌ *3.56x slower*)   | `15.23 ns` (🚀 **45.78x faster**)  | `118.87 ns` (🚀 **5.86x faster**)  | `8.93 ns` (🚀 **78.08x faster**)  | `10.90 ns` (🚀 **63.94x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `341.21 us` (✅ **1.00x**) | `1.27 ms` (❌ *3.72x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `27.38 ns` (❌ *3.89x slower*)     | `120.83 ns` (❌ *17.16x slower*)   | `19.90 ns` (❌ *2.83x slower*)    | `7.04 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `317.44 ns` (❌ *7.11x slower*)    | `7.91 us` (❌ *177.36x slower*)    | `82.84 ns` (❌ *1.86x slower*)    | `44.62 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `293.74 ns` (❌ *7.61x slower*)    | `5.54 us` (❌ *143.61x slower*)    | `71.27 ns` (❌ *1.85x slower*)    | `38.60 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.65 us` (❌ *2.20x slower*)     | `30.14 us` (❌ *3.97x slower*)     | `17.44 us` (❌ *2.30x slower*)    | `7.58 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `673.60 ns` (❌ *10.70x slower*)   | `16.12 us` (❌ *256.14x slower*)   | `133.36 ns` (❌ *2.12x slower*)   | `62.93 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `657.64 ns` (❌ *6.96x slower*)    | `16.07 us` (❌ *170.22x slower*)   | `190.06 ns` (❌ *2.01x slower*)   | `94.43 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.75 ns` (✅ **1.00x**) | `9.34 ns` (❌ *1.21x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.43 ns` (✅ **1.00x**) | `12.84 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.56 ns` (✅ **1.00x**) | `4.80 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.47 ns` (✅ **1.00x**) | `4.48 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `177.41 ns` (✅ **1.00x**) | `254.88 ns` (❌ *1.44x slower*)   | `33.29 ns` (🚀 **5.33x faster**)    | `59.96 ns` (🚀 **2.96x faster**)     | `118.26 ns` (✅ **1.50x faster**)    | `745.63 ns` (❌ *4.20x slower*)    |
| **`serialize_uncompressed`**             | `236.27 ns` (✅ **1.00x**) | `400.23 ns` (❌ *1.69x slower*)   | `33.32 ns` (🚀 **7.09x faster**)    | `60.00 ns` (🚀 **3.94x faster**)     | `117.75 ns` (🚀 **2.01x faster**)    | `750.00 ns` (❌ *3.17x slower*)    |
| **`deserialize_compressed`**             | `337.46 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.44x slower*)     | `55.13 ns` (🚀 **6120.70x faster**) | `112.04 ns` (🚀 **3012.05x faster**) | `245.31 ns` (🚀 **1375.64x faster**) | `1.48 us` (🚀 **227.62x faster**)  |
| **`deserialize_compressed_unchecked`**   | `78.06 us` (✅ **1.00x**)  | `217.48 us` (❌ *2.79x slower*)   | `55.39 ns` (🚀 **1409.09x faster**) | `115.80 ns` (🚀 **674.07x faster**)  | `246.67 ns` (🚀 **316.44x faster**)  | `1.49 us` (🚀 **52.44x faster**)   |
| **`deserialize_uncompressed`**           | `258.41 us` (✅ **1.00x**) | `999.55 us` (❌ *3.87x slower*)   | `55.18 ns` (🚀 **4683.32x faster**) | `112.23 ns` (🚀 **2302.38x faster**) | `245.08 ns` (🚀 **1054.38x faster**) | `1.48 us` (🚀 **174.86x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `270.75 ns` (✅ **1.00x**) | `562.40 ns` (❌ *2.08x slower*)   | `55.63 ns` (🚀 **4.87x faster**)    | `113.27 ns` (🚀 **2.39x faster**)    | `245.52 ns` (✅ **1.10x faster**)    | `1.48 us` (❌ *5.46x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.70 s` (✅ **1.00x**)  | `9.57 s` (❌ *3.55x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `32.89 us` (✅ **1.00x**) | `77.37 us` (❌ *2.35x slower*)   | `206.57 us` (❌ *6.28x slower*)    |
| **`legendre_for_qr`**    | `11.35 us` (✅ **1.00x**) | `35.04 us` (❌ *3.09x slower*)   | `35.24 us` (❌ *3.11x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.75 ns` (✅ **1.00x**)  | `5.01 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `72.83 ns` (✅ **1.00x**) | `140.03 ns` (❌ *1.92x slower*)    |
| **`from_big-endian_bits`**    | `72.91 ns` (✅ **1.00x**) | `132.43 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.85 ns` (✅ **1.00x**)  | `5.35 ns` (✅ **1.10x slower**)    |
| **`equality`**                | `5.39 ns` (✅ **1.00x**)  | `5.51 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `4.69 ns` (✅ **1.00x**)  | `4.73 ns` (✅ **1.01x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.00 ns` (✅ **1.00x**) | `96.68 ns` (❌ *2.25x slower*)    |
| **`into_bigint`** | `27.47 ns` (✅ **1.00x**) | `48.93 ns` (❌ *1.78x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

