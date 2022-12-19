# Profiling

## Starting Code:

### Midmark
- time: 23.718 Seconds
- valgrind instruction count: 296,499,077,764

### Sandmark
- time: 15 Minutes 24.734 Seconds

## compiling with lto:

### Midmark
- time: 23.421 Seconds
- valgrind instruction count:

### Sandmark
- time: 15 Minutes 32.780 Seconds

## removed redundant check (contains):

### Midmark
- time: 624 ms
- valgrind instruction count: 3,119,707,449

### Sandmark
- time: 15.714 Seconds

## moving definition of variables outside of the loop:

### Midmark
- time: 619 ms
- valgrind fetch count: 3,119,705,247

### Sandmark
- time 15.699 Seconds