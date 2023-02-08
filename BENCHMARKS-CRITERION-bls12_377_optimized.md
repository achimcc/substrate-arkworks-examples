# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377_optimized](#sample_bls12_377_optimized)
    - [arithmetic_for_bls12_377_optimized](#arithmetic_for_bls12_377_optimized)
    - [serialization_for_bls12_377_optimized](#serialization_for_bls12_377_optimized)
    - [msm_for_bls12_377_optimized](#msm_for_bls12_377_optimized)
    - [squareroot_for_bls12_377_optimized](#squareroot_for_bls12_377_optimized)
    - [bitwise_operations_for_bls12_377_optimized](#bitwise_operations_for_bls12_377_optimized)
    - [conversions_for_bls12_377_optimized](#conversions_for_bls12_377_optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `285.47 us` (✅ **1.00x**)                 | `2.53 ms` (❌ *8.88x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.78 us` (✅ **1.00x**)          | `5.56 us` (❌ *3.12x slower*)     | `35.29 ns` (🚀 **50.42x faster**)  | `240.10 ns` (🚀 **7.41x faster**)  | `24.43 ns` (🚀 **72.85x faster**) | `10.74 ns` (🚀 **165.70x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.95 us` (✅ **1.00x**)          | `5.81 us` (❌ *2.98x slower*)     | `34.29 ns` (🚀 **56.90x faster**)  | `226.04 ns` (🚀 **8.63x faster**)  | `19.53 ns` (🚀 **99.91x faster**) | `11.38 ns` (🚀 **171.44x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `1.31 us` (✅ **1.00x**)          | `3.95 us` (❌ *3.01x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.31 us` (✅ **1.00x**)          | `4.13 us` (❌ *3.14x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `866.99 ns` (✅ **1.00x**)        | `2.83 us` (❌ *3.27x slower*)     | `16.90 ns` (🚀 **51.30x faster**)  | `138.03 ns` (🚀 **6.28x faster**)  | `13.18 ns` (🚀 **65.76x faster**) | `10.60 ns` (🚀 **81.80x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `462.17 us` (✅ **1.00x**)        | `1.47 ms` (❌ *3.18x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `27.32 ns` (❌ *3.31x slower*)     | `144.17 ns` (❌ *17.48x slower*)   | `21.00 ns` (❌ *2.55x slower*)    | `8.25 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `322.83 ns` (❌ *7.21x slower*)    | `8.64 us` (❌ *193.13x slower*)    | `91.74 ns` (❌ *2.05x slower*)    | `44.76 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `359.81 ns` (❌ *9.45x slower*)    | `6.14 us` (❌ *161.20x slower*)    | `82.32 ns` (❌ *2.16x slower*)    | `38.06 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.31 us` (❌ *2.34x slower*)     | `31.69 us` (❌ *4.54x slower*)     | `16.02 us` (❌ *2.30x slower*)    | `6.98 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `710.75 ns` (❌ *11.28x slower*)   | `17.21 us` (❌ *273.13x slower*)   | `142.00 ns` (❌ *2.25x slower*)   | `63.01 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `672.48 ns` (❌ *6.82x slower*)    | `17.53 us` (❌ *177.71x slower*)   | `227.24 ns` (❌ *2.30x slower*)   | `98.65 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.95 ns` (✅ **1.00x**)        | `10.58 ns` (❌ *1.33x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.47 ns` (✅ **1.00x**)       | `15.04 ns` (❌ *1.44x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.51 ns` (✅ **1.00x**)        | `4.83 ns` (✅ **1.07x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.12 ns` (✅ **1.00x**)        | `4.33 ns` (✅ **1.05x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `204.13 ns` (✅ **1.00x**)        | `270.13 ns` (❌ *1.32x slower*)   | `36.10 ns` (🚀 **5.65x faster**)    | `62.80 ns` (🚀 **3.25x faster**)     | `123.29 ns` (✅ **1.66x faster**)    | `875.39 ns` (❌ *4.29x slower*)    |
| **`serialize_uncompressed`**             | `255.06 ns` (✅ **1.00x**)        | `397.44 ns` (❌ *1.56x slower*)   | `38.05 ns` (🚀 **6.70x faster**)    | `61.28 ns` (🚀 **4.16x faster**)     | `121.50 ns` (🚀 **2.10x faster**)    | `832.55 ns` (❌ *3.26x slower*)    |
| **`deserialize_compressed`**             | `435.37 us` (✅ **1.00x**)        | `1.34 ms` (❌ *3.08x slower*)     | `59.98 ns` (🚀 **7258.43x faster**) | `132.07 ns` (🚀 **3296.51x faster**) | `289.82 ns` (🚀 **1502.19x faster**) | `1.65 us` (🚀 **263.86x faster**)  |
| **`deserialize_compressed_unchecked`**   | `92.54 us` (✅ **1.00x**)         | `245.88 us` (❌ *2.66x slower*)   | `60.73 ns` (🚀 **1523.84x faster**) | `129.11 ns` (🚀 **716.78x faster**)  | `291.43 ns` (🚀 **317.55x faster**)  | `1.69 us` (🚀 **54.80x faster**)   |
| **`deserialize_uncompressed`**           | `341.42 us` (✅ **1.00x**)        | `1.10 ms` (❌ *3.23x slower*)     | `58.49 ns` (🚀 **5836.79x faster**) | `132.50 ns` (🚀 **2576.82x faster**) | `296.57 ns` (🚀 **1151.22x faster**) | `1.71 us` (🚀 **199.83x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `326.74 ns` (✅ **1.00x**)        | `667.31 ns` (❌ *2.04x slower*)   | `59.85 ns` (🚀 **5.46x faster**)    | `132.43 ns` (🚀 **2.47x faster**)    | `299.92 ns` (✅ **1.09x faster**)    | `1.76 us` (❌ *5.37x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `3.53 s` (✅ **1.00x**)           | `10.61 s` (❌ *3.01x slower*)      |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `34.27 us` (✅ **1.00x**) | `89.92 us` (❌ *2.62x slower*)   | `254.63 us` (❌ *7.43x slower*)    |
| **`legendre_for_qr`**    | `11.93 us` (✅ **1.00x**) | `46.45 us` (❌ *3.89x slower*)   | `44.68 us` (❌ *3.75x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.41 ns` (✅ **1.00x**)        | `4.77 ns` (✅ **1.08x slower**)    |
| **`from_little-endian_bits`** | `72.94 ns` (✅ **1.00x**)       | `131.12 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `74.07 ns` (✅ **1.00x**)       | `130.78 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.59 ns` (✅ **1.00x**)        | `4.78 ns` (✅ **1.04x slower**)    |
| **`equality`**                | `4.93 ns` (✅ **1.00x**)        | `5.76 ns` (❌ *1.17x slower*)      |
| **`is_zero`**                 | `4.38 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.04x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.28 ns` (✅ **1.00x**) | `96.49 ns` (❌ *2.18x slower*)    |
| **`into_bigint`** | `26.17 ns` (✅ **1.00x**) | `54.58 ns` (❌ *2.09x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

