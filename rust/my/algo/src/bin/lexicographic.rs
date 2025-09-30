// Lexicographic order (dictionary order):
// 1) Compare left to right.
// 2) First differing character decides.
// 3) If one string is a prefix of the other, the shorter one is smaller.
//    e.g. "ab" < "aba", "z" < "za", "zz" < "zzz"
//
// NOTE: All demos use lowercase ASCII so slicing by byte indices is safe.

fn main() {
    // --- Basic comparisons (what "larger/smaller" mean) ---
    assert!("a" < "b");        // 'a' comes before 'b'
    assert!("abc" < "abd");    // first difference: 'c' < 'd'
    assert!("az" > "ay");      // first difference: 'z' > 'y'

    // Prefix rule: shorter prefix is smaller
    assert!("ab" < "aba");
    assert!("zz" < "zzz");
    assert!("z"  < "za");

    // --- Key property used in the problem ---
    // For a FIXED start index i, longer substring is lexicographically larger
    // because the shorter is a proper prefix of the longer.
    let s = "abcxyz"; // ASCII => 1 byte per char
    // start at byte index 1 ('b')
    assert!(&s[1..2] < &s[1..3]); // "b"   < "bc"
    assert!(&s[1..3] < &s[1..4]); // "bc"  < "bcx"
    assert!(&s[1..4] < &s[1..5]); // "bcx" < "bcxy"
    assert!(&s[1..5] < &s[1..6]); // "bcxy"< "bcxyz"

    // --- Our insight: "prefix vs suffix" winners ---
    // Winner can be a PREFIX (i = 0) if the biggest letter appears early
    assert_eq!(answer_string("zaa", 2), "za"); // L = 2 → candidates: "za", "aa", "a" → "za" (prefix) ✅
    assert_eq!(answer_string("baa", 2), "ba"); // L = 2 → "ba", "aa", "a" → "ba" (prefix) ✅

    // Winner can be a SUFFIX when a bigger letter appears later
    assert_eq!(answer_string("abdcgf", 3), "gf"); // L = 4 → ...,"cgf","gf","f" → "gf" (suffix) ✅
    assert_eq!(answer_string("abz", 2), "z");     // L = 2 → "ab","bz","z" → "z" (suffix) ✅
    assert_eq!(answer_string("aaaz", 3), "z");    // L = 2 → "aa","aa","az","z" → "z" (suffix) ✅

    // Special case: m == 1 → only split is the whole word
    assert_eq!(answer_string("gh", 1), "gh");

    println!("All assertions passed ✅");
}

// Minimal solution implementing the rule:
// L = n - m + 1; for each start i, keep only the LONGEST allowed substring:
// candidate(i) = word[i .. min(i+L, n)], then take the global max.
// (ASCII-only slicing; for general Unicode use char_indices.)
fn answer_string(word: &str, num_friends: usize) -> String {
    let n = word.len();
    if num_friends == 1 {
        return word.to_string();
    }
    let l = n - num_friends + 1;

    let mut best: &str = "";
    for i in 0..n {
        let end = (i + l).min(n);
        let cand = &word[i..end];
        if cand > best {
            best = cand;
        }
    }
    best.to_string()
}
