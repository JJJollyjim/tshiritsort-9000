use lazy_static::lazy_static;
use regex::{Captures, Regex};

unsafe fn parse_u16(text: &[u8]) -> (u16, usize) {
    let mut idx = text.len() - 1;

    while text[idx].is_ascii_digit() {
        idx -= 1;
    }
    (
        u16::from_str_radix(std::str::from_utf8_unchecked(&text[(idx + 1)..]), 10).unwrap(),
        idx,
    )
}

enum Polarity {
    Small,
    Large,
}

impl Polarity {
    fn polarise(self, x: i32) -> i32 {
        match self {
            Polarity::Small => -x,
            Polarity::Large => x,
        }
    }
}

// pub fn size_sort_key(text: &[u8]) -> (&str, i32) {
pub fn size_sort_key(text_str: &str) -> (&str, i32) {
    unsafe {
        let text = text_str.as_bytes();

        let mut idx = text.len() - 1;

        while text[idx] == b' ' {
            idx -= 1;
        }

        if text[idx].is_ascii_digit() {
            let (n, idx2) = parse_u16(&text[..=idx]);
            return (std::str::from_utf8_unchecked(&text[..=idx2]), n as i32);
        }

        if text[..=idx].ends_with(b"medium") {
            return (std::str::from_utf8_unchecked(&text[..=(idx - 6)]), 0);
        } else if text[..=idx].ends_with(b"Medium") {
            return (std::str::from_utf8_unchecked(&text[..=(idx - 6)]), 0);
        } else if text[..=idx].ends_with(b"m") {
            return (std::str::from_utf8_unchecked(&text[..=(idx - 1)]), 0);
        } else if text[..=idx].ends_with(b"M") {
            return (std::str::from_utf8_unchecked(&text[..=(idx - 1)]), 0);
        }

        let (polarity, mut idx) = match text[..=idx] {
            [.., b's'] => (Polarity::Small, idx - 1),
            [.., b'S'] => (Polarity::Small, idx - 1),
            [.., b's', b'm', b'a', b'l', b'l'] => (Polarity::Small, idx - 5),
            [.., b'S', b'm', b'a', b'l', b'l'] => (Polarity::Small, idx - 5),
            [.., b'l'] => (Polarity::Large, idx - 1),
            [.., b'L'] => (Polarity::Large, idx - 1),
            [.., b'l', b'a', b'r', b'g', b'e'] => (Polarity::Large, idx - 5),
            [.., b'L', b'a', b'r', b'g', b'e'] => (Polarity::Large, idx - 5),
            _ => return (text_str, 0),
        };

        let mut xcount = 0;
        loop {
            let mut idx_inner = idx;
            while !text[idx_inner].is_ascii_alphanumeric() {
                idx_inner -= 1;
            }

            match text[..=idx_inner] {
                [.., b'x'] => idx = idx_inner - 1,
                [.., b'X'] => idx = idx_inner - 1,
                [.., b'e', b'x', b't', b'r', b'a'] => idx = idx_inner - 5,
                [.., b'E', b'x', b't', b'r', b'a'] => idx = idx_inner - 5,
                _ => break,
            }

            xcount += 1;
        }

        if xcount == 1 {
            let mut idx_inner = idx;
            while !text[idx_inner].is_ascii_alphanumeric() {
                idx_inner -= 1;
            }

            if text[idx_inner].is_ascii_digit() {
                let (n, idx2) = parse_u16(&text[..=idx_inner]);
                (
                    std::str::from_utf8_unchecked(&text[..=idx2]),
                    polarity.polarise(n as i32 + 1),
                )
            } else {
                (
                    std::str::from_utf8_unchecked(&text[..=idx]),
                    polarity.polarise(xcount + 1),
                )
            }
        } else {
            (
                std::str::from_utf8_unchecked(&text[..=idx]),
                polarity.polarise(xcount + 1),
            )
        }
    }
}

