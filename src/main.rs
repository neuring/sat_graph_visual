use anyhow::{anyhow, Context};
use graph::Graph;
use macroquad::prelude::*;

mod dimacs;
mod graph;
mod physics;

const CAM_SPEED: f32 = 2000.;

fn rand_vec2() -> Vec2 {
    let x = rand::gen_range(-1., 1.);
    let y = rand::gen_range(-1., 1.);
    vec2(x, y)
}

struct ViewState {
    camera: Camera2D,
    zoom: f32,
}

impl ViewState {
    fn new() -> Self {
        let camera = Camera2D::default();
        let zoom = 0.0005;

        Self { camera, zoom }
    }

    fn update_cam(&mut self) {
        if is_key_down(KeyCode::PageUp) {
            self.zoom *= 1. - (1. * get_frame_time());
        }
        if is_key_down(KeyCode::PageDown) {
            self.zoom *= 1. + (1. * get_frame_time());
        }

        let mut direction = vec2(0., 0.);
        if is_key_down(KeyCode::Down) {
            direction.y -= 1.;
        }
        if is_key_down(KeyCode::Up) {
            direction.y += 1.;
        }
        if is_key_down(KeyCode::Left) {
            direction.x -= 1.;
        }
        if is_key_down(KeyCode::Right) {
            direction.x += 1.;
        }

        // I can't do linear algebra and this works OK!!
        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_pos0 = self.camera.screen_to_world(mouse_pos);
        let mouse_pos1 = self.camera.screen_to_world(mouse_pos + vec2(1.0, 0.));
        let zoom_coeff = mouse_pos0.distance(mouse_pos1);
        self.camera.target += direction * CAM_SPEED * get_frame_time() * zoom_coeff;

        //let middle = (screen_width() / 2., screen_height() / 2.);
        //self.camera.target = self.camera.screen_to_world(middle.into());

        let h = screen_width() / screen_height();
        self.camera.zoom = vec2(1., h) * self.zoom;
        set_camera(&self.camera);
    }
}

#[derive(Default)]
struct MouseState {
    drag_start: Option<Vec2>,
}

impl MouseState {
    fn update(&mut self, view: &ViewState) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let pos = view.camera.screen_to_world(mouse_position().into());
            self.drag_start = Some(pos);
        }
    }
}

fn draw_line(view_state: &mut ViewState, start: Vec2, end: Vec2, color: Color) {
    let r = 1. / view_state.zoom;
    macroquad::prelude::draw_line(start.x, start.y, end.x, end.y, r * 0.005, color);
}

fn draw_circle(view_state: &mut ViewState, pos: Vec2, r: f32, color: Color) {
    let r = 1. / view_state.zoom * r;
    macroquad::prelude::draw_circle(pos.x, pos.y, r, color);
}

#[macroquad::main("Graph")]
async fn main() -> anyhow::Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Missing input file argument"))?;
    let input_dimacs = std::fs::read_to_string(input_file).context("Couldn't read input file?")?;
    let input_formula = dimacs::Dimacs::parse(&input_dimacs).context("Invalid dimacs format.")?;

    let mut variable_nodes = Vec::new();
    let mut clause_nodes = Vec::new();

    let mut graph = Graph::default();

    for _cls in &input_formula {
        clause_nodes.push(graph.add_node(rand_vec2() * 1000.));
    }

    let max_var = input_formula
        .iter()
        .flat_map(|cls| cls.iter().map(|i| i.abs()))
        .max()
        .unwrap();

    for _var in 0..max_var {
        variable_nodes.push(graph.add_node(rand_vec2() * 1000.));
    }

    for (cls_id, cls) in input_formula.iter().enumerate() {
        for lit in cls {
            let var = lit.abs() as usize;

            let cls_node = clause_nodes[cls_id];
            let var_node = variable_nodes[var - 1];
            graph.add_edge(cls_node, var_node);
        }
    }

    let mut view_state = ViewState::new();

    loop {
        clear_background(WHITE);

        view_state.update_cam();

        graph.update_positions(get_frame_time() * 3.);

        for (id, node) in graph.nodes().enumerate() {
            for &neighbor_id in &node.neighbors {
                if neighbor_id < id {
                    continue;
                }
                let neighbor = &graph[neighbor_id];

                draw_line(&mut view_state, node.pos, neighbor.pos, BLACK);
            }
        }

        for &cls_node in &clause_nodes {
            let cls_node = &graph[cls_node];
            draw_circle(&mut view_state, cls_node.pos, 0.01, BLUE);
        }

        for &var_node in &variable_nodes {
            let var_node = &graph[var_node];
            draw_circle(&mut view_state, var_node.pos, 0.01, ORANGE);
        }

        let pos = mouse_position().into();
        let pos = view_state.camera.screen_to_world(pos);
        draw_circle(&mut view_state, pos, 0.02, GREEN);

        next_frame().await
    }
}
