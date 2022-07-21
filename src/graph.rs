use kdtree::KdTree;
use macroquad::prelude::Vec2;

pub type NodeId = usize;

#[derive(Debug)]
pub struct Node {
    pub pos: Vec2,
    pub tmp_pos: Vec2,
    pub vel: Vec2,
    pub neighbors: Vec<NodeId>,
}

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn add_node(&mut self, pos: Vec2) -> NodeId {
        let node_id = self.nodes.len();
        self.nodes.push(Node {
            pos,
            tmp_pos: Vec2::default(),
            vel: Vec2::default(),
            neighbors: Vec::new(),
        });
        node_id
    }

    pub fn add_edge(&mut self, a: NodeId, b: NodeId) {
        self.nodes[a].neighbors.push(b);
        self.nodes[b].neighbors.push(a);
    }

    pub fn get(&self, a: NodeId) -> Option<&Node> {
        self.nodes.get(a)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> + '_ {
        self.nodes.iter()
    }

    pub fn node_size(&self) -> usize {
        self.nodes.len()
    }

    pub fn build_kdtree(&self) -> KdTree<f32, NodeId, [f32; 2]> {
        let mut kdtree = KdTree::new(2);

        for (node_id, node) in self.nodes.iter().enumerate() {
            kdtree
                .add(node.pos.as_ref().to_owned(), node_id)
                .unwrap_or_else(|_| panic!("couldn't add node {node:?}"))
        }

        kdtree
    }
}

impl std::ops::Index<NodeId> for Graph {
    type Output = Node;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index]
    }
}

impl std::ops::IndexMut<NodeId> for Graph {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
