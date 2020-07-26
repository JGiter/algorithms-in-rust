# Circle Formation

## Algorithm

1. Sorting an array;
2. Odd elements of the sorted array are added to the end of the desired sequence, and odd elements to the beginning.

## Testing

```bash
cargo test test_circle
```

## Assimptotic difficulty

The construction of the ring and calculating the maximum discomfort is performed
in two single cycles; therefore, the complexity is O(n)