use std::cmp::min;
use std::u64;

use proconio::input;

struct SegmentTree<F, T> {
    size: usize,
    tree: Vec<T>,
    element: T,
    eval: F,
}

impl<F: Fn(T, T) -> T, T: Copy + Eq + std::fmt::Debug> SegmentTree<F, T> {
    fn new(max: usize, element: T, eval: F) -> Self {
        let size = max.next_power_of_two();
        Self {
            size,
            tree: vec![element; size * 2],
            element,
            eval,
        }
    }

    fn get(&self, left: usize, right: usize) -> T {
        return self._get(left, right, 0, 0, self.size);
    }

    fn _get(&self, left: usize, right: usize, now_pos: usize, l: usize, r: usize) -> T {
        if r <= left || right <= l {
            self.element
        } else if left <= l && r <= right {
            self.tree[now_pos]
        } else {
            (self.eval)(
                self._get(left, right, now_pos * 2 + 1, l, (l + r) / 2),
                self._get(left, right, now_pos * 2 + 2, (l + r) / 2, r),
            )
        }
    }

    pub fn update(&mut self, index: usize, value: T) {
        let mut i = (self.size-1) + index;
        self.tree[i] = value;

        while i > 0 {
            i = (i-1) / 2;
            self.tree[i] = (self.eval)(self.tree[i*2+1], self.tree[i*2+2]);
        }
    }
}

impl<F, T: std::fmt::Debug> std::fmt::Debug for SegmentTree<F, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SegmentTree{:?}", self.tree)
    }
}

fn main() {
    input!{
        N: usize,
        Q: usize,
        A: [u64; N],
        TXY: [(usize, usize, u64); Q],
    }

    let mut segtree = SegmentTree::new(N, u64::MAX, |a, b| min(a, b));
    for i in 0..A.len() {
        segtree.update(i, A[i]);
    }

    let mut anss = vec!{};
    for (t, x, y) in TXY {
        if t == 1 {
            segtree.update(x, y);
        } else if t == 2 {
            let minval = segtree.get(x as usize, y as usize);
            anss.push(minval);
        }
    }

    for ans in anss {
        println!("{}", ans);
    }
}
