pub fn arrangements(data: &str) -> usize {
    if !data.contains('?') {
        return 0;
    }

    let (record, groups_str) = data.split_once(" ").unwrap();
    let groups = groups_str
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    return permutations(String::from(record))
        .iter()
        .filter(|x| is_valid(x.as_str(), &groups))
        .count();
}

fn permutations(line: String) -> Vec<String> {
    let unknown = line.find('?');
    return match unknown {
        None => vec![line],
        Some(_) => {
            let with_dot = permutations(line.replacen('?', ".", 1));
            let with_spring = permutations(line.replacen('?', "#", 1));
            return vec![with_dot, with_spring]
                .iter()
                .flatten()
                .map(|x| String::from(x))
                .collect::<Vec<String>>();
        }
    };
}

fn is_valid(line: &str, groups: &Vec<usize>) -> bool {
    let mut line_groups: Vec<usize> = vec![];
    let mut current_group_size = 0;
    for c in line.chars() {
        if c.eq(&'#') {
            current_group_size += 1
        }

        if c.eq(&'.') {
            if current_group_size > 0 {
                line_groups.push(current_group_size);
            }
            current_group_size = 0;
        }
    }

    if current_group_size > 0 {
        line_groups.push(current_group_size);
    }

    return line_groups.eq(groups);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_broken_records() {
        assert_eq!(arrangements("# 1"), 0);
        assert_eq!(arrangements(".# 1"), 0);
        assert_eq!(arrangements(".#. 1"), 0);
    }

    #[test]
    fn it_checks_valid_groups() {
        assert_eq!(is_valid(".#.", &vec![1]), true);
        assert_eq!(is_valid(".##.#", &vec![2, 1]), true);
        assert_eq!(is_valid("#.###....#", &vec![1, 3, 1]), true);
    }

    #[test]
    fn it_creates_permutations() {
        assert_eq!(
            permutations(String::from(".?.")),
            vec![String::from("..."), String::from(".#.")]
        );
        assert_eq!(
            permutations(String::from(".??.")),
            vec![
                String::from("...."),
                String::from("..#."),
                String::from(".#.."),
                String::from(".##.")
            ]
        );
    }
}
