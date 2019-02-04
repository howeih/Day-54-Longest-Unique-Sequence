Day 54: Longest Unique Sequence
<br>
In a sequence find the longest unique consecutive subsequence, i.e. sequence consisting of unique items. Make the algorithm run in O(n) time in worst case.
<br>
run:
```
fn main() {
    let sequence:&str = "Premature optimization is the root of all evil -- DonaldKnuth";
    let (i, j) = longest_unique_sequence(sequence);
    println!("{} {} {}",  i, j, &sequence[i..j]);
}
```
result:
```
3 12 mature op
```