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
|        | `194.97 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.43x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `4.81 us` (❌ *3.85x slower*)   | `23.24 ns` (🚀 **53.75x faster**) | `178.70 ns` (🚀 **6.99x faster**)  | `12.52 ns` (🚀 **99.74x faster**)  | `8.71 ns` (🚀 **143.37x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.30 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.74x slower*)   | `23.28 ns` (🚀 **55.75x faster**) | `161.91 ns` (🚀 **8.02x faster**)  | `12.75 ns` (🚀 **101.81x faster**) | `8.82 ns` (🚀 **147.22x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `906.25 ns` (✅ **1.00x**) | `3.43 us` (❌ *3.79x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `941.49 ns` (✅ **1.00x**) | `3.47 us` (❌ *3.68x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `591.10 ns` (✅ **1.00x**) | `2.27 us` (❌ *3.84x slower*)   | `12.28 ns` (🚀 **48.14x faster**) | `67.06 ns` (🚀 **8.81x faster**)   | `7.14 ns` (🚀 **82.73x faster**)   | `5.92 ns` (🚀 **99.79x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `325.46 us` (✅ **1.00x**) | `1.17 ms` (❌ *3.58x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.13 ns` (❌ *3.59x slower*)    | `94.30 ns` (❌ *15.29x slower*)    | `18.36 ns` (❌ *2.98x slower*)     | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.48 ns` (❌ *6.31x slower*)   | `7.12 us` (❌ *165.33x slower*)    | `76.26 ns` (❌ *1.77x slower*)     | `43.04 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `243.24 ns` (❌ *6.67x slower*)   | `5.01 us` (❌ *137.38x slower*)    | `66.41 ns` (❌ *1.82x slower*)     | `36.47 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.17 us` (❌ *2.15x slower*)    | `27.45 us` (❌ *3.88x slower*)     | `14.81 us` (❌ *2.09x slower*)     | `7.07 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `587.04 ns` (❌ *9.53x slower*)   | `14.56 us` (❌ *236.37x slower*)   | `117.68 ns` (❌ *1.91x slower*)    | `61.60 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `577.24 ns` (❌ *6.47x slower*)   | `14.47 us` (❌ *162.13x slower*)   | `162.69 ns` (❌ *1.82x slower*)    | `89.22 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.63 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.72 ns` (✅ **1.00x**) | `10.32 ns` (❌ *1.18x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `158.15 ns` (✅ **1.00x**) | `219.49 ns` (❌ *1.39x slower*)   | `30.70 ns` (🚀 **5.15x faster**)    | `56.70 ns` (🚀 **2.79x faster**)    | `110.16 ns` (✅ **1.44x faster**)    | `697.47 ns` (❌ *4.41x slower*)    |
| **`serialize_uncompressed`**             | `214.58 ns` (✅ **1.00x**) | `332.11 ns` (❌ *1.55x slower*)   | `31.11 ns` (🚀 **6.90x faster**)    | `55.77 ns` (🚀 **3.85x faster**)    | `110.11 ns` (🚀 **1.95x faster**)    | `698.64 ns` (❌ *3.26x slower*)    |
| **`deserialize_compressed`**             | `315.07 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.37x slower*)     | `52.33 ns` (🚀 **6020.52x faster**) | `92.61 ns` (🚀 **3402.21x faster**) | `209.86 ns` (🚀 **1501.34x faster**) | `1.28 us` (🚀 **245.98x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.27 us` (✅ **1.00x**)  | `184.46 us` (❌ *2.70x slower*)   | `52.37 ns` (🚀 **1303.55x faster**) | `92.61 ns` (🚀 **737.14x faster**)  | `209.74 ns` (🚀 **325.48x faster**)  | `1.28 us` (🚀 **53.37x faster**)   |
| **`deserialize_uncompressed`**           | `247.33 us` (✅ **1.00x**) | `875.31 us` (❌ *3.54x slower*)   | `52.23 ns` (🚀 **4735.85x faster**) | `92.55 ns` (🚀 **2672.33x faster**) | `209.43 ns` (🚀 **1181.00x faster**) | `1.28 us` (🚀 **193.12x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `222.88 ns` (✅ **1.00x**) | `469.60 ns` (❌ *2.11x slower*)   | `52.22 ns` (🚀 **4.27x faster**)    | `92.59 ns` (🚀 **2.41x faster**)    | `209.46 ns` (✅ **1.06x faster**)    | `1.27 us` (❌ *5.69x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.38 s` (✅ **1.00x**)  | `8.30 s` (❌ *3.50x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.13 us` (✅ **1.00x**) | `67.95 us` (❌ *2.18x slower*)   | `183.10 us` (❌ *5.88x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `32.02 us` (❌ *2.93x slower*)   | `32.07 us` (❌ *2.93x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.10 ns` (✅ **1.00x**) | `89.32 ns` (❌ *1.86x slower*)    |
| **`from_big-endian_bits`**    | `48.04 ns` (✅ **1.00x**) | `89.21 ns` (❌ *1.86x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.68 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.83 ns` (✅ **1.00x**) | `75.16 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `22.78 ns` (✅ **1.00x**) | `47.11 ns` (❌ *2.07x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

