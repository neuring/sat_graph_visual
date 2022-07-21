use macroquad::prelude::{vec2, Vec2};

use crate::graph::Graph;

pub struct Physics {
    pub spring_force: f32,
    pub repell_force: f32,
    pub frame_multiplier: f32,
}

pub fn update_positions(config: &Physics, graph: &mut Graph, frametime: f32) {
    const EQUILIBRIUM_LEN: f32 = 4.;
    const STIFFNESS: f32 = 1.;
    const AIR_RESISTANCE: f32 = 0.8;

    //let kdtree = graph.build_kdtree();

    for node_id in 0..graph.nodes.len() {
        let node = &graph.nodes[node_id];

        // Repell from every other node.
        let mut repell_vel = vec2(0., 0.);
        for other_node_id in 0..graph.nodes.len() {
            let other_node = &graph[other_node_id];

            if other_node_id == node_id {
                continue;
            }

            let squared_distance = node.pos.distance_squared(other_node.pos);

            let force = 5. / f32::max(squared_distance.sqrt(), 0.0000001);
            if force < 0.001 {
                break;
            }

            let other_node = &graph.nodes[other_node_id];
            let dir = -(other_node.pos - node.pos)
                .try_normalize()
                .unwrap_or_default();
            repell_vel += dir * force;
        }

        let mut spring_vel = vec2(0., 0.);
        // Spring force for every neighbor
        for &neighbor_id in &node.neighbors {
            let neighbor = &graph.nodes[neighbor_id];

            let distance = node.pos.distance(neighbor.pos);
            let delta = distance - EQUILIBRIUM_LEN;
            let force = STIFFNESS * delta;

            let dir = (neighbor.pos - node.pos)
                .try_normalize()
                .unwrap_or_default();

            spring_vel += dir * force;
        }

        let vec_acc = repell_vel * config.repell_force + spring_vel * config.spring_force;
        let vec_acc = vec_acc.clamp_length_max(1000.);

        let node = &mut graph.nodes[node_id];
        node.vel += vec_acc;
        node.vel *= AIR_RESISTANCE;

        node.tmp_pos = node.pos + node.vel * frametime * config.frame_multiplier;
    }

    let avg_pos = graph.nodes.iter().map(|node| &node.pos).sum::<Vec2>() / graph.node_size() as f32;

    for node in &mut graph.nodes {
        std::mem::swap(&mut node.pos, &mut node.tmp_pos);
        node.pos -= avg_pos;
        //draw_line(
        //    node.pos.x,
        //    node.pos.y,
        //    (node.pos + node.vel).x,
        //    (node.pos + node.vel).y,
        //    1.,
        //    RED,
        //);
    }
}
