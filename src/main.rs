use macroquad::prelude::*;

const DEAD_CLR: Color = BLACK;
const ALIVE_CLR: Color = WHITE;


struct CellGrid {
    data: Vec<u8>,
    width: usize,
    height: usize
}

impl CellGrid {
    fn get(&self, x: usize, y: usize) -> u8{
        let i = y * self.width + x;
        self.data[i]
    }

    fn set(&mut self, x: usize, y: usize, color: u8){
        let i = y * self.width + x;
        self.data[i] = color;
    }

    fn new(&self, width: usize, height: usize) -> Self{
        CellGrid { data: vec![0; height * width], 
                    width: width, 
                    height: height }
    }
}


#[macroquad::main("cellular-automata")]
async fn main() {
    let mut image = Image::gen_image_color(200, 200, BLACK);
    for x in 1..image.width - 1 {
        for y in 1..image.height - 1 {
            if rand::gen_range(0, 3) == 0 {
                image.set_pixel(x.into(), y.into(), WHITE);

            }
        }
    }
    let mut fpss: Vec<i32> = Vec::new();
    
    loop {
        clear_background(BLACK);


        for x in 1..image.width - 1 {
            for y in 1..image.height - 1 {
                let x = x.into();
                let y = y.into();
                let alive = image.get_pixel(x, y) == ALIVE_CLR;
                let mut alive_neighbours = 0;

                let neigbours = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), 
                                            (1, -1), (1, 0), (1, 1)];

                for (a, b) in neigbours.iter() { 
                    alive_neighbours += (image.get_pixel((x as i32 + a) as u32, (y as i32 + b) as u32) == WHITE) as u32;
                }

                if alive && !vec![2, 3].contains(&alive_neighbours){
                    image.set_pixel(x, y, DEAD_CLR);
                } else if !alive && alive_neighbours == 3{
                    image.set_pixel(x, y, ALIVE_CLR);
                }
            }
        }
        let texture = Texture2D::from_image(&image);
        texture.update(&image);
        // let camera = Camera2D {
        //     zoom: Vec2::new(1.0, 1.0),
        //     ..Default::default()
        // };
        set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, image.width.into(), image.height.into())));
        draw_texture(texture, 0.0, 0.0, WHITE);
        fpss.push(get_fps());
        let l = fpss.len().saturating_sub(10);
        let fps_window = &fpss[l..];
        let fps = fps_window.iter().sum::<i32>() as f32 / fps_window.len() as f32;
        println!("{:?}", fps);
        next_frame().await;
    }
}
