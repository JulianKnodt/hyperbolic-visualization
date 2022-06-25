use std::collections::HashSet;
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
    pub fn depth_first_iter<const order: TraversalOrder> (
        &self,
        from: DAGID,
    ) -> DepthFirstIter<'_, T, order> {
        DepthFirstIter {
            tree: self,
            parents: vec![(from, 0)],
            visited: HashSet::new(),
        }
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
    pub depth: usize,
    pub child_num: usize,
    pub dagid: DAGID,
}

impl<'a, T, const order: TraversalOrder> Iterator for DepthFirstIter<'a, T, order>
where
    T: Hash + Eq,
{
    type Item = DFOut;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let depth = self.parents.len();
            let (curr, child_num) = self.parents.last_mut()?;
            let dagid = *curr;

            match self.tree.neighbors(*curr).get(*child_num) {
                Some(child_idx) => {
                    let original_child_num = *child_num;
                    *child_num += 1;
                    if self.visited.insert(*child_idx) {
                        self.parents.push((*child_idx, 0));
                    }
                    if order == TraversalOrder::PreOrder && original_child_num == 0 {
                        return Some(DFOut {
                            depth,
                            child_num: original_child_num,
                            dagid,
                        });
                    }
                }
                None => {
                    assert!(self.parents.pop().is_some());
                    if order == TraversalOrder::PostOrder {
                        let child_num = self
                            .parents
                            .last()
                            .map(|(_, ci)| *ci)
                            .unwrap_or(1)
                            .saturating_sub(1);
                        return Some(DFOut {
                            depth: self.parents.len(),
                            child_num,
                            dagid,
                        });
                    }
                }
            }
        }
    }
}
