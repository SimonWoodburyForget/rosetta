/// Rust has powerful lazy iterator types:
/// - `flat_map` maps range iterators and then flattens them;
/// - `filter_map` maps the iterator and then filters it by `Some(value)`;
/// - `impl Iterator` allows returning the iterator statically.
pub fn pts(n: u32) -> impl Iterator<Item = (u32, u32, u32)> {
    (1..=n).flat_map(move |x| {
        (x..=n).flat_map(move |y| {
            (y..=n).filter_map(move |z| {
                if x.pow(2) + y.pow(2) == z.pow(2) {
                    Some((x, y, z))
                } else {
                    None
                }
            })
        })
    })
}

macro_rules! comp {
    ($e:expr; $x:pat in $xs:expr $(; $c:expr)?) => {{
        $xs.filter_map(move |$x| if $($c &&)? true { Some($e) } else { None })
    }};
    ($e:expr; $x:pat in $xs:expr $(, $y:pat in $ys:expr)+ $(; $c:expr)?) => {{
        $xs.flat_map(move |$x| comp!($e; $($y in $ys),+ $(; $c)?))
    }};
}

/// Rust also has a powerful syntax extension system, with `macro_rules!`
/// it's possible to build a macro which treats the input much like list
/// comprehension, and just expands to iterator operations.
pub fn pts1(n: u32) -> impl Iterator<Item = (u32, u32, u32)> {
    comp!(
        (x, y, z);
        x in 1..=n,
        y in x..=n,
        z in y..=n;
        x.pow(2) + y.pow(2) == z.pow(2)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checks() {
        let it = pts(20);

        let v: Vec<_> = it.collect();
        let start = vec![
            (3, 4, 5),
            (5, 12, 13),
            (6, 8, 10),
            (8, 15, 17),
            (9, 12, 15),
            (12, 16, 20),
        ];
        let len = start.len();
        assert_eq!(&v[..len], &start[..]);
        //assert_eq!(&v[..3], &[(0, 0, 0), (0, 0, 1), (0, 0, 2)]);
        //assert_eq!(&v[99..=101], &[(0, 0, 99), (0, 0, 100), (0, 1, 1)])
    }
    #[test]
    fn comp_test() {
        let it = comp!(x + 1; x in 0..5);
        assert_eq!(it.sum::<u32>(), (1..6).sum::<u32>());

        let it = comp!(x + y; x in 0..5, y in 0..5);
        assert_eq!(it.sum::<u32>(), 100);

        let it = comp!(x + 1; x in 0..5; x == 2);
        assert_eq!(it.sum::<u32>(), 3);

        let it = comp!(x + y; x in 0..5, y in 0..5; true);
        assert_eq!(it.sum::<u32>(), 100);
    }
}
