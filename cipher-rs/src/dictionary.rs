use phf::phf_ordered_map;
use phf::phf_ordered_set;
use phf::phf_set;

pub static ENGLISH: phf::Set<&'static str> =
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/words_s.txt"));

pub static ALPHABET_SET: phf::OrderedSet<char> = phf_ordered_set! {
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
};

pub static ALPHABET_ARRAY: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub static LETTER_FREQ: phf::OrderedMap<char, f64> = phf_ordered_map! {
    'e' => 0.12702,
    't' => 0.09056,
    'a' => 0.08167,
    'o' => 0.07507,
    'i' => 0.06966,
    'n' => 0.06749,
    's' => 0.06327,
    'h' => 0.06094,
    'r' => 0.05987,
    'd' => 0.04253,
    'l' => 0.04025,
    'c' => 0.02782,
    'u' => 0.02758,
    'm' => 0.02406,
    'w' => 0.02360,
    'f' => 0.02288,
    'g' => 0.02015,
    'y' => 0.01974,
    'p' => 0.01929,
    'b' => 0.01492,
    'v' => 0.00978,
    'k' => 0.00772,
    'j' => 0.00153,
    'x' => 0.00150,
    'q' => 0.00095,
    'z' => 0.00074,
};

pub static L_ENGLISH_ARR: [&str; 403978] =
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/words.txt"));
