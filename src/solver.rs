extern crate ascii;

use std::slice::{Iter};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use self::ascii::*;
use board::{Board, BoardPosition};

type BMap = BTreeMap<AsciiChar, BTNode>;


#[derive(Debug)]
pub struct BoardSolution {
    words: Vec<AsciiString>,
    score: u32,
}

impl BoardSolution {
    pub fn new() -> Self {
        BoardSolution {
            words: Vec::new(),
            score: 0,
        }
    }

    pub fn add_word(mut self, word: AsciiString) {
        self.score += BogglerTrie::count_word(&word);
        self.words.push(word);
    }
}

impl fmt::Display for BoardSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for w in self.words.iter() {
            s.push_str(&format!("{}\n", w));
        }
        write!(f, "\nScore: {}\nWords:\n{}", self.score, s)
    }
}

pub struct BogglerTrie {
    children: BMap
}

impl BogglerTrie {
    pub fn new(words: Vec<String>) -> Self {
        let mut bt = BogglerTrie {
            children: BTreeMap::new()
        };
        for w in words {
            match BogglerTrie::prepare_word(w) {
                None => (),
                Some(ref ascii_word) => {
                    &bt.add_word(ascii_word);
                }
            }
        }
        bt
    }

    fn from_ascii_words(words: Vec<AsciiString>) -> Self {
        let mut bt = BogglerTrie {
            children: BTreeMap::new()
        };
        for w in words {
            &bt.add_word(&w);
        }
        bt
    }

    pub fn from_file(fname: &str) -> Option<BogglerTrie> {
        match File::open(fname) {
            Err(e) => {
                println!("error: {}", e);
                None
            },
            Ok(f) => {
                let mut reader = BufReader::new(f);
                let mut words: Vec<AsciiString> = Vec::new();
                let mut s = String::new();
                while let Ok(_) = reader.read_line(&mut s) {
                    if let Some(w) = BogglerTrie::prepare_word(s.clone()) {
                        words.push(w);
                        s.clear();
                    }
                }
                Some(BogglerTrie::from_ascii_words(words))
            }
        }
    }

    fn count_word(word: &AsciiString) -> u32 {
        let q = &AsciiChar::from('q').unwrap();
        let u = &AsciiChar::from('u').unwrap();
        let mut prev_q = false;
        let mut cnt = 0;
        for c in word.chars() {
            if c == q {
                prev_q = true;
                cnt += 1;
            } else {
                if !(c == u && prev_q) {
                    cnt += 1;
                }
                prev_q = false;
            }
        };
        cnt
    }

    fn prepare_word(word: String) -> Option<AsciiString> {
        AsciiString::from_ascii(word).ok().and_then(|aw| {
            let aw_lower = aw.to_ascii_lowercase();
            let cnt = BogglerTrie::count_word(&aw_lower);
            if cnt >= 3 && cnt <= 16 {
                Some(aw_lower)
            } else {
                None
            }
        })
    }

    fn add_word(&mut self, word: &AsciiString) {
        let mut chrs = word.chars();
        match chrs.next() {
            None => (),
            Some(ch) => self.children.entry(*ch).or_insert(BTNode::new()).insert(&mut chrs, word)
        }
    }

    pub fn find(&self, word: String) -> Option<String> {
        match AsciiString::from_ascii(word) {
            Err(_) => None,
            Ok(ascii_word) => {
                let mut chrs = ascii_word.chars();
                match chrs.next() {
                    None => None,
                    Some(c) => self.children.get(c).and_then(|ref child| child.find(&mut chrs, &ascii_word)),
                }
            }
        }
    }

    fn find_letter(&self, chr: &AsciiChar) -> Option<&BTNode> {
        self.children.get(chr)
    }

    pub fn solve(&self, board: &Board) -> BoardSolution {
        let mut x = 0;
        let mut y = 0;
        let mut seen: HashSet<BoardPosition> = HashSet::with_capacity(16);
        while x < 16 && y < 16 {
            let adjcts = board.adjacents(x, y);
        }
        unimplemented!()
    }
}

#[derive(Debug)]
struct BTNode {
    full_word: Option<AsciiString>,
    children: BMap,
}

impl BTNode {
    fn new() -> Self {
        BTNode {
            full_word: None,
            children: BTreeMap::new(),
        }
    }

    fn insert(&mut self, chrs: &mut Iter<AsciiChar>, word: &AsciiString) {
        match chrs.next() {
            None => self.full_word = Some(word.to_owned()),
            Some(c) => self.children.entry(*c).or_insert(BTNode::new()).insert(chrs, word),
        }
    }

