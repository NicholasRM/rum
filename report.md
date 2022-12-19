# Profiling

## Starting Code:

### Midmark
- time: 24 Seconds
- valgrind instruction count: 296,499,077,764

### Sandmark
- time: 15 Minutes

## compiling with lto:

### Midmark
- time: 23 Seconds
- valgrind instruction count:

### Sandmark
- time: 15 Minutes

## removed redundant check (contains):

### Midmark
- time: 624 ms
- valgrind instruction count: 3,119,707,449

### Sandmark
- time: 17 Seconds

## moving definition of variables outside of the loop:

### Midmark
- time: 619 ms
- valgrind fetch count:

### Sandmark