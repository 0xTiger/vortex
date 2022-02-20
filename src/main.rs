use macroquad::prelude::*;


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
                let mut color = BLACK;
                let alive = image.get_pixel(x, y) == WHITE;
                let mut alive_neighbours = 0;
                image.get_pixel(x, y);

                let neigbours: &[(i32, i32)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), 
                                            (1, -1), (1, 0), (1, 1)];

                for &(a,b) in neigbours.iter() {
                    
                alive_neighbours += (image.get_pixel((x as i32 + a) as u32, (y as i32 + b) as u32) == WHITE) as u32;
                }

                if alive && !vec![2, 3].contains(&alive_neighbours){
                    image.set_pixel(x, y, BLACK);
                } else if !alive && alive_neighbours == 3{
                    image.set_pixel(x, y, WHITE);
                }
            }
        }
        let texture = Texture2D::from_image(&image);
        texture.update(&image);
        // let camera = Camera2D {
        //     zoom: Vec2::new(1.0, 1.0),
        //     ..Default::default()
        // };
        set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, 100.0, 100.0)));
        draw_texture(texture, 0.0, 0.0, WHITE);
        fpss.push(get_fps());
        let l = fpss.len().saturating_sub(10);
        let fps_window = &fpss[l..];
        let fps = fps_window.iter().sum::<i32>() as f32 / fps_window.len() as f32;
        println!("{:?}", fps);
        next_frame().await;
    }
}
