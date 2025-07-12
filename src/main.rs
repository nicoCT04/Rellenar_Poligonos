use raylib::prelude::*;

type Point = (i32, i32);

fn poligono1() -> Vec<Point> {
    vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ]
}

// Algoritmo scanline fill para rellenar el polígono
fn fill_polygon(image: &mut Image, polygon: &[Point], color: Color) {
    // Encuentra el rango vertical
    let (min_y, max_y) = polygon.iter().fold((i32::MAX, i32::MIN), |(min_y, max_y), &(_, y)| {
        (min_y.min(y), max_y.max(y))
    });
    for y in min_y..=max_y {
        // Buscar intersecciones de la scanline con los lados del polígono
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
        // Rellenar entre pares de intersecciones
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

// Algoritmo de Bresenham para el contorno
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

fn main() {
    let image_width = 900;
    let image_height = 500;

    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    // Rellenar polígono 1 (rojo)
    fill_polygon(&mut image, &poligono1(), Color::YELLOW);

    // Contorno blanco
    draw_polygon_outline(&mut image, &poligono1(), Color::WHITE);

    // Exporta el resultado
    image.export_image("out.bmp");
    println!("Imagen generada: out.bmp");
}
