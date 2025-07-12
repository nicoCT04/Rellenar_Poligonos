use raylib::prelude::*;

type Point = (i32, i32);

fn poligono4() -> Vec<Point> {
    vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)
    ]
}

fn poligono5() -> Vec<Point> {
    vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ]
}

fn flip_points(points: &[Point], image_height: i32) -> Vec<Point> {
    points.iter().map(|&(x, y)| (x, image_height - y)).collect()
}

fn draw_line_bresenham(image: &mut Image, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        image.draw_pixel(x0, y0, color);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_polygon_outline(image: &mut Image, points: &[Point], color: Color) {
    for i in 0..points.len() {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % points.len()];
        draw_line_bresenham(image, x0, y0, x1, y1, color);
    }
}

fn fill_polygon_with_hole(image: &mut Image, outer: &[Point], hole: &[Point], color: Color) {
    let (min_y, max_y) = outer.iter().fold((i32::MAX, i32::MIN), |(min_y, max_y), &(_, y)| {
        (min_y.min(y), max_y.max(y))
    });
    for y in min_y..=max_y {
        let mut nodes_outer = Vec::new();
        let n_outer = outer.len();
        let mut j = n_outer - 1;
        for i in 0..n_outer {
            let (xi, yi) = outer[i];
            let (xj, yj) = outer[j];
            if (yi < y && yj >= y) || (yj < y && yi >= y) {
                let x = xi + (y - yi) * (xj - xi) / (yj - yi);
                nodes_outer.push(x);
            }
            j = i;
        }
        nodes_outer.sort();

        let mut nodes_hole = Vec::new();
        let n_hole = hole.len();
        let mut j = n_hole - 1;
        for i in 0..n_hole {
            let (xi, yi) = hole[i];
            let (xj, yj) = hole[j];
            if (yi < y && yj >= y) || (yj < y && yi >= y) {
                let x = xi + (y - yi) * (xj - xi) / (yj - yi);
                nodes_hole.push(x);
            }
            j = i;
        }
        nodes_hole.sort();

        let mut i = 0;
        while i + 1 < nodes_outer.len() {
            let x_start = nodes_outer[i];
            let x_end = nodes_outer[i + 1];
            let mut x = x_start;
            while x <= x_end {
                let mut inside_hole = false;
                let mut k = 0;
                while k + 1 < nodes_hole.len() {
                    if x >= nodes_hole[k] && x <= nodes_hole[k + 1] {
                        inside_hole = true;
                        break;
                    }
                    k += 2;
                }
                if !inside_hole {
                    image.draw_pixel(x, y, color);
                }
                x += 1;
            }
            i += 2;
        }
    }
}

fn main() {
    let image_width = 900;
    let image_height = 500;
    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    // Polígono 4 (verde) con polígono 5 como agujero
    let p4 = flip_points(&poligono4(), image_height);
    let p5 = flip_points(&poligono5(), image_height);

    fill_polygon_with_hole(&mut image, &p4, &p5, Color::GREEN);
    draw_polygon_outline(&mut image, &p4, Color::WHITE);
    draw_polygon_outline(&mut image, &p5, Color::WHITE);

    image.export_image("out.bmp");
    println!("Imagen generada: out.bmp");
}
