extern crate levenshtein;
use levenshtein::levenshtein;

pub fn count_mirrors(ground: &str, with_smudge: &bool) -> usize {
    let lines = ground.lines().collect::<Vec<&str>>();
    let horizontal = find_horizontal_reflection(&lines, with_smudge);
    let vertical = find_vertical_reflection(ground, with_smudge);
    if !with_smudge {
        if horizontal.is_some() {
            return horizontal.unwrap().0 * 100;
        } else {
            return vertical.and_then(|x| Some(x.0)).unwrap_or(0);
        }
    }

    if horizontal.is_some_and(|x| x.1) {
        return horizontal.unwrap().0 * 100;
    }

    if vertical.is_some_and(|x| x.1) {
        return vertical.unwrap().0;
    }

    return horizontal
        .and_then(|x| Some(x.0 * 100))
        .or_else(|| Some(vertical.unwrap().0))
        .unwrap();
}

fn find_horizontal_reflection(lines: &Vec<&str>, with_smudge: &bool) -> Option<(usize, bool)> {
    for i in 0..(lines.len() - 1) {
        if lines.get(i).eq(&lines.get(i + 1)) {
            if let Some(found_smudge) = is_reflection(&lines, i, with_smudge) {
                if found_smudge {
                    return Some((i + 1, found_smudge));
                }
            }
        }
        if *with_smudge && levenshtein(lines[i], lines[i + 1]) == 1 {
            if let Some(_) = is_reflection(&lines, i, &false) {
                return Some((i + 1, true));
            }
        }
    }

    return None;
}

fn find_vertical_reflection(ground: &str, with_smudge: &bool) -> Option<(usize, bool)> {
    let convert = get_horizontal_lines(ground);
    let horizontal_lines: Vec<&str> = convert.iter().map(|x| x.as_str()).collect();
    return find_horizontal_reflection(&horizontal_lines, with_smudge);
}

fn is_reflection(lines: &Vec<&str>, position: usize, with_smudge: &bool) -> Option<bool> {
    let steps = std::cmp::min(position, lines.len() - position - 2);
    let mut found_smudge = !with_smudge;
    for i in 1..steps + 1 {
        if lines.get(position - i).ne(&lines.get(position + i + 1)) {
            if found_smudge {
                return None;
            }

            let distance = levenshtein(lines[position - i], lines[position + i + 1]);
            if distance == 1 {
                found_smudge = true;
                continue;
            }

            return None;
        }
    }
    return Some(found_smudge);
}

fn get_horizontal_lines(ground: &str) -> Vec<String> {
    let size = ground.find("\n").unwrap();
    let mut lines = vec![];

    for i in 0..size {
        let mut res: Vec<char> = vec![];
        for line in ground.lines() {
            let c = line.chars().nth(i).unwrap();
            res.push(c);
        }
        lines.push(String::from_iter(res));
    }

    return lines;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_reflections() {
        // horizontal
        assert_eq!(count_mirrors(&"..\n.#", &false), 0);
        assert_eq!(count_mirrors(&"..\n.#", &false), 0);
        assert_eq!(count_mirrors(&".#\n.#", &false), 100);
        assert_eq!(count_mirrors(&"..\n.#\n.#", &false), 200);
        assert_eq!(count_mirrors(&"..\n.#\n.#\n..", &false), 200);
        assert_eq!(count_mirrors(&"..\n#.\n.#\n.#", &false), 300);
        assert_eq!(count_mirrors(&"..\n#.\n.#\n.#\n#.", &false), 300);

        // vertical
        assert_eq!(count_mirrors(&"....\n####", &false), 1);
    }

    #[test]
    fn it_crates_horizontal_lines() {
        assert_eq!(
            get_horizontal_lines("....\n####"),
            vec![String::from(".#"); 4]
        )
    }
}
