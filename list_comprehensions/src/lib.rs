/// Rust has powerful lazy iteration machinisms.
///
/// Primarily list-comprehension can be thought of as being
/// made up of three parts:
///
/// 1. `flatten`
/// 2. `filter`
/// 3. `map`
///
/// We can compose iterators by nesting them into `flat_map`
/// which is just a short-hand for `.map().flatten()`, and then
/// we can use `Option` as an iterator, in `flap_map` which both
/// builds the triplet `[x, y, z]` and then filters it.
#[inline]
pub fn pts(n: u32) -> impl Iterator<Item = [u32; 3]> {
    (1..n + 1).flat_map(move |x| {
        (x..n + 1).flat_map(move |y| {
            (y..n + 1).filter_map(move |z| {
                if x.pow(2) + y.pow(2) == z.pow(2) {
                    Some([x, y, z])
                } else {
                    None
                }
            })
        })
    })
}

/// This macro makes simple use of the methods found
/// on the `Iterator` trait and applies `flap_map`
/// recursively.
macro_rules! comp {
    ($e:expr, for $x:pat in $xs:expr $(, if $c:expr)?) => {{
        $xs.filter_map(move |$x| if $($c &&)? true { Some($e) } else { None })
    }};
    ($e:expr, for $x:pat in $xs:expr $(, for $y:pat in $ys:expr)+ $(, if $c:expr)?) => {{
        $xs.flat_map(move |$x| comp!($e, $(for $y in $ys),+ $(, if $c)?))
    }};
}

/// Rust also has a powerful syntax extension system, with `macro_rules!`
/// it's possible to build a macro which treats the input much like typical
/// list comprehension, and just expand it to iterator operations.
#[inline]
pub fn pts1(n: u32) -> impl Iterator<Item = [u32; 3]> {
    comp!(
        [x, y, z],
        for x in 1..=n,
        for y in x..=n,
        for z in y..=n,
        if x.pow(2) + y.pow(2) == z.pow(2)
    )
}

// The primary reason for the use of the keywords `for` and `if` is
// to ensure there aren't multiple parsing options as `macro_rules!`
// can't deal ambigiouty. It would be possible to use a combination of
// other unambigious tokens like `,` and `;` but ones which could be
// valid after an expression, like `|` and so on.

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checks() {
        let it = pts1(20);

        let v: Vec<_> = it.collect();
        let start = vec![
            [3, 4, 5],
            [5, 12, 13],
            [6, 8, 10],
            [8, 15, 17],
            [9, 12, 15],
            [12, 16, 20],
        ];
        let len = start.len();
        assert_eq!(&v[..len], &start[..]);
        //assert_eq!(&v[..3], &[(0, 0, 0), (0, 0, 1), (0, 0, 2)]);
        //assert_eq!(&v[99..=101], &[(0, 0, 99), (0, 0, 100), (0, 1, 1)])
    }
    #[test]
    fn comp_test() {
        let it = comp!(x + 1, for x in 0..5);
        assert_eq!(it.sum::<u32>(), (1..6).sum::<u32>());

        let it = comp!(x + y, for x in 0..5, for y in 0..5);
        assert_eq!(it.sum::<u32>(), 100);

        let it = comp!(x + 1, for x in 0..5, if x == 2);
        assert_eq!(it.sum::<u32>(), 3);

        let it = comp!(x + y, for x in 0..5, for y in 0..5, if true);
        assert_eq!(it.sum::<u32>(), 100);

        let it = comp!(
            x + y + z + w,
            for x in 0..5,
            for y in 0..5,
            for z in 0..5,
            for w in 0..5,
            if x + y + z + w == 0
        );
        assert_eq!(it.sum::<u32>(), 0);
    }
}
