# Curfew

## Algorithm

1. Processing all rooms from left to right
2. If there are b or more students in the room move to the next room
3. Othewise try to move missing number of students from othe rooms

## Testing

```bash
cargo test test_curfew
```

## Assimptotic difficulty

Maximum count of operations needed for processing first and last rooms are n-1, 
for processing second and n-2 rooms are n-3. Therefore total count of operations are:

````
2*(n-1 + n-3 + .. n-(n-1)) = 2*n*(n-1)/4
```

Conclusion: Algorithm complexity is of the order O(n^2)