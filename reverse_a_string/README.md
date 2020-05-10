
Reversing ASCII byte-slice (in-place):

``` rust
let mut buffer = b"abcdef".to_vec();
buffer.reverse();
assert_eq!(buffer, b"fedcba");
```

Reversing Unicode scalar values:

``` rust
let output: String = "一二三四五六七八九十".chars().rev().collect();
assert_eq!(output, "十九八七六五四三二一");
```

Reversing a `Chars` iterator doesn't solve the complete problem, because it iterates unicode scalar values, which doesn't account for combining marks:

``` rust
let output: String = "as⃝df̅".chars().rev().collect();
assert_ne!(output, "f̅ds⃝a"); // should be this
assert_eq!(output, "̅fd⃝sa");

```

Reversing graphemes clusters, which is provided by the [unicode-segmentation](https://unicode-rs.github.io/unicode-segmentation/unicode_segmentation/struct.Graphemes.html) crate, solves the problem:

``` rust
use unicode_segmentation::UnicodeSegmentation;

let output: String = "as⃝df̅".graphemes(true).rev().collect();
assert_eq!(output, "f̅ds⃝a");
```
