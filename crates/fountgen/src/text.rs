use rand::{
    Rng,
    distr::{SampleString, slice::Choose},
};

pub fn generate_text<R>(
    rng: &mut R,
    len: usize,
    allow_markdown: bool,
    include_numbers: bool,
    inclode_whitespace: bool,
) -> String
where
    R: Rng,
{
    let alphabet = match (include_numbers, inclode_whitespace) {
        (true, true) => ALPHANUMERIC.as_slice(),
        (false, true) => ALPHABETIC.as_slice(),
        (true, false) => ALPHANUMERIC_NO_WHITESPACE.as_slice(),
        (false, false) => ALPHABET.as_slice(),
    };

    Choose::new(alphabet).unwrap().sample_string(rng, len)
}

const ALPHABETIC: [char; 53] = concat_arrays(ALPHABET, WHITESPACE);
const ALPHANUMERIC: [char; 63] = concat_arrays(ALPHABETIC, NUMERIC);
const ALPHANUMERIC_NO_WHITESPACE: [char; 62] = concat_arrays(ALPHABET, NUMERIC);
const ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const NUMERIC: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const WHITESPACE: [char; 1] = [' '];

const fn concat_arrays<const N: usize, const M: usize, const OUT: usize>(
    a: [char; N],
    b: [char; M],
) -> [char; OUT] {
    assert!(OUT == N + M, "Output size must equal N + M");

    let mut res = ['\0'; OUT];

    let mut i = 0;
    while i < N {
        res[i] = a[i];
        i += 1;
    }

    let mut j = 0;
    while j < M {
        res[N + j] = b[j];
        j += 1;
    }

    res
}
