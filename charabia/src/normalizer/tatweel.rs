use super::CharNormalizer;
use crate::normalizer::CharOrStr;
use crate::detection::{Script};
use crate::{Token};


/// TODO describe
pub struct TatweelNormalizer;

const TATWEEL: char = '\u{0640}';

impl CharNormalizer for TatweelNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        (c != TATWEEL).then(|| c.into())
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Arabic && token.lemma().chars().any(|c| c == TATWEEL)
    }
}
