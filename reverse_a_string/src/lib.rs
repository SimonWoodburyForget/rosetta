mod ascii {
    /// Reversing ASCII byte-slice.
    #[test]
    fn reverse() {
        let mut string = b"abcdef".to_vec();
        string.reverse();
        assert_eq!(string, b"fedcba");
    }
}

mod chars {
    /// Reversing Unicode scalar values.
    #[test]
    fn reverse() {
        let string: String = "as⃝df̅".chars().rev().collect();
        assert_ne!(string, "f̅ds⃝a");
        assert_eq!(string, "̅fd⃝sa");
    }
}

mod graphemes {
    /// Reversing graphemes
    #[test]
    fn reverse() {
        use unicode_segmentation::UnicodeSegmentation;
        let string: String = "as⃝df̅".graphemes(true).rev().collect();
        assert_eq!(string, "f̅ds⃝a");
    }
}
