use crate::{detection, tools};

pub fn scores(text: &str) -> (f64, f64, f64, f64, f64, f64) {
    let (s1, s2, s3, s4) = tools::get_scores(text);
    let (s5, s6) = detection::get_scores(text);
    (s1, s2, s3, s4, s5, s6)
}
