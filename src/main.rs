use macroquad::prelude::*;

const DEAD_CLR: Color = BLACK;
const ALIVE_CLR: Color = WHITE;


struct CellGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize
}

impl<T: Clone + Copy + Into<u8>> CellGrid<T> {
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

    fn to_colors(&self) -> Vec<Color> {
        let mut v = Vec::new();
        for x in &self.data {
            v.push(Color::from_rgba(x.clone().into(), x.clone().into(), x.clone().into(), 255))
        }
        v
    }

    fn to_texture(&self) -> Texture2D {
        let mut bytes = Vec::new();
        for x in &self.data {
            bytes.push(x.clone().into());
            bytes.push(x.clone().into());
            bytes.push(x.clone().into());
            bytes.push(255);
        }
        Texture2D::from_rgba8(self.width as u16, self.height as u16, &bytes)
    }
}


#[macroquad::main("cellular-automata")]
async fn main() {
    let mut cells = CellGrid::new(200, 200, 0);
    let image = Image::gen_image_color(200, 200, BLACK);
    for x in 1..cells.width - 1 {
        for y in 1..cells.height - 1 {
            if rand::gen_range(0, 3) == 0 {
                cells.set(x, y, 255);

            }
        }
    }
    let mut fpss: Vec<i32> = Vec::new();
    
    loop {
        clear_background(BLACK);


        for x in 1..cells.width - 1 {
            for y in 1..cells.height - 1 {
                let x = x.into();
                let y = y.into();
                let alive = cells.get(x, y) > 0;
                let mut alive_neighbours = 0;

                let neigbours = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), 
                                            (1, -1), (1, 0), (1, 1)];

                for (a, b) in neigbours.iter() { 
                    alive_neighbours += (cells.get((x as i32 + a) as usize, (y as i32 + b) as usize) > 0) as u32;
                }

                if alive && !vec![2, 3].contains(&alive_neighbours){
                    cells.set(x, y, 0);
                } else if !alive && alive_neighbours == 3{
                    cells.set(x, y, 255);
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
        let texture = cells.to_texture();
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
