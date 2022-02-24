use macroquad::prelude::*;

const N: usize = 200;
const DAMPENING: f32 = 0.98;
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

    // fn to_texture(&self) -> Texture2D {
    //     let mut bytes = Vec::new();
    //     for x in &self.data {
    //         bytes.push(x * 255.0);
    //         bytes.push(x * 255.0);
    //         bytes.push(x * 255.0);
    //         bytes.push(255);
    //     }
    //     Texture2D::from_rgba8(self.width as u16, self.height as u16, &bytes)
    // }
}


#[macroquad::main("cellular-automata")]
async fn main() {
    let mut cells = CellGrid::new(N, N, 0f32);
    for x in 1..cells.width - 1 {
        for y in 1..cells.height - 1 {
            if rand::gen_range::<i32>(0, 500) == 0 {
                cells.set(x, y, 0.5);
            }
        }
    }
    let mut fpss: Vec<i32> = Vec::new();
    
    loop {
        clear_background(BLACK);
        let mut cells_new = cells.clone();
        for x in 1..cells.width - 1 {
            for y in 1..cells.height - 1 {
                let b = cells.get(x, y);
                let donation = rand::gen_range::<f32>(0.0, 1.5*b);
                let donation_split = rand::gen_range::<f32>(0.0, 1.0);
                let v_donation = donation_split * donation;
                let h_donation = (1.0 - donation_split) * donation;
                cells_new.set(x, y - 1, cells_new.get(x, y - 1) + v_donation * DAMPENING);
                cells_new.set(x - 1, y, cells_new.get(x - 1, y) + h_donation * DAMPENING);
                cells_new.set(x, y, cells_new.get(x, y) - donation);
            }
        }

        // for x in 1..cells.width - 1 {
        //     for y in 1..cells.height - 1 {
        //         if rand::gen_range::<i32>(0, 100) == 0 {
        //             cells_new.set(x, y, (cells_new.get(x, y) * 3.0).clamp(0.0, 0.7));
        //         }
        //     }
        // }

        for x in 1..cells.width - 1 {
            for y in 1..cells.height - 1 {
                if rand::gen_range::<i32>(0, 1000) == 0 {
                    cells_new.set(x, y, 0.5);
                }
            }
        }

        // let texture = Texture2D::from_image(&image);
        // image.update(&cells.to_colors());
        // texture.update(&image);
        // let camera = Camera2D {
        //     zoom: Vec2::new(1.0, 1.0),
        //     ..Default::default()
        // };
        let mut bytes = Vec::new();
        for x in &cells_new.data {
            let brightness = (x.clamp(0.0, 1.0) * 255.0) as u8;
            bytes.push(brightness);
            bytes.push(brightness);
            bytes.push(brightness);
            bytes.push(255);
        }
        cells = cells_new;
        let texture = Texture2D::from_rgba8(cells.width as u16, cells.height as u16, &bytes);
        texture.set_filter(FilterMode::Nearest);
        let m = screen_width().min(screen_height());
        // let m = 800.0;
        // set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, image.width.into(), image.height.into())));
        draw_texture_ex(texture, 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(m, m)), ..Default::default()});
        fpss.push(get_fps());
        let l = fpss.len().saturating_sub(10);
        let fps_window = &fpss[l..];
        let fps = fps_window.iter().sum::<i32>() as f32 / fps_window.len() as f32;
        println!("{:?}", fps);
        next_frame().await;
    }
}
