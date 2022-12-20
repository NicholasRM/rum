# rum (Profiling and Tuning)

## Contributors and Contributions

This assignment has been completed by Nicholas Mendes and Matthew Kelley.
Help was received from the sources below:

* Noah Daniels: suggested turning on lto
* [rust-lang.com](https://www.rust-lang.org/): understanding the .contains() method for vectors
given amount of values

## Most Expensive Program Component
Due to kcachegrind we discovered that after we optimized our program the slowest section is now Map Segment:

```rust
    pub fn map_seg(&mut self, size: usize) -> usize {
        if self.available_segs.is_empty() {
            self.segs.push(vec![0; size]);
            self.segs.len() - 1
        } else {
            let next_seg = self.available_segs.pop().unwrap() as usize;
            self.segs[next_seg] = vec![0; size];
            next_seg
        }
    }
```
We can improve this by reducing the number of branches in the code, which can be done by writing much of the code on a single as shown below:

```rust
self.available_segs.pop.unwrap_or_else(|| {self.active_segs.push(Vec::new()); self.active_segs.len() - 1});
```
Due to worries about code legibility and the marginal increase in speed this change would provide, we decided not to implement it.

## Time Used

Approximately 3 hours were spent analyzing the assignemnt

Approximately 8 hours were spent solving the problems in our code and implementing changes following our analysis.
