use std::collections::HashMap;

type Cache = HashMap<(usize, usize), usize>;

pub fn arrangements(data: &str) -> usize {
    let mut cache: Cache = HashMap::new();
    let (record, groups_str) = data.split_once(" ").unwrap();

    let groups = groups_str
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    return count_arrangements(&record.chars().collect::<Vec<char>>(), &groups, &mut cache);
}

pub fn arrangements_long(data: &str) -> usize {
    let mut cache: Cache = HashMap::new();
    let (record, groups_str) = data.split_once(" ").unwrap();

    let groups = groups_str
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
        .repeat(5);

    return count_arrangements(
        &[record; 5].join("?").chars().collect::<Vec<char>>(),
        &groups,
        &mut cache,
    );
}

fn count_arrangements(mut chars: &[char], groups: &[usize], cache: &mut Cache) -> usize {
    while let ['.', rest @ ..] = chars {
        chars = rest;
    }

    if chars.is_empty() {
        return usize::from(groups.is_empty());
    }

    if groups.is_empty() {
        return usize::from(chars.iter().all(|s| s.ne(&'#')));
    }

    let key = (chars.len(), groups.len());

    if let Some(count) = cache.get(&key) {
        return *count;
    }

    if chars.len() < groups.iter().sum::<usize>() + groups.len() - 1 {
        cache.insert(key, 0);
        return 0;
    }

    if chars[0] == '?' {
        let count_with_dot = count_arrangements(&chars[1..], groups, cache);
        let count_with_hash = match fits_group(chars, groups[0]) {
            Some(next_chars) => {
                count_arrangements(next_chars.get(1..).unwrap_or_default(), &groups[1..], cache)
            }
            None => 0,
        };

        cache.insert(key, count_with_dot + count_with_hash);
        return count_with_dot + count_with_hash;
    }

    let res = match fits_group(chars, groups[0]) {
        Some(next_chars) => {
            count_arrangements(next_chars.get(1..).unwrap_or_default(), &groups[1..], cache)
        }
        None => 0,
    };
    cache.insert(key, res);
    return res;
}

fn fits_group(mut chars: &[char], size: usize) -> Option<&[char]> {
    for _ in 0..size {
        if let ['#' | '?', rest @ ..] = chars {
            chars = rest;
        } else {
            return None;
        }
    }

    if chars.first() == Some(&'#') {
        return None;
    } else {
        return Some(chars);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fits_group() {
        assert_eq!(fits_group(&['#', '?'], 1).unwrap(), &vec!['?']);
        assert_eq!(fits_group(&['?', '?'], 2).unwrap(), &vec![]);
        assert_eq!(fits_group(&['#', '?'], 3), None);
        assert_eq!(fits_group(&['.', '?'], 1), None);
        assert_eq!(fits_group(&['#', '?', '#'], 2), None);
    }

    #[test]
    fn it_counts_arrangements() {
        assert_eq!(count_arrangements(&['?'], &[1], &mut HashMap::new()), 1);
    }
}
