pub fn extrapolate_next(data: &str) -> i64 {
    let values = data
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut recordings = vec![values.clone()];
    let mut index = 0;
    while !is_final_history(recordings[index].clone()) {
        recordings.push(next_history(recordings[index].clone()));
        index += 1;
    }
    recordings.reverse();
    return recordings
        .iter()
        .fold(0, |acc, history| *history.last().unwrap() + acc);
}

pub fn extrapolate_previous(data: &str) -> i64 {
    let values = data
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut recordings = vec![values.clone()];
    let mut index = 0;
    while !is_final_history(recordings[index].clone()) {
        recordings.push(previous_history(recordings[index].clone()));
        index += 1;
    }
    recordings.reverse();
    return recordings.iter().fold(0, |acc, history| history[0] - acc);
}

fn next_history(history: Vec<i64>) -> Vec<i64> {
    return history[1..]
        .iter()
        .enumerate()
        .map(|(index, value)| *value - history[index])
        .collect();
}

fn previous_history(history: Vec<i64>) -> Vec<i64> {
    return history[0..history.len() - 1]
        .iter()
        .enumerate()
        .map(|(index, value)| history[index + 1] - *value)
        .collect();
}

fn is_final_history(history: Vec<i64>) -> bool {
    history.iter().all(|x| *x == history[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_next_history() {
        assert_eq!(next_history(vec![2, 4, 6]), vec![2, 2]);
        assert_eq!(next_history(vec![0, 2, 8]), vec![2, 6]);
    }

    #[test]
    fn it_computes_previous_history() {
        assert_eq!(previous_history(vec![2, 4, 6]), vec![2, 2]);
        assert_eq!(previous_history(vec![0, 2, 8]), vec![2, 6]);
    }
}