pub fn size_sort_key_old(text: &str) -> (&str, i32) {
    /*
    .-------------.
    | size |  key |
    |------+------|
    | nxs  | -n-1 |
    | s    |   -1 |
    | m    |    0 |
    | l    |   +1 |
    | nxl  | +n+1 |
    `-------------`
     */
    lazy_static! {
        static ref REGEXES: [(Regex, fn(&Captures) -> i32); 6] = [
            // Purely numeric size
            (Regex::new(r"(?i-u)(\d+)\s*$").unwrap(), |c| {
                let n: u16 = c[1].parse().unwrap();
                n.into()
            }),
            // Medium
            (Regex::new(r"(?i-u)(?:m|med|medium)\s*$").unwrap(), |_| {
                0
            }),
            // Numeric XS
            (
                Regex::new(r"(?i-u)(\d+)\W*(?:x|extra)\W*(?:s|sm|small)\s*$").unwrap(),
                |c| {
                    // https://turbo.fish/
                    -1 - (c[1].parse::<u16>().unwrap() as i32)
                }
            ),
            // Numeric XL
            (
                Regex::new(r"(?i-u)(\d+)\W*(?:x|extra)\W*(?:l|lg|large)\s*$").unwrap(),
                |c| {
                    1 + (c[1].parse::<u16>().unwrap() as i32)
                }
            ),
            // Repeated XS (0 repetitions for S)
            (
                Regex::new(r"(?i-u)((?:(?:x|extra)\W*)*)(?:s|sm|small)\s*$").unwrap(),
                |c| {
                    -1 - (c[1].chars().filter(|&c| c == 'x' || c == 'X').count() as i32)
                }
            ),
            // Repeated XL (0 repetitions for L)
            (
                Regex::new(r"(?i-u)((?:(?:x|extra)\W*)*)(?:l|lg|large)\s*$").unwrap(),
                |c| {
                    1 + (c[1].chars().filter(|&c| c == 'x' || c == 'X').count() as i32)
                }
            ),
        ];
    }

    for (r, clos) in REGEXES.iter() {
        if let Some(caps) = r.captures(text) {
            let style = &text[0..caps.get(0).unwrap().start()];
            return (style, clos(&caps));
        }
    }

    return (text, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    const shirt: [&str; 16] = [
        "fittEd 4-x-s",
        "fittEd 3-x-s",
        "fittEd x-s",
        "fittEd small",
        "fittEd m",
        "fittEd l",
        "fittEd 1X L ",
        "fittEd 4X L   ",
        "unfittEd xxxx-s",
        "unfittEd 3-x-s",
        "unfittEd xx-s",
        "unfittEd Small",
        "unfittEd m",
        "unfittEd l",
        "unfittEd 1X L ",
        "unfittEd 4X L   ",
    ];

    const shirt_unspaced: [&str; 16] = [
        "fittEd4-x-s",
        "fittEd3-x-s",
        "fittEdx-s",
        "fittEdsmall",
        "fittEdm",
        "fittEdl",
        "fittEd1X L ",
        "fittEd4X L   ",
        "unfittEdxxxx-s",
        "unfittEd3-x-s",
        "unfittEdxx-s",
        "unfittEdSmall",
        "unfittEdm",
        "unfittEdl",
        "unfittEd1X L ",
        "unfittEd4X L   ",
    ];

    const hoodie: [&str; 13] = [
        "Chicks 8",
        "Chicks 10",
        "Chicks 12",
        "Chicks 16",
        "Chicks 18",
        "Dude's Small",
        "Dude's Medium",
        "Dude's Large",
        "Dude's XLarge",
        "Dude's 2XLarge",
        "Dude's 3XLarge",
        "Dude's 4XLarge",
        "Dude's 5XLarge",
    ];

    const hoodie_unspaced: [&str; 13] = [
        "Chicks8",
        "Chicks10",
        "Chicks12",
        "Chicks16",
        "Chicks18",
        "Dude'sSmall",
        "Dude'sMedium",
        "Dude'sLarge",
        "Dude'sXLarge",
        "Dude's2XLarge",
        "Dude's3XLarge",
        "Dude's4XLarge",
        "Dude's5XLarge",
    ];

    fn test_sorting(correct: &[&str]) {
        // TODO there's gotta be a one-liner here right?
        let mut to_sort: Vec<&str> = Vec::with_capacity(correct.len());
        to_sort.extend_from_slice(correct);

        for _ in 1..10 {
            to_sort.shuffle(&mut thread_rng());

            // Sanity check of shuffling, has been run but is not technically true 100% of the time so is commented
            // assert_ne!(to_sort, correct);
            to_sort.sort_by_key(|x| size_sort_key(x));

            assert_eq!(to_sort, correct);
        }
    }

    #[test]
    fn hoodie_2018_keys() {
        assert_eq!(
            hoodie.iter().map(|x| size_sort_key(x)).collect::<Vec<_>>(),
            hoodie
                .iter()
                .map(|x| size_sort_key_old(x))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn hoodie_unspaced_keys() {
        assert_eq!(
            hoodie_unspaced
                .iter()
                .map(|x| size_sort_key(x))
                .collect::<Vec<_>>(),
            hoodie_unspaced
                .iter()
                .map(|x| size_sort_key_old(x))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn shirt_2018_keys() {
        assert_eq!(
            shirt.iter().map(|x| size_sort_key(x)).collect::<Vec<_>>(),
            shirt
                .iter()
                .map(|x| size_sort_key_old(x))
                .collect::<Vec<_>>()
        );
    }


    #[test]
    fn shirt_unspaced_keys() {
        assert_eq!(
            shirt_unspaced.iter().map(|x| size_sort_key(x)).collect::<Vec<_>>(),
            shirt_unspaced
                .iter()
                .map(|x| size_sort_key_old(x))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn hoodie_2018() {
        test_sorting(&hoodie);
    }

    #[test]
    fn shirt_2018() {
        test_sorting(&shirt);
    }
}