    fn find(&self, chrs: &mut Iter<AsciiChar>, word: &AsciiString) -> Option<String> {
        match chrs.next() {
            None => {
                match self.full_word {
                    None => None,
                    Some(ref fw) => {
                        if fw == word {
                            Some(fw.to_owned().into())
                        } else {
                            None
                        }
                    }
                }
            },
            Some(c) => self.children.get(c).and_then(|child| child.find(chrs, word))
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_empty() {
        let v: Vec<String> = Vec::new();
        let bt = BogglerTrie::new(v);
        assert!(bt.children.is_empty());
    }

    #[test]
    fn new_one_word() {
        let v = vec![String::from("her")];
        let bt = BogglerTrie::new(v);
        let h = AsciiChar::from('h').unwrap();
        let e = AsciiChar::from('e').unwrap();
        let r = AsciiChar::from('r').unwrap();
        let res = bt.children.get(&h).and_then(|c1| c1.children.get(&e)).and_then(|c2| c2.children.get(&r)).as_ref().unwrap().full_word.as_ref().unwrap();
        assert_eq!(AsciiString::from_ascii("her").unwrap(), *res);
    }

    #[test]
    fn two_new_words() {
        let v = vec![String::from("her"), String::from("hero")];
        let bt = BogglerTrie::new(v);
        let h = AsciiChar::from('h').unwrap();
        let e = AsciiChar::from('e').unwrap();
        let r = AsciiChar::from('r').unwrap();
        let o = AsciiChar::from('o').unwrap();
        let r1 = bt.children.get(&h).and_then(|c1| c1.children.get(&e)).and_then(|c2| c2.children.get(&r)).as_ref().unwrap().full_word.as_ref().unwrap();
        let r2 = bt.children.get(&h).and_then(|c1| c1.children.get(&e)).and_then(|c2| c2.children.get(&r)).and_then(|c3| c3.children.get(&o)).as_ref().unwrap().full_word.as_ref().unwrap();
        assert_eq!(AsciiString::from_ascii("her").unwrap(), *r1);
        assert_eq!(AsciiString::from_ascii("hero").unwrap(), *r2);
    }

    #[test]
    fn find() {
        let v = vec![String::from("her"), String::from("hero"), String::from("heroin"),
                     String::from("heroine"), String::from("heroic"),
                     String::from("heroism")];
        let bt = BogglerTrie::new(v);
        match &bt.find(String::from("her")).take() {
            None => panic!("'her' not found"),
            Some(w) => assert_eq!(String::from("her"), *w)
        }
        match &bt.find(String::from("hero")).take() {
            None => panic!("'hero' not found"),
            Some(w) => assert_eq!(String::from("hero"), *w)
        }
        match &bt.find(String::from("heroin")).take() {
            None => panic!("'heroin' not found"),
            Some(w) => assert_eq!(String::from("heroin"), *w)
        }
        match &bt.find(String::from("heroine")).take() {
            None => panic!("'heroine' not found"),
            Some(w) => assert_eq!(String::from("heroine"), *w)
        }
        match &bt.find(String::from("heroic")).take() {
            None => panic!("'heroic' not found"),
            Some(w) => assert_eq!(String::from("heroic"), *w)
        }
        match &bt.find(String::from("heroism")).take() {
            None => panic!("'heroism'not found"),
            Some(w) => assert_eq!(String::from("heroism"), *w)
        }
        match &bt.find(String::from("here")).take() {
            None => (),
            Some(_) => panic!("found 'here'")
        }
    }

    #[test]
    fn count_word_no_q() {
        assert_eq!(3, BogglerTrie::count_word(&AsciiString::from_ascii("her").unwrap()));
    }

    #[test]
    fn count_word_q() {
        assert_eq!(6, BogglerTrie::count_word(&AsciiString::from_ascii("aqueous").unwrap()));
    }

    #[test]
    fn count_word_u_no_q() {
        assert_eq!(4, BogglerTrie::count_word(&AsciiString::from_ascii("four").unwrap()));
    }

    #[test]
    fn prepare_word_lowercase() {
        let res = BogglerTrie::prepare_word(String::from("Heron")).unwrap();
        assert_eq!(AsciiString::from_ascii("heron").unwrap(), res);
    }

    #[test]
    fn find_letter() {
        let mut bt = BogglerTrie {
            children: BTreeMap::new()
        };
        bt.add_word(AsciiString::from_ascii("a").as_ref().unwrap());
        assert!(bt.find_letter(AsciiChar::from('a').as_ref().unwrap()).is_some());
    }
}
