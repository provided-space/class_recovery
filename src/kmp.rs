// Original: https://github.com/RemieRichards/kmpsearch-rs/blob/master/src/lib.rs
pub trait Haystack {
    // Produce a 'pattern table' for use with the Knuth Morris Pratt algorithm.
    fn pattern_table(needle: &[u8]) -> Vec<usize> {
        let mut i = 0;
        let mut j = 1;
        let mut array = vec![0; needle.len()];
        while j < needle.len() {
            if needle[i] == needle[j] {
                i += 1;
                array[j] = i;
                j += 1;
                continue;
            }

            if i != 0 {
                i = array[i - 1];
                continue;
            }

            array[j] = i;
            j += 1;
        }
        return array;
    }

    fn indices_of_needle<N: AsRef<[u8]>>(&self, needle: N) -> Vec<usize>;
}

// Implementation allowing anything convertible to a &[u8] to use Haystack methods.
impl<H: AsRef<[u8]>> Haystack for H {
    fn indices_of_needle<N: AsRef<[u8]>>(&self, needle: N) -> Vec<usize> {
        let needle = needle.as_ref();
        let pattern_table = Self::pattern_table(needle);
        let haystack = &self.as_ref();

        let mut haystack_c = 0usize;
        let mut needle_c = 0usize;

        let haystack_len = haystack.len();
        let needle_len = needle.len();

        let mut indices = Vec::new();

        while haystack_c < haystack_len {
            if haystack[haystack_c] == needle[needle_c] {
                haystack_c += 1;
                needle_c += 1;
            }

            if needle_c == needle_len {
                indices.push(haystack_c - needle_len);
                needle_c = 0;
                continue;
            }

            if haystack_c >= haystack_len || haystack[haystack_c] == needle[needle_c] {
                continue;
            }

            if needle_c != 0 {
                needle_c = pattern_table[needle_c - 1];
                continue;
            }

            haystack_c += 1;
        }

        return indices;
    }
}
