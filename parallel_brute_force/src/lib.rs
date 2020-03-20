use rayon::prelude::*;
use sha2::{Digest, Sha256};

/// Recursive increament a buffer of lower case characters.
fn incr(mut buffer: &mut [u8], idx: usize) -> bool {
    if buffer.len() <= idx {
        false
    } else if buffer[idx] >= b'z' {
        incr(&mut buffer, idx + 1);
        buffer[idx] = b'a';
        true
    } else {
        buffer[idx] += 1;
        true
    }
}

/// Naive single threaded brute force; looks for a string with bytes within
/// range `b'a'..=b'z'` which hashes down to the input hash, and returns `Some`
/// vector of bytes if it's found, otherwise returns `None`.
fn cracker(hash: &[u8], len: usize) -> Option<Vec<u8>> {
    let mut buffer = vec![b'a'; len];
    while &Sha256::digest(&buffer)[..] != hash {
        if !incr(&mut buffer, 0) {
            return None;
        }
    }
    Some(buffer)
}

#[test]
fn test_cracker() {
    let hash = &Sha256::digest(b"i")[..];
    assert_eq!(cracker(hash, 1), Some(b"i".to_vec()));

    let hash = &Sha256::digest(b"ab")[..];
    assert_eq!(cracker(hash, 2), Some(b"ab".to_vec()));

    let hash = &Sha256::digest(b"abc")[..];
    assert_eq!(cracker(hash, 3), Some(b"abc".to_vec()));

    let hash = &Sha256::digest(b"az")[..];
    assert_eq!(cracker(hash, 2), Some(b"az".to_vec()));
}
