use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

const OCEAN_BLUE: Color = color_u8!(0x33, 0x66, 0xcc, 0xff);    // Wave trough

struct DebounceToggle<F: Fn() -> bool>(F, usize);

impl<F: Fn() -> bool> DebounceToggle<F> {
    fn new(f: F) -> DebounceToggle<F> {
        DebounceToggle(f, 0)
    }
    fn get(&mut self) -> bool {
        let DebounceToggle(f, ref mut state) = self;

        *state = match (*state, f()) {
            (0, true) => 1,
            (1, false) => 2,
            (2, true) => 3,
            (3, false) => 0,
            (_, _) => *state,
        };

        *state == 2
    }
}


fn final_color(x: f32, gradient: (Color, Color)) -> Color {
    let x = x.clamp(0.0, 1.0);
    let r = gradient.0.r*(1.0-x) + gradient.1.r*x;
    let g = gradient.0.g*(1.0-x) + gradient.1.g*x;
    let b = gradient.0.b*(1.0-x) + gradient.1.b*x;
    let a = 1.0;
    Color::new(r, g, b, a)
}

#[derive(Clone)]
struct CellGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize
}


impl<T: Clone + Copy> CellGrid<T> {
    fn get(&self, x: usize, y: usize) -> T {
        let i = y * self.width + x;
        self.data[i]
    }

    fn set(&mut self, x: usize, y: usize, cell_data: T) {
        let i = y * self.width + x;
        self.data[i] = cell_data;
    }

    fn new(width: usize, height: usize, cell_data: T) -> Self {
        CellGrid { data: vec![cell_data; height * width], 
                    width: width, 
                    height: height }
    }
}


impl CellGrid<f32> {
    fn bytes(&self, gradient: (Color, Color)) -> Vec<u8> {
        let mut bytes = Vec::new();
        for x in &self.data {
            
            let c = final_color(*x, gradient);
            bytes.push((c.r*255.0) as u8);
            bytes.push((c.g*255.0) as u8);
            bytes.push((c.b*255.0) as u8);
            bytes.push((c.a*255.0) as u8);
        }
        bytes
    }
}


fn screen_dims(longest_side: usize) -> (usize, usize) {
    if screen_width() > screen_height() {
        let shortest_side = screen_height() / screen_width() * longest_side as f32;
        (longest_side, shortest_side as usize)
    } else {
        let shortest_side = screen_width() / screen_height() * longest_side as f32;
        (shortest_side as usize, longest_side)
    }
}

#[macroquad::main("cellular-automata")]
async fn main() {
    let (w, h) = screen_dims(400);

    let mut cells = CellGrid::new(w, h, 0f32);
    for x in 1..cells.width - 1 {
        for y in 1..cells.height - 1 {
            if rand::gen_range::<i32>(0, 500) == 0 {
                cells.set(x, y, 0.8);
            }
        }
    }

    let mut fpss: Vec<i32> = Vec::new();
    let mut uiopen = DebounceToggle::new(|| is_key_down(KeyCode::Space));
    let mut spawnprob = 1000f32;
    let mut dampening = 0.98;
    let mut transfer = 1.5;
    let mut theme = (OCEAN_BLUE, WHITE);
    loop {
        let mut cells_new = cells.clone();

        let (m1, m2) = mouse_position();
        let (c1, c2) = (screen_width() as usize / w, screen_height() as usize / h);
        let m1 = (m1 as usize).clamp(0, c1*w - 1) / c1;
        let m2 = (m2 as usize).clamp(0, c2*w - 1) / c2;

        let mpos = Vec2::new(m1 as f32, m2 as f32);
        show_mouse(false);

        
        if uiopen.get() {
            show_mouse(true);
            widgets::Window::new(hash!(), vec2(100., 100.), vec2(300., 200.))
                .label("Options")
                .ui(&mut *root_ui(), |ui| {
                    ui.label(None, "Parameters");
                    ui.slider(hash!(), "spawnprob", 500f32..10000f32, &mut spawnprob);
                    ui.slider(hash!(), "dampening", 0.9f32..1.0f32, &mut dampening);
                    ui.slider(hash!(), "transfer", 0.0f32..2.0f32, &mut transfer);
                    ui.label(None, "Themes");
                    if ui.button(None, "Ocean") {
                        theme = (OCEAN_BLUE, WHITE);
                    }
                    ui.same_line(0.);
                    if ui.button(None, "Void") {
                        theme = (BLACK, WHITE);
                    }
                    ui.same_line(0.);
                    if ui.button(None, "Paper") {
                        theme = (WHITE, BLACK);
                    }
                    ui.same_line(0.);
                    if ui.button(None, "Matrix") {
                        theme = (BLACK, LIME);
                    }
                    });
        }

        for x in 0..cells.width {
            for y in 0..cells.height {
                let b = cells.get(x, y);
                let cellpos = Vec2::new(x as f32, y as f32);
                let diffpos = cellpos - mpos;

                let donation = rand::gen_range::<f32>(0.0, transfer*b);
                let split_max = diffpos.y.abs() / (diffpos.x.abs() + diffpos.y.abs() + std::f32::EPSILON);
                // println!("{split_max}");
                // let donation_split = rand::gen_range::<f32>(0.0, 1.0);
                let v_donation = split_max * donation;
                let h_donation = (1.0 - split_max) * donation;
                // let h_donation = rand::gen_range::<f32>(0.0, 0.75*b);
                // let v_donation = rand::gen_range::<f32>(0.0, 0.75*b);
                // let donation = h_donation + v_donation;
                let dx: i32 = if diffpos.x < 0.0 {1} else {-1};
                let dy: i32 = if diffpos.y < 0.0 {1} else {-1};
                cells_new.set(x, y, cells_new.get(x, y) - donation);
                if x > 0 && x < cells.width - 1 && y > 0 && y < cells.height - 1 {
                    cells_new.set(x, (y as i32 + dy) as usize, cells_new.get(x, (y as i32 + dy) as usize) + v_donation * dampening);
                    cells_new.set((x as i32 + dx) as usize, y, cells_new.get((x as i32 + dx) as usize, y) + h_donation * dampening);
                }
            }
        }

        for x in 1..cells.width - 1 {
            for y in 1..cells.height - 1 {
                if rand::gen_range::<i32>(0, spawnprob as i32) == 0 {
                    cells_new.set(x, y, 0.8);
                }
            }
        }

        let texture = Texture2D::from_rgba8(cells_new.width as u16, cells_new.height as u16, &cells_new.bytes(theme));
        texture.set_filter(FilterMode::Nearest);
        
        draw_texture_ex(texture, 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(screen_width(), screen_height())), ..Default::default()});
        cells = cells_new;

        fpss.push(get_fps());
        let l = fpss.len().saturating_sub(10);
        let fps_window = &fpss[l..];
        let fps = fps_window.iter().sum::<i32>() as f32 / fps_window.len() as f32;
        println!("{:?}", fps);
        if is_key_down(KeyCode::Enter){
            let (w, h) = screen_dims(400);
            cells = CellGrid::new(w, h, 0f32);
            for x in 1..cells.width - 1 {
                for y in 1..cells.height - 1 {
                    if rand::gen_range::<i32>(0, 500) == 0 {
                        cells.set(x, y, 0.5);
                    }
                }
            }
        }
        next_frame().await;

    }
}
