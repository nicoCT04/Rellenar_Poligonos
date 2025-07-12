use raylib::prelude::*;

type Point = (i32, i32);

// Puntos del tercer polígono
fn poligono3() -> Vec<Point> {
    vec![
        (377, 249), (411, 197), (436, 249)
    ]
}

// Reflejar puntos en Y
fn flip_points(points: &[Point], image_height: i32) -> Vec<Point> {
    points.iter().map(|&(x, y)| (x, image_height - y)).collect()
}

// Algoritmo de Bresenham para líneas
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

// Dibuja el contorno del polígono
fn draw_polygon_outline(image: &mut Image, points: &[Point], color: Color) {
    for i in 0..points.len() {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % points.len()];
        draw_line_bresenham(image, x0, y0, x1, y1, color);
    }
}

// Rellenar el polígono (scanline fill)
fn fill_polygon(image: &mut Image, polygon: &[Point], color: Color) {
    let (min_y, max_y) = polygon.iter().fold((i32::MAX, i32::MIN), |(min_y, max_y), &(_, y)| {
        (min_y.min(y), max_y.max(y))
    });
    for y in min_y..=max_y {
        let mut nodes = Vec::new();
        let n = polygon.len();
        let mut j = n - 1;
        for i in 0..n {
            let (xi, yi) = polygon[i];
            let (xj, yj) = polygon[j];
            if (yi < y && yj >= y) || (yj < y && yi >= y) {
                let x = xi + (y - yi) * (xj - xi) / (yj - yi);
                nodes.push(x);
            }
            j = i;
        }
        nodes.sort();
        let mut i = 0;
        while i + 1 < nodes.len() {
            let x_start = nodes[i];
            let x_end = nodes[i + 1];
            for x in x_start..=x_end {
                image.draw_pixel(x, y, color);
            }
            i += 2;
        }
    }
}

fn main() {
    let image_width = 900;
    let image_height = 500;

    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    // Aplica el flip a los puntos
    let p1 = flip_points(&poligono3(), image_height);

    // Rellenar polígono 1 (amarillo)
    fill_polygon(&mut image, &p1, Color::RED);

    // Dibuja el contorno en blanco
    draw_polygon_outline(&mut image, &p1, Color::WHITE);

    // Exporta la imagen
    image.export_image("out.bmp");
    println!("Imagen generada: out.bmp");
}
