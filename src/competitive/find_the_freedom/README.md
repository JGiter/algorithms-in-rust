# Find the freedom

## Algorithm

1. Sort collection;
2. If collection is not empty build chain from elements which are powers of `k` of minimal elements;
3. Increment `freedom` by half lenght of the chain

## Testing

```bash
cargo test find_freedom
```

## Assimptotic difficulty

*Worst case*: When no k-multiple elements the outer `while` loop contains n
 iteration in each of which is performed as many times as many elements are left
  in collection:
  ```
  n + (n-1) + ... 1 = (n+1)*n/2 - count of multiplications and comparisons
  
  n - count of deletions
  ```
*Best case*: All elements are k-multiples of each other. Will need to do 
(n-1) comparisons, multiplications and deletions

**Resume**: Asimptotic difficulty in worst case is O(n^2)