# Profiling

## Starting Code:

### Midmark
- time: 23.718 Seconds
- current vs intial: 1.000
- current vs previous: 1.000
- valgrind instruction count: 296,499,077,764

### Advent
- time: did not finish*
- current vs intial: N/A
- current vs previous: N/A

### Sandmark
- time: 15 Minutes 24.734 Seconds
- current vs intial: 1.000
- current vs previous: 1.000

## compiling with lto:

### Midmark
- time: 23.421 Seconds
- current vs intial: 0.987
- current vs previous: 0.987
- valgrind instruction count: 296,391,886,525

### Advent
- time: did not finish*
- current vs intial: N/A
- current vs previous: N/A

### Sandmark
- time: 15 Minutes 32.780 Seconds
- current vs intial: 1.009
- current vs previous: 1.009

## removed redundant check (contains):

### Midmark
- time: 624 ms
- current vs intial: 0.026
- current vs previous: 0.027
- valgrind instruction count: 3,119,707,449

### Advent
- time: 6.668 Seconds
- current vs intial: N/A
- current vs previous: N/A

### Sandmark
- time: 15.714 Seconds
- current vs intial: 0.017
- current vs previous: 0.017

## moving definition of variables outside of the loop:

### Midmark
- time: 619 ms
- current vs intial: 0.026
- current vs previous: 0.992
- valgrind fetch count: 3,119,705,247

### Advent
- time: 6.734 Seconds
- current vs intial: N/A
- current vs previous: 1.010

### Sandmark
- time 15.699 Seconds
- current vs intial: 0.017
- current vs previous: 0.999

#### Notes:
*we ran it for over 20 minutes and it didn't finish