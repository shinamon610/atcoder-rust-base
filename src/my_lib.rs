// use rand::Rng;

// fn generate_random_an(max_size: usize, min_value: i64, max_value: i64) -> Vec<i64> {
//     let mut rng = rand::thread_rng();
//     let size = rng.gen_range(1, max_size);
//     (0..size)
//         .map(|_| rng.gen_range(min_value, max_value))
//         .collect()
// }
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(0, 0);
    }
}

*/

/// Depth-First Search
/// startから辿れるだけ辿って、その経緯を記録する
/// no_nextで、次のパスがないも場合(currentが探索の最後の頂点になっている場合)にのみ記録するようにしている。
fn dfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<Vec<usize>> {
    fn dfs_recursive(
        graph: &Vec<Vec<usize>>,
        current: usize,
        path: &mut Vec<usize>,
        all_paths: &mut Vec<Vec<usize>>,
    ) {
        path.push(current);
        let mut no_next = true;
        for next in 0..graph.len() {
            if graph[current][next] != 0 && !path.contains(&next) {
                dfs_recursive(graph, next, path, all_paths);
                no_next = false;
            }
        }
        if no_next {
            all_paths.push(path.clone());
        }
        path.pop();
    }

    let mut all_paths = Vec::new();
    let mut path = Vec::new();
    dfs_recursive(graph, start, &mut path, &mut all_paths);
    all_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let graph = vec![
            vec![0, 1, 1, 0],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![0, 1, 1, 0],
        ];
        let start = 0;
        let paths = dfs(&graph, start);
        let expected_paths = vec![
            vec![0, 1, 2, 3],
            vec![0, 1, 3, 2],
            vec![0, 2, 1, 3],
            vec![0, 2, 3, 1],
        ];
        assert_eq!(paths, expected_paths);
    }
}
