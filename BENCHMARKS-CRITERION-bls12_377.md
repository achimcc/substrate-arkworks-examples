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
|        | `194.87 us` (✅ **1.00x**)        | `2.04 ms` (❌ *10.44x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `4.80 us` (❌ *3.84x slower*)   | `23.30 ns` (🚀 **53.65x faster**) | `179.63 ns` (🚀 **6.96x faster**)  | `12.53 ns` (🚀 **99.77x faster**)  | `8.77 ns` (🚀 **142.51x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.30 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.74x slower*)   | `23.31 ns` (🚀 **55.69x faster**) | `162.24 ns` (🚀 **8.00x faster**)  | `12.75 ns` (🚀 **101.77x faster**) | `8.81 ns` (🚀 **147.35x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `904.56 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.80x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `939.55 ns` (✅ **1.00x**) | `3.47 us` (❌ *3.70x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `590.71 ns` (✅ **1.00x**) | `2.27 us` (❌ *3.84x slower*)   | `12.33 ns` (🚀 **47.90x faster**) | `73.74 ns` (🚀 **8.01x faster**)   | `7.15 ns` (🚀 **82.61x faster**)   | `5.92 ns` (🚀 **99.81x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `325.68 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.57x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.92 ns` (❌ *3.72x slower*)    | `93.73 ns` (❌ *15.20x slower*)    | `18.80 ns` (❌ *3.05x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.01 ns` (❌ *6.30x slower*)   | `7.10 us` (❌ *164.90x slower*)    | `76.31 ns` (❌ *1.77x slower*)     | `43.05 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `243.17 ns` (❌ *6.67x slower*)   | `5.01 us` (❌ *137.34x slower*)    | `66.38 ns` (❌ *1.82x slower*)     | `36.46 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.12 us` (❌ *2.14x slower*)    | `27.46 us` (❌ *3.89x slower*)     | `14.79 us` (❌ *2.09x slower*)     | `7.07 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `587.31 ns` (❌ *9.54x slower*)   | `14.58 us` (❌ *236.83x slower*)   | `117.72 ns` (❌ *1.91x slower*)    | `61.56 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `577.86 ns` (❌ *6.48x slower*)   | `14.48 us` (❌ *162.24x slower*)   | `162.02 ns` (❌ *1.82x slower*)    | `89.24 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.63 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**) | `10.33 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.65 ns` (✅ **1.00x**) | `4.52 ns` (✅ **1.03x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `158.87 ns` (✅ **1.00x**) | `219.41 ns` (❌ *1.38x slower*)   | `30.74 ns` (🚀 **5.17x faster**)    | `56.59 ns` (🚀 **2.81x faster**)    | `110.23 ns` (✅ **1.44x faster**)    | `697.21 ns` (❌ *4.39x slower*)    |
| **`serialize_uncompressed`**             | `217.58 ns` (✅ **1.00x**) | `332.43 ns` (❌ *1.53x slower*)   | `31.17 ns` (🚀 **6.98x faster**)    | `55.62 ns` (🚀 **3.91x faster**)    | `110.43 ns` (🚀 **1.97x faster**)    | `698.95 ns` (❌ *3.21x slower*)    |
| **`deserialize_compressed`**             | `315.10 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.37x slower*)     | `52.53 ns` (🚀 **5998.07x faster**) | `92.65 ns` (🚀 **3400.84x faster**) | `210.60 ns` (🚀 **1496.17x faster**) | `1.28 us` (🚀 **246.68x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.27 us` (✅ **1.00x**)  | `184.48 us` (❌ *2.70x slower*)   | `52.53 ns` (🚀 **1299.70x faster**) | `92.67 ns` (🚀 **736.72x faster**)  | `210.60 ns` (🚀 **324.19x faster**)  | `1.28 us` (🚀 **53.41x faster**)   |
| **`deserialize_uncompressed`**           | `247.16 us` (✅ **1.00x**) | `875.11 us` (❌ *3.54x slower*)   | `52.39 ns` (🚀 **4717.84x faster**) | `92.47 ns` (🚀 **2672.85x faster**) | `209.81 ns` (🚀 **1178.01x faster**) | `1.28 us` (🚀 **192.58x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `222.44 ns` (✅ **1.00x**) | `470.96 ns` (❌ *2.12x slower*)   | `52.39 ns` (🚀 **4.25x faster**)    | `92.64 ns` (🚀 **2.40x faster**)    | `209.83 ns` (✅ **1.06x faster**)    | `1.28 us` (❌ *5.75x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.38 s` (✅ **1.00x**)  | `8.31 s` (❌ *3.50x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.12 us` (✅ **1.00x**) | `67.92 us` (❌ *2.18x slower*)   | `183.02 us` (❌ *5.88x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `32.06 us` (❌ *2.93x slower*)   | `32.09 us` (❌ *2.93x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.09 ns` (✅ **1.00x**) | `89.27 ns` (❌ *1.86x slower*)    |
| **`from_big-endian_bits`**    | `48.15 ns` (✅ **1.00x**) | `89.32 ns` (❌ *1.85x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.82 ns` (✅ **1.00x**) | `75.24 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `22.82 ns` (✅ **1.00x**) | `47.06 ns` (❌ *2.06x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

