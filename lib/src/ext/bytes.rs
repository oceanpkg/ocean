/// Extended functionality for
/// [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html).
pub trait BytesExt {
    /// Returns whether `self` matches case-insensitively to `other`, which only
    /// contains [ASCII] characters with the `0b100000` (lowercase) bit set.
    ///
    /// This method can often be used in place of
    /// [`eq_ignore_ascii_case`](https://doc.rust-lang.org/std/primitive.slice.html#method.eq_ignore_ascii_case)
    /// and is more performant since this uses a simple bitwise OR instead of a
    /// lookup table. The main restriction is that only the following [ASCII]
    /// characters may be in `other`:
    ///
    /// | Type          | Values |
    /// | :------------ | :----- |
    /// | Alphanumeric  | `a`-`z`, `0`-`9` |
    /// | Punctuation   | `!`, `?`, `.`, `,`, `:`, `;`, `'`, `` ` ``, `\`, `/`, `#`, `$`, `&`, <code>&#124;</code>, `~` |
    /// | Brackets      | `<`, `>`, `(`, `)`, `{`, `}` |
    /// | Math          | `+`, `-`, `*`, `%`, `=` |
    /// | Non-Graphical | `SPACE`, `DELETE` |
    ///
    /// # Examples
    ///
    /// This method can be used to match against filesystem paths:
    ///
    /// ```rust
    /// use ocean::ext::BytesExt;
    ///
    /// let lower = b"../hello.txt";
    /// let upper = b"../HELLO.TXT";
    ///
    /// assert!(upper.matches_special_lowercase(lower));
    /// assert!(lower.matches_special_lowercase(lower));
    /// assert!(!lower.matches_special_lowercase(upper));
    /// assert!(!upper.matches_special_lowercase(upper));
    /// ```
    ///
    /// [ASCII]: https://en.wikipedia.org/wiki/ASCII
    fn matches_special_lowercase<B: AsRef<[u8]>>(self, other: B) -> bool;
}

// Monomorphized form
fn matches_special_lowercase_imp(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(&a, &b)| { a | 0b100000 == b })
}

impl BytesExt for &[u8] {
    fn matches_special_lowercase<B: AsRef<[u8]>>(self, other: B) -> bool {
        matches_special_lowercase_imp(self.as_ref(), other.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_special_lowercase() {
        let cases = [
            (["ocean.toml", "ocean.toml"], true),
            (["OCEAN.toMl", "ocean.toml"], true),
            (["ocean.toml", "OCEAN.toml"], false),
            (["ocean.tom",  "ocean.toml"], false),
        ];
        for &([a, b], cond) in cases.iter() {
            assert_eq!(a.as_bytes().matches_special_lowercase(b), cond);
        }
    }
}
