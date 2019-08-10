use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn size_sort_key(text: &str) -> (&str, i32) {
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
            (Regex::new(r"(?i)(\d+)\s*$").unwrap(), |c| {
                let n: u16 = c[1].parse().unwrap();
                n.into()
            }),
            // Medium
            (Regex::new(r"(?i)(?:m|med|medium)\s*$").unwrap(), |_| {
                0
            }),
            // Numeric XS
            (
                Regex::new(r"(?i)(\d+)\W*(?:x|extra)\W*(?:s|sm|small)\s*$").unwrap(),
                |c| {
                    // https://turbo.fish/
                    -1 - (c[1].parse::<u16>().unwrap() as i32)
                }
            ),
            // Numeric XL
            (
                Regex::new(r"(?i)(\d+)\W*(?:x|extra)\W*(?:l|lg|large)\s*$").unwrap(),
                |c| {
                    1 + (c[1].parse::<u16>().unwrap() as i32)
                }
            ),
            // Repeated XS (0 repetitions for S)
            (
                Regex::new(r"(?i)((?:(?:x|extra)\W*)*)(?:s|sm|small)\s*$").unwrap(),
                |c| {
                    -1 - (c[1].chars().filter(|&c| c == 'x' || c == 'X').count() as i32)
                }
            ),
            // Repeated XL (0 repetitions for L)
            (
                Regex::new(r"(?i)((?:(?:x|extra)\W*)*)(?:l|lg|large)\s*$").unwrap(),
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
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    fn test_sorting(correct: &[&str]) {
        // TODO there's gotta be a one-liner here right?
        let mut to_sort: Vec<&str> = Vec::with_capacity(correct.len());
        to_sort.extend_from_slice(correct);

        for _ in 1..10 {
            to_sort.shuffle(&mut thread_rng());

            // Sanity check of shuffling, has been run but is not technically true 100% of the time so is commented
            // assert_ne!(to_sort, correct);
            to_sort.sort_by_key(|x| super::size_sort_key(x));

            assert_eq!(to_sort, correct);
        }
    }

    #[test]
    fn hoodie_2018() {
        test_sorting(&[
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
        ]);
    }

    #[test]
    fn shirt_2018() {
        test_sorting(&[
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
            "unfittEd small",
            "unfittEd m",
            "unfittEd l",
            "unfittEd 1X L ",
            "unfittEd 4X L   ",
        ]);
    }
}
