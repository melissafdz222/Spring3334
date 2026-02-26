use std::{thread, time, io::Write};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const BUFFER_SIZE: usize = WIDTH * HEIGHT * 3;

fn set_pixel(buffer: &mut [u8; BUFFER_SIZE], x: usize, y: usize, r: u8, g: u8, b: u8) {
    if x < WIDTH && y < HEIGHT {
        let index = (y * WIDTH + x) * 3;
        buffer[index] = r;
        buffer[index + 1] = g;
        buffer[index + 2] = b;
    }
}

fn clear_buffer(buffer: &mut [u8; BUFFER_SIZE], r: u8, g: u8, b: u8) {
    for i in (0..BUFFER_SIZE).step_by(3) {
        buffer[i] = r;
        buffer[i + 1] = g;
        buffer[i + 2] = b;
    }
}

fn draw_star(buffer: &mut [u8; BUFFER_SIZE], center_x: usize, center_y: usize, size: usize, brightness: u8) {
    // Draw the main body of the star
    for dy in 0..size {
        for dx in 0..size {
            let x = center_x + dx - size / 2;
            let y = center_y + dy - size / 2;
            if (dx == size / 2 || dy == size / 2) && x < WIDTH && y < HEIGHT {
                set_pixel(buffer, x, y, brightness, brightness, brightness);
            }
        }
    }

    // Draw the diagonals of the star
    for d in 0..size/2 {
        if center_x + d < WIDTH && center_y + d < HEIGHT {
            set_pixel(buffer, center_x + d, center_y + d, brightness, brightness, brightness);
            set_pixel(buffer, center_x - d, center_y - d, brightness, brightness, brightness);
            set_pixel(buffer, center_x + d, center_y - d, brightness, brightness, brightness);
            set_pixel(buffer, center_x - d, center_y + d, brightness, brightness, brightness);
        }
    }
}

fn display_buffer(buffer: &[u8; BUFFER_SIZE]) {
    // Clear the console (should work on most terminals)
    print!("\x1B[2J\x1B[1;1H");
    
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = (y * WIDTH + x) * 3;
            let avg = (buffer[index] as u32 + buffer[index + 1] as u32 + buffer[index + 2] as u32) / 3;
            print!("{}", if avg > 128 { "★" } else { " " });
        }
        println!();
    }
    // Ensure the output is displayed immediately
    std::io::stdout().flush().unwrap();
}

fn main() {
    let mut frame_buffer = [0u8; BUFFER_SIZE];
    let center_x = WIDTH / 2;
    let center_y = HEIGHT / 2;
    let max_size = 15;
    let min_size = 5;
    let frame_delay = time::Duration::from_millis(50);

    for cycle in 0..10 {
        for size in (min_size..=max_size).chain((min_size..max_size).rev()) {
            clear_buffer(&mut frame_buffer, 0, 0, 0);
            let brightness = ((size - min_size) as f32 / (max_size - min_size) as f32 * 255.0) as u8;
            draw_star(&mut frame_buffer, center_x, center_y, size, brightness);
            display_buffer(&frame_buffer);
            thread::sleep(frame_delay);
        }
    }

    println!("Animation complete! The star twinkled 10 times.");
}