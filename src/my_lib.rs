use std::{
    cmp::{Ordering, Reverse},
    collections::HashSet,
};

fn eratosthenes(n: i32) -> Vec<bool> {
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n as usize {
        if is_prime[i] {
            let mut j = 2 * i;
            while j <= n as usize {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

fn euclidean_algorithm(mut a: i32, mut b: i32) -> i32 {
    while a != 0 && b != 0 {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    return if a == 0 { b } else { a };
}

fn mod_pow(a: i128, b: i128, mod_n: i128) -> i128 {
    let mut aas = vec![a];
    for _ in 0..30 {
        aas.push((aas.last().unwrap() * aas.last().unwrap()) % mod_n);
    }
    let binary_b: Vec<char> = format!("{:b}", b).chars().rev().collect();
    let mut res = 1;
    for (i, bb) in binary_b.iter().enumerate() {
        if *bb == '1' {
            res = (res * aas[i]) % mod_n;
        }
    }
    return res;
}

fn mod_div(a: i128, b: i128, mod_n: i128) -> i128 {
    return (a * mod_pow(b, mod_n - 2, mod_n)) % mod_n;
}

fn mod_factorial(mut a: i128, mod_n: i128) -> i128 {
    let mut res = 1;
    while a > 1 {
        res = (res * a) % mod_n;
        a -= 1;
    }
    return res;
}

// mod_n=1000000007
fn mod_ncr(n: i128, r: i128, mod_n: i128) -> i128 {
    mod_div(
        mod_factorial(n, mod_n),
        mod_factorial(r, mod_n) * mod_factorial(n - r, mod_n),
        mod_n,
    )
}
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

fn to_yes_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}

// bell_splitで使ってるから、持ってきただけ
fn xor(groups: Vec<Vec<u64>>) -> u64 {
    return groups
        .into_iter()
        .map(|ss| ss.into_iter().sum::<u64>())
        .fold(0, |acc, x| acc ^ x);
}

// anを1からn個のgroupに分割する方法。
// ここでは、分割したあと、xorしてそれを集合を返している。genericに書き直すとTLEするからもうこのまま持ってきた。
fn bell_split(an: &Vec<u64>, depth: usize, groups: &mut Vec<Vec<u64>>) -> HashSet<u64> {
    if depth == an.len() {
        let mut hs = HashSet::new();
        hs.insert(xor(groups.clone()));
        return hs;
    }
    let mut res = HashSet::new();
    let n = groups.len().clone();

    for i in 0..=groups.len() {
        if i == n {
            groups.push(vec![an[depth]]);
        } else {
            groups[i].push(an[depth]);
        }
        res.extend(bell_split(&an, depth + 1, groups));
        if i == n {
            groups.pop();
        } else {
            groups[i].pop();
        }
    }
    return res;
}

// priority_queueは、BinaryHeapを使う。
// max-heapなので、デフォルトだと、大きいものが優先的に出てくる。
// 以下実装は、タプルの第二要素を対象としてpriority_queueを実装するためのやつ
// https://qiita.com/tinsep19/items/0c40770969f8c185ffa6 使い方はここ参照
struct Priority<T, P: Ord + Copy>(T, P);

impl<T, P: Ord + Copy> Priority<T, P> {
    fn new(obj: T, priority: P) -> Priority<T, P> {
        Priority(obj, priority)
    }
    fn rev(obj: T, priority: P) -> Reverse<Priority<T, P>> {
        Reverse(Priority::new(obj, priority))
    }
}

impl<T, P: Ord + Copy> Eq for Priority<T, P> {}
impl<T, P: Ord + Copy> PartialEq for Priority<T, P> {
    fn eq(&self, other: &Priority<T, P>) -> bool {
        self.1.eq(&other.1)
    }
}
impl<T, P: Ord + Copy> PartialOrd for Priority<T, P> {
    fn partial_cmp(&self, other: &Priority<T, P>) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}
impl<T, P: Ord + Copy> Ord for Priority<T, P> {
    fn cmp(&self, other: &Priority<T, P>) -> Ordering {
        self.1.cmp(&other.1)
    }
}

// ac_libraryの使い方サンプル
/*
==Disjoint Set Union==

use proconio::{input, fastout};
use ac_library::Dsu;

#[fastout]
fn main() {
    // 入力例
    // n=5, m=3, q=3
    // 辺: (0,1), (2,3), (3,4)
    // クエリ:
    //   same(0,1)? -> yes
    //   same(1,2)? -> no
    //   same(2,4)? -> yes
    input! {
        n: usize,   // 頂点数
        m: usize,   // 辺数
        q: usize,   // クエリ数
        edges: [(usize, usize); m], // 辺の一覧
        queries: [(usize, usize); q], // クエリ頂点ペア
    }

    let mut dsu = Dsu::new(n);

    // 全ての辺でunion操作を行い、連結成分を確定
    for (u, v) in edges {
        dsu.merge(u, v);
    }

    // クエリに応える
    for (x, y) in queries {
        if dsu.same(x, y) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
*/

/*
use std::ops::{Bound::*, RangeBounds};
use ac_library::{LazySegtree, Monoid, MapMonoid};

// 区間和を管理するMonoid
struct SumMonoid;

impl Monoid for SumMonoid {
    type S = i64;          // 要素の型
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(&a: &Self::S, &b: &Self::S) -> Self::S {
        a + b
    }
}

// 区間加算ができるMapMonoidの実装
struct AddMap;

impl MapMonoid for AddMap {
    type M = SumMonoid; // モノイドとしてSumMonoidを利用
    type F = i64;       // 区間に加える値の型

    fn identity_map() -> Self::F {
        0 // 作用しない（加算しない）時の値
    }

    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        x + f // xにfを加算する
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g // 作用の合成は加算の合成
    }
}

fn main() {
    // 例:
    // 初期列: a = [1, 2, 3, 4, 5]
    let a = vec![1, 2, 3, 4, 5];
    let mut seg = LazySegtree::<AddMap>::from(a);

    // 区間和の取得: prod(range)
    // [1,4)の区間和 = a[1]+a[2]+a[3] = 2+3+4=9
    let sum_1_4 = seg.prod(1..4);
    println!("sum of [1,4) = {}", sum_1_4); // 出力: 9

    // 区間加算: apply_range(range, value)
    // [0,3) に +10 を加算 -> [1+10, 2+10, 3+10, 4, 5] = [11,12,13,4,5]
    seg.apply_range(0..3, 10);

    // 再び区間和取得:
    // [1,4)の区間和 = 12 + 13 + 4 = 29
    let sum_1_4_after = seg.prod(1..4);
    println!("sum of [1,4) after update = {}", sum_1_4_after); // 出力: 29

    // 単一要素の取得: get(pos)
    // a[0]は現在11になっているはず
    let val_0 = seg.get(0);
    println!("value at index 0 = {}", val_0); // 出力: 11
}

*/
