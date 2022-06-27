use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct DAG<T> {
    elements: Vec<T>,
    edges: Vec<Vec<DAGID>>,
}

/// Alias for usize for an item inserted into a tree.
pub type DAGID = usize;

impl<T> DAG<T> {
    pub fn new() -> Self {
        Self {
            elements: vec![],
            edges: vec![],
        }
    }
    pub fn num_nodes(&self) -> usize {
        self.elements.len()
    }
    pub fn num_edges(&self) -> usize {
        self.edges.iter().map(|e| e.len()).sum()
    }
    pub fn insert(&mut self, v: T) -> DAGID {
        let idx = self.elements.len();
        self.elements.push(v);
        self.edges.push(Vec::new());
        idx
    }
    pub fn insert_edge(&mut self, from: DAGID, to: DAGID) {
        self.edges[from].push(to);
    }
    pub fn neighbors(&self, of: DAGID) -> &[DAGID] {
        &self.edges[of]
    }
    pub fn get(&self, id: DAGID) -> &T {
        &self.elements[id]
    }
    pub fn depth_first_iter<const order: TraversalOrder>(
        &self,
        from: DAGID,
    ) -> DepthFirstIter<'_, T, order> {
        DepthFirstIter {
            tree: self,
            parents: vec![(from, 0)],
            visited: HashSet::new(),
        }
    }
    pub fn from_pairs(pairs: impl IntoIterator<Item = (T, T)>) -> Self
    where
        T: Hash + Eq + Clone,
    {
        let mut seen = HashMap::new();
        let mut out = Self::new();
        for (src, dst) in pairs {
            let src_id = *seen
                .entry(src)
                .or_insert_with_key(|src| out.insert(src.clone()));
            let dst_id = *seen
                .entry(dst)
                .or_insert_with_key(|dst| out.insert(dst.clone()));
            out.insert_edge(src_id, dst_id);
        }
        out
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TraversalOrder {
    PreOrder,
    PostOrder,
}

#[derive(Debug, Clone)]
pub struct DepthFirstIter<'a, T, const order: TraversalOrder> {
    tree: &'a DAG<T>,
    /// Stack of previous parents and which child it's currently visiting.
    parents: Vec<(DAGID, usize)>,
    visited: HashSet<DAGID>,
}

#[derive(Debug, Clone, Copy)]
pub struct DFOut {
    pub dagid: DAGID,

    pub depth: usize,
    /// Parent's DAGID, and index of this child is this
    pub parent_ref: Option<(DAGID, usize)>,
}

impl<'a, T, const order: TraversalOrder> Iterator for DepthFirstIter<'a, T, order>
where
    T: Hash + Eq,
{
    type Item = DFOut;
    fn next(&mut self) -> Option<Self::Item> {
        while self.parents.len() > 0 {
            let depth = self.parents.len() - 1;
            let parent_ref: Option<(DAGID, usize)> = self
                .parents
                .len()
                .checked_sub(2)
                .and_then(|idx| self.parents.get(idx))
                .map(|(parent, cn)| (*parent, *cn - 1));

            let (curr, child_num) = self.parents.last_mut()?;
            let dagid = *curr;

            match self.tree.neighbors(*curr).get(*child_num) {
                Some(child_id) => {
                    let original_child_num = std::mem::replace(child_num, *child_num + 1);
                    let unseen = self.visited.insert(*child_id);
                    if unseen {
                        self.parents.push((*child_id, 0));
                    }
                    if order == TraversalOrder::PreOrder && unseen && original_child_num == 0 {
                        return Some(DFOut {
                            depth,
                            parent_ref,
                            dagid,
                        });
                    }
                }
                None => {
                    let original_child_num = self.parents.pop().unwrap().1;
                    if order == TraversalOrder::PostOrder {
                        return Some(DFOut {
                            depth,
                            parent_ref,
                            dagid,
                        });
                    } else if original_child_num == 0 {
                        return Some(DFOut {
                            depth,
                            parent_ref,
                            dagid,
                        });
                    }
                }
            }
        }
        None
    }
}

#[test]
fn test_linked_list() {
    let pairs = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
    let dag = DAG::from_pairs(pairs);
    let iter = dag.depth_first_iter::<{ TraversalOrder::PreOrder }>(0);
    for (i, v) in iter.enumerate() {
        assert_eq!(i, v.depth);
        assert_eq!(i, v.dagid);
    }
}

#[test]
fn test_simple_tree() {
    let pairs = [(0, 1), (1, 2), (1, 3), (3, 4), (3, 5), (5, 6)];
    let dag = DAG::from_pairs(pairs);
    let iter = dag.depth_first_iter::<{ TraversalOrder::PreOrder }>(0);
    for v in iter {
        //println!("{:?}", v);
    }
}

#[test]
fn test_small_cycle() {
    let pairs = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 0)];
    let dag = DAG::from_pairs(pairs);
    let iter = dag.depth_first_iter::<{ TraversalOrder::PreOrder }>(0);
    for v in iter {
        //println!("{:?}", v);
    }
}
