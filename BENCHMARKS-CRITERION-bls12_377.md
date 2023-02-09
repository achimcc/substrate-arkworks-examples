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
|        | `193.29 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.49x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.24 us` (✅ **1.00x**)   | `4.80 us` (❌ *3.86x slower*)   | `23.16 ns` (🚀 **53.60x faster**) | `179.51 ns` (🚀 **6.91x faster**)  | `12.51 ns` (🚀 **99.26x faster**)  | `8.70 ns` (🚀 **142.66x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.28 us` (✅ **1.00x**)   | `4.86 us` (❌ *3.80x slower*)   | `23.22 ns` (🚀 **55.14x faster**) | `158.83 ns` (🚀 **8.06x faster**)  | `12.72 ns` (🚀 **100.69x faster**) | `8.81 ns` (🚀 **145.33x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `895.91 ns` (✅ **1.00x**) | `3.42 us` (❌ *3.82x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `916.44 ns` (✅ **1.00x**) | `3.46 us` (❌ *3.77x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `606.75 ns` (✅ **1.00x**) | `2.26 us` (❌ *3.73x slower*)   | `12.34 ns` (🚀 **49.16x faster**) | `67.26 ns` (🚀 **9.02x faster**)   | `7.14 ns` (🚀 **84.94x faster**)   | `5.87 ns` (🚀 **103.39x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `316.31 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.66x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.13 ns` (❌ *3.61x slower*)    | `98.84 ns` (❌ *16.11x slower*)    | `18.32 ns` (❌ *2.99x slower*)     | `6.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.27 ns` (❌ *6.35x slower*)   | `7.11 us` (❌ *166.42x slower*)    | `74.54 ns` (❌ *1.74x slower*)     | `42.71 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `250.09 ns` (❌ *6.88x slower*)   | `5.01 us` (❌ *137.66x slower*)    | `66.49 ns` (❌ *1.83x slower*)     | `36.36 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.16 us` (❌ *2.15x slower*)    | `27.36 us` (❌ *3.88x slower*)     | `14.78 us` (❌ *2.10x slower*)     | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `588.10 ns` (❌ *9.58x slower*)   | `14.50 us` (❌ *236.32x slower*)   | `117.86 ns` (❌ *1.92x slower*)    | `61.37 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `575.84 ns` (❌ *6.46x slower*)   | `14.42 us` (❌ *161.91x slower*)   | `163.41 ns` (❌ *1.83x slower*)    | `89.08 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.63 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**) | `10.29 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `156.85 ns` (✅ **1.00x**) | `220.63 ns` (❌ *1.41x slower*)   | `32.21 ns` (🚀 **4.87x faster**)    | `58.18 ns` (🚀 **2.70x faster**)    | `109.68 ns` (✅ **1.43x faster**)    | `704.56 ns` (❌ *4.49x slower*)    |
| **`serialize_uncompressed`**             | `211.39 ns` (✅ **1.00x**) | `332.71 ns` (❌ *1.57x slower*)   | `31.30 ns` (🚀 **6.75x faster**)    | `55.95 ns` (🚀 **3.78x faster**)    | `109.64 ns` (🚀 **1.93x faster**)    | `709.30 ns` (❌ *3.36x slower*)    |
| **`deserialize_compressed`**             | `311.38 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.40x slower*)     | `52.57 ns` (🚀 **5923.49x faster**) | `91.82 ns` (🚀 **3391.24x faster**) | `208.82 ns` (🚀 **1491.18x faster**) | `1.28 us` (🚀 **243.07x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.66 us` (✅ **1.00x**)  | `182.77 us` (❌ *2.70x slower*)   | `52.56 ns` (🚀 **1287.26x faster**) | `92.01 ns` (🚀 **735.39x faster**)  | `208.83 ns` (🚀 **324.00x faster**)  | `1.28 us` (🚀 **52.83x faster**)   |
| **`deserialize_uncompressed`**           | `243.72 us` (✅ **1.00x**) | `871.65 us` (❌ *3.58x slower*)   | `52.47 ns` (🚀 **4644.60x faster**) | `92.04 ns` (🚀 **2647.87x faster**) | `208.48 ns` (🚀 **1169.03x faster**) | `1.28 us` (🚀 **190.34x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.63 ns` (✅ **1.00x**) | `470.37 ns` (❌ *2.08x slower*)   | `52.48 ns` (🚀 **4.32x faster**)    | `92.00 ns` (🚀 **2.46x faster**)    | `208.46 ns` (✅ **1.09x faster**)    | `1.28 us` (❌ *5.65x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.34 s` (✅ **1.00x**)  | `8.25 s` (❌ *3.53x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.07 us` (✅ **1.00x**) | `67.31 us` (❌ *2.17x slower*)   | `181.85 us` (❌ *5.85x slower*)    |
| **`legendre_for_qr`**    | `10.90 us` (✅ **1.00x**) | `31.32 us` (❌ *2.87x slower*)   | `31.35 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `50.21 ns` (✅ **1.00x**) | `89.52 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `50.21 ns` (✅ **1.00x**) | `89.47 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.75 ns` (✅ **1.00x**) | `74.94 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `23.76 ns` (✅ **1.00x**) | `47.03 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

