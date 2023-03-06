# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761_optimized](#sample_bw6_761_optimized)
    - [arithmetic_for_bw6_761_optimized](#arithmetic_for_bw6_761_optimized)
    - [serialization_for_bw6_761_optimized](#serialization_for_bw6_761_optimized)
    - [msm_for_bw6_761_optimized](#msm_for_bw6_761_optimized)
    - [squareroot_for_bw6_761_optimized](#squareroot_for_bw6_761_optimized)
    - [bitwise_operations_for_bw6_761_optimized](#bitwise_operations_for_bw6_761_optimized)
    - [conversions_for_bw6_761_optimized](#conversions_for_bw6_761_optimized)
    - [pairing_for_bw6_761optimized](#pairing_for_bw6_761optimized)

## Benchmark Results

### sample_bw6_761_optimized

|        | `g1projectivebw6_761_elements`          | `g2projectivebw6_761_elements`           |
|:-------|:----------------------------------------|:---------------------------------------- |
|        | `2.03 ms` (✅ **1.00x**)                 | `2.07 ms` (✅ **1.02x slower**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                    | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.81 us` (✅ **1.00x**)        | `4.60 us` (✅ **1.05x faster**) | `104.69 ns` (🚀 **45.98x faster**) | `205.68 ns` (🚀 **23.40x faster**) | `35.02 ns` (🚀 **137.46x faster**) | `22.64 ns` (🚀 **212.59x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.84 us` (✅ **1.00x**)        | `4.64 us` (✅ **1.04x faster**) | `97.62 ns` (🚀 **49.62x faster**)  | `197.26 ns` (🚀 **24.56x faster**) | `35.28 ns` (🚀 **137.28x faster**) | `17.78 ns` (🚀 **272.46x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.35 us` (✅ **1.00x**)        | `3.46 us` (✅ **1.03x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.35 us` (✅ **1.02x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.25 us` (✅ **1.00x**)        | `2.15 us` (✅ **1.04x faster**) | `85.21 ns` (🚀 **26.36x faster**)  | `161.94 ns` (🚀 **13.87x faster**) | `26.82 ns` (🚀 **83.75x faster**)  | `9.07 ns` (🚀 **247.74x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.76 ms` (✅ **1.00x**)        | `1.74 ms` (✅ **1.01x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `80.65 ns` (❌ *3.84x slower*)     | `149.20 ns` (❌ *7.10x slower*)    | `28.29 ns` (❌ *1.35x slower*)     | `21.01 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.52 us` (❌ *31.24x slower*)     | `7.99 us` (❌ *99.07x slower*)     | `320.31 ns` (❌ *3.97x slower*)    | `80.61 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.84 us` (❌ *26.41x slower*)     | `5.37 us` (❌ *77.22x slower*)     | `254.10 ns` (❌ *3.66x slower*)    | `69.49 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `58.83 us` (❌ *3.75x slower*)     | `67.31 us` (❌ *4.29x slower*)     | `56.08 us` (❌ *3.57x slower*)     | `15.69 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.24 us` (❌ *41.13x slower*)     | `15.62 us` (❌ *122.73x slower*)   | `480.26 ns` (❌ *3.77x slower*)    | `127.29 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.42 us` (❌ *29.77x slower*)     | `15.99 us` (❌ *87.91x slower*)    | `667.61 ns` (❌ *3.67x slower*)    | `181.92 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `9.44 ns` (✅ **1.00x**)        | `18.30 ns` (❌ *1.94x slower*)   | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `12.60 ns` (✅ **1.00x**)       | `24.86 ns` (❌ *1.97x slower*)   | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.65 ns` (✅ **1.00x**)        | `4.78 ns` (✅ **1.03x slower**)  | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.29 ns` (✅ **1.00x**)        | `4.37 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                        | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `526.11 ns` (✅ **1.00x**)      | `525.86 ns` (✅ **1.00x faster**) | `58.01 ns` (🚀 **9.07x faster**)      | `184.29 ns` (🚀 **2.85x faster**)    | `559.17 ns` (✅ **1.06x slower**)  | `1.13 us` (❌ *2.14x slower*)      |
| **`serialize_uncompressed`**             | `728.43 ns` (✅ **1.00x**)      | `753.97 ns` (✅ **1.04x slower**) | `60.14 ns` (🚀 **12.11x faster**)     | `189.00 ns` (🚀 **3.85x faster**)    | `547.33 ns` (✅ **1.33x faster**)  | `1.13 us` (❌ *1.55x slower*)      |
| **`deserialize_compressed`**             | `1.53 ms` (✅ **1.00x**)        | `1.57 ms` (✅ **1.02x slower**)   | `114.15 ns` (🚀 **13419.17x faster**) | `374.26 ns` (🚀 **4092.93x faster**) | `1.08 us` (🚀 **1421.28x faster**) | `2.07 us` (🚀 **740.47x faster**)  |
| **`deserialize_compressed_unchecked`**   | `296.52 us` (✅ **1.00x**)      | `293.32 us` (✅ **1.01x faster**) | `109.46 ns` (🚀 **2708.81x faster**)  | `358.03 ns` (🚀 **828.19x faster**)  | `1.08 us` (🚀 **274.97x faster**)  | `2.30 us` (🚀 **128.96x faster**)  |
| **`deserialize_uncompressed`**           | `1.32 ms` (✅ **1.00x**)        | `1.35 ms` (✅ **1.02x slower**)   | `107.80 ns` (🚀 **12233.61x faster**) | `358.34 ns` (🚀 **3680.19x faster**) | `1.07 us` (🚀 **1237.47x faster**) | `2.10 us` (🚀 **628.35x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `776.47 ns` (✅ **1.00x**)      | `762.46 ns` (✅ **1.02x faster**) | `114.91 ns` (🚀 **6.76x faster**)     | `356.91 ns` (🚀 **2.18x faster**)    | `1.09 us` (❌ *1.41x slower*)      | `2.13 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.71 s` (✅ **1.00x**)        | `12.60 s` (✅ **1.01x faster**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `78.56 us` (✅ **1.00x**) | `294.05 us` (❌ *3.74x slower*)   | `6.93 ms` (❌ *88.15x slower*)     |
| **`legendre_for_qr`**    | `34.15 us` (✅ **1.00x**) | `292.62 us` (❌ *8.57x slower*)   | `297.09 us` (❌ *8.70x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.92 ns` (✅ **1.00x**)        | `4.93 ns` (✅ **1.00x slower**)    |
| **`from_little-endian_bits`** | `126.75 ns` (✅ **1.00x**)      | `276.29 ns` (❌ *2.18x slower*)    |
| **`from_big-endian_bits`**    | `124.19 ns` (✅ **1.00x**)      | `245.76 ns` (❌ *1.98x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `4.96 ns` (✅ **1.02x slower**)    |
| **`equality`**                | `5.71 ns` (✅ **1.00x**)        | `5.90 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `4.70 ns` (✅ **1.00x**)        | `5.01 ns` (✅ **1.07x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `93.02 ns` (✅ **1.00x**) | `324.40 ns` (❌ *3.49x slower*)    |
| **`into_bigint`** | `49.93 ns` (✅ **1.00x**) | `167.92 ns` (❌ *3.36x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `19.26 ns` (✅ **1.00x**)                       | `11.95 ns` (✅ **1.61x faster**)                | `4.27 ms` (❌ *221794.76x slower*)           | `4.21 ms` (❌ *218749.25x slower*)                    | `8.85 ms` (❌ *459465.09x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

