use phf::phf_map;
use phf::phf_ordered_set;
use phf::phf_set;

pub static ALPHABET_SET: phf::OrderedSet<char> = phf_ordered_set! {
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
};

pub static ALPHABET_ARRAY: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub static LETTER_FREQ: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02288, 0.02015, 0.06094, 0.06966, 0.00153,
    0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
    0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
];

pub static LETTER_FREQ_ARRANG: [char; 26] = [
    'z', 'q', 'x', 'j', 'k', 'v', 'b', 'p', 'y', 'g', 'f', 'w', 'm', 'u', 'c', 'l', 'd', 'r', 'h',
    's', 'n', 'i', 'o', 'a', 't', 'e',
];

pub static ENGLISH_ARR: [&str; 403997] =
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/arr_words.txt"));

pub static BIGRAM_FREQ: phf::Map<&str, f64> =
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/2gram.txt"));

pub static TRIGRAM_FREQ: phf::Map<&str, f64> =
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/3gram.txt"));

pub static ENGLISH_SET: phf::Set<&'static str> =
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/set_words.txt"));
