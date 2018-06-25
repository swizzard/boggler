extern crate rand;

use self::rand::{ThreadRng, seq};
use std::fmt;
use std::num::Wrapping;

#[derive(Debug, Eq, PartialEq)]
pub struct Die {
    letters: [&'static str; 6],
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for c in self.letters.iter() {
            s.push_str(&format!("{}", c))
        };
        write!(f, "[ {} ]", s)
    }
}

impl Die {
    fn roll(&self, rng: &mut ThreadRng) -> &str {
        let sample = seq::sample_iter(rng, self.letters.iter(), 1);
        match sample {
            Err(_) => panic!("empty die"),
            Ok(s) => s[0]
        }
    }
}

const DICE: [Die; 16] = [Die { letters: ["a", "a", "e", "e", "g", "n"] },
                         Die { letters: ["e", "l", "r", "t", "t", "y"] },
                         Die { letters: ["a", "o", "o", "t", "t", "w"] },
                         Die { letters: ["a", "b", "b", "j", "o", "o"] },
                         Die { letters: ["e", "h", "r", "t", "v", "w"] },
                         Die { letters: ["c", "i", "m", "o", "t", "u"] },
                         Die { letters: ["d", "i", "s", "t", "t", "y"] },
                         Die { letters: ["e", "i", "o", "s", "s", "t"] },
                         Die { letters: ["d", "e", "l", "r", "v", "y"] },
                         Die { letters: ["a", "c", "h", "o", "p", "s"] },
                         Die { letters: ["h", "i", "m", "n", "qu", "u"] },
                         Die { letters: ["e", "e", "i", "n", "s", "u"] },
                         Die { letters: ["e", "e", "g", "h", "n", "w"] },
                         Die { letters: ["a", "f", "f", "k", "p", "s"] },
                         Die { letters: ["h", "l", "n", "n", "r", "z"] },
                         Die { letters: ["d", "e", "i", "l", "r", "x"] }];


#[derive(Debug)]
pub struct Board {
    dice: Vec<Vec<&'static str>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BoardPosition {
    chr: &'static str,
    x: usize,
    y: usize,
}

impl BoardPosition {
    pub fn new(i: usize, j: usize, c: &'static str) -> BoardPosition {
        BoardPosition {
            x: i,
            y: j,
            chr: c
        }
    }
}

impl Board {
    pub fn new(rng: &mut ThreadRng) -> Board {
        let shaken = seq::sample_iter(rng, DICE.iter(), 16).unwrap();
        let mut b: Vec<Vec<&'static str>> = Vec::new();
        for ds in shaken.chunks(4) {
            let ls: Vec<&'static str> = ds.iter().map(|d| d.roll(rng)).collect();
            b.push(ls);
        }
        Board  { dice: b }
    }

    pub fn adjacents(&self, i: usize, j: usize) -> Vec<BoardPosition> {
        let wi = Wrapping(i);
        let wj = Wrapping(j);
        let w1 = Wrapping(1);
        let coords = vec![(wi - w1, wj - w1), (wi - w1, wj), (wi - w1, wj + w1),
                          (wi, wj - w1), (wi, wj + w1),
                          (wi + w1, wj - w1), (wi + w1, wj), (wi + w1, wj + w1)];
        coords.iter().filter_map(|(ix_i, ix_j)|
                                    self.dice.get(ix_i.0).and_then(
                                        |inner| inner.get(ix_j.0).cloned().map(
                                            |c| BoardPosition::new(ix_j.0, ix_i.0, c)))).collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in self.dice.iter() {
            for letter in row.iter() {
                s.push_str(&format!(" {} ", letter.to_ascii_uppercase()))
            };
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adjacents_upper_left_corner() {
        let b = Board {
            dice: vec![vec!["a", "b", "c"],
                       vec!["d", "e", "f"],
                       vec!["g", "h", "i"]]
        };
        let res = b.adjacents(0, 0);
        let expected = vec![BoardPosition::new(1, 0, "b"),
                            BoardPosition::new(0, 1, "d"),
                            BoardPosition::new(1, 1, "e")];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_adjacents_upper_right_corner() {
        let b = Board {
            dice: vec![vec!["a", "b", "c"],
                       vec!["d", "e", "f"],
                       vec!["g", "h", "i"]]
        };
        let res = b.adjacents(0, 2);
        let expected = vec![BoardPosition::new(1, 0, "b"),
                            BoardPosition::new(1, 1, "e"),
                            BoardPosition::new(2, 1, "f")];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_adjacents_top() {
        let b = Board {
            dice: vec![vec!["a", "b", "c"],
                       vec!["d", "e", "f"],
                       vec!["g", "h", "i"]]
        };
        let res = b.adjacents(0, 1);
        let expected = vec![BoardPosition::new(0, 0, "a"),
                            BoardPosition::new(2, 0, "c"),
                            BoardPosition::new(0, 1, "d"),
                            BoardPosition::new(1, 1, "e"),
                            BoardPosition::new(2, 1, "f")];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_adjacents_bottom() {
        let b = Board {
            dice: vec![vec!["a", "b", "c"],
                       vec!["d", "e", "f"],
                       vec!["g", "h", "i"]]
        };
        let res = b.adjacents(2, 1);
        let expected = vec![BoardPosition::new(0, 1, "d"),
                            BoardPosition::new(1, 1, "e"),
                            BoardPosition::new(2, 1, "f"),
                            BoardPosition::new(0, 2, "g"),
                            BoardPosition::new(2, 2, "i")];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_adjacents_left_side() {
        let b = Board {
            dice: vec![vec!["a", "b", "c"],
                       vec!["d", "e", "f"],
                       vec!["g", "h", "i"]]
        };
        let res = b.adjacents(1, 0);
        let expected = vec![BoardPosition::new(0, 0, "a"),
                            BoardPosition::new(1, 0, "b"),
                            BoardPosition::new(1, 1, "e"),
                            BoardPosition::new(0, 2, "g"),
                            BoardPosition::new(1, 2, "h")];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_adjacents_right_side() {
        let b = Board {
            dice: vec![vec!["a", "b", "c"],
                       vec!["d", "e", "f"],
                       vec!["g", "h", "i"]]
        };
        let res = b.adjacents(1, 2);
        let expected = vec![BoardPosition::new(1, 0, "b"),
                            BoardPosition::new(2, 0, "c"),
                            BoardPosition::new(1, 1, "e"),
                            BoardPosition::new(1, 2, "h"),
                            BoardPosition::new(2, 2, "i")];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_adjacents_middle() {
        let b = Board {
            dice: vec![vec!["a", "b", "c"],
                       vec!["d", "e", "f"],
                       vec!["g", "h", "i"]]
        };
        let res = b.adjacents(1, 1);
        let expected = vec![BoardPosition::new(0, 0, "a"),
                            BoardPosition::new(1, 0, "b"),
                            BoardPosition::new(2, 0, "c"),
                            BoardPosition::new(0, 1, "d"),
                            BoardPosition::new(2, 1, "f"),
                            BoardPosition::new(0, 2, "g"),
                            BoardPosition::new(1, 2, "h"),
                            BoardPosition::new(2, 2, "i")];
        assert_eq!(res, expected);
    }
}
