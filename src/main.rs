use macroquad::prelude::*;
use rand::gen_range;

// 1. LA GESTIÓN DE LA MATERIA (Física de partículas de agua)
struct Drop {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

#[macroquad::main("AXIOM FLUID // CELLULAR FLUIDS")]
async fn main() {
    let mut water_drops: Vec<Drop> = Vec::new();
    
    // Suelo y obstáculos rígidos fijos en el escenario
    let floor_y = 500.0;
    let obstacle_x = 400.0;
    let obstacle_y = 350.0;
    let obstacle_radius = 60.0;

    loop {
        // Fondo negro búnker profundo
        clear_background(Color::new(0.01, 0.01, 0.03, 1.0));

        // 🔥 GENERADOR CONTINUO DE AGUA
        // Spawnea gotas de agua en la parte superior simulando un grifo abierto
        if water_drops.len() < 4000 {
            for _ in 0..8 {
                water_drops.push(Drop {
                    x: gen_range( screen_width() / 2.0 - 40.0, screen_width() / 2.0 + 40.0),
                    y: 50.0,
                    vx: gen_range(-1.0, 1.0),
                    vy: 2.0,
                });
            }
        }

        let dt = get_frame_time();

        // 🔥 EL MOTOR DE FLUIDOS (Gravedad + Viscosidad + Colisión)
        for drop in &mut water_drops {
            // Aplicamos gravedad nativa a la materia
            drop.vy += 9.8 * dt * 2.0;

            // Actualizamos posición
            drop.x += drop.vx;
            drop.y += drop.vy;

            // Colisión física contra el obstáculo circular del centro (Efecto paraguas)
            let dx = drop.x - obstacle_x;
            let dy = drop.y - obstacle_y;
            let dist = (dx*dx + dy*dy).sqrt();

            if dist < obstacle_radius {
                // Vector de rebote hacia fuera del círculo para simular flujo fluido
                drop.x = obstacle_x + (dx / dist) * obstacle_radius;
                drop.y = obstacle_y + (dy / dist) * obstacle_radius;
                drop.vx = (dx / dist) * gen_range(1.5, 3.5);
                drop.vy = (dy / dist) * gen_range(1.0, 2.5);
            }

            // Colisión contra el suelo duro (Acumulación de líquido y fricción)
            if drop.y >= floor_y {
                drop.y = floor_y;
                drop.vy = 0.0;
                // Las gotas se expanden hacia los lados de forma viscosa al chocar con el fondo
                if drop.vx == 0.0 {
                    drop.vx = if gen_range(0, 2) == 0 { 1.5 } else { -1.5 };
                }
                drop.x += drop.vx * gen_range(0.8, 1.2);
            }

            // Límites de pantalla laterales
            if drop.x < 50.0 { drop.x = 50.0; drop.vx *= -0.5; }
            if drop.x > screen_width() - 50.0 { drop.x = screen_width() - 50.0; drop.vx *= -0.5; }
        }

        // 🔥 RENDERIZADO DEL FLUIDO NEÓN
        // Dibujamos el agua con un color cian/azul eléctrico translúcido de alta intensidad
        for drop in &water_drops {
            draw_circle(drop.x, drop.y, 3.5, Color::new(0.0, 0.6, 1.0, 0.4));
        }

        // Dibujamos el obstáculo físico del centro del mapa
        draw_circle(obstacle_x, obstacle_y, obstacle_radius, Color::new(0.1, 0.1, 0.15, 1.0));
        draw_circle_lines(obstacle_x, obstacle_y, obstacle_radius, 2.0, MAGENTA);

        // Dibujamos la línea del suelo duro
        draw_line(50.0, floor_y + 4.0, screen_width() - 50.0, floor_y + 4.0, 3.0, RED);

        // HUD de Telemetría
        draw_rectangle(10.0, 10.0, 310.0, 80.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 30.0, 20.0, GREEN);
        draw_text("CORE: CELLULAR FLUID DYNAMICS", 20.0, 50.0, 14.0, MAGENTA);
        draw_text(&format!("WATER PARTICLES: {}", water_drops.len()), 20.0, 70.0, 15.0, CYAN);

        next_frame().await
    }
}
