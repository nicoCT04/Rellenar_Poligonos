# Rellenar_Poligonos

Este repositorio contiene la solución al laboratorio de rellenado de polígonos utilizando Rust y raylib.

## Estructura del repositorio

- Cada rama `poligono1`, `poligono2`, `poligono3`, `poligono4` contiene **únicamente** el polígono correspondiente, con su color y orilla blanca.
- La rama `todos-los-poligonos` contiene el resultado **con todos los polígonos juntos**, usando los colores y reglas del enunciado.
- La rama principal (`main`) solo contiene commits de merge de las demás ramas, cumpliendo la rúbrica.

## Colores y ramas

- `poligono1`: Polígono 1 (amarillo)
- `poligono2`: Polígono 2 (azul)
- `poligono3`: Polígono 3 (rojo)
- `poligono4`: Polígono 4 (verde) con polígono 5 como agujero
- `todos-los-poligonos`: Todos los polígonos juntos

## Instrucciones de uso

1. **Clona el repositorio:**
    ```bash
    git clone https://github.com/nicoCT04/Rellenar_Poligonos.git
    cd Rellenar_Poligonos
    ```

2. **Cambia a la rama que deseas probar:**
    ```bash
    git checkout nombre-de-la-rama
    ```

3. **Compila y ejecuta el programa:**
    ```bash
    cargo run 
    ```

4. **El resultado se guarda como `out.bmp`. Ábrelo con cualquier visor de imágenes.**

## Requisitos

- [Rust](https://www.rust-lang.org/)
- [raylib](https://github.com/deltaphc/raylib-rs) para Rust (`raylib = "5.5.1"` en Cargo.toml)

## Nota

Este proyecto cumple con todos los criterios de la rúbrica:  
- Ramas separadas para cada polígono y una final con todos.  
- La rama principal (`main`) solo tiene merges de las otras ramas, sin commits directos de código.

---

Autor: [nicoCT04](https://github.com/nicoCT04)
