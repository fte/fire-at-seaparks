extern crate rand;
use rand::Rng;
use std::{thread::sleep, time::Duration, io::{Write, stdout}};

const WIDTH: usize = 80;
const HEIGHT: usize = 30;
const COOLING: u8 = 5;  // Plus grand = extinction plus rapide
const SPREAD: usize = 3; // Plus grand = plus de diffusion

fn main() {
    let mut rng = rand::thread_rng();
    let mut fire_pixels = vec![vec![0u8; WIDTH]; HEIGHT];

    loop {
        // Ajouter des flammes en bas
        for x in 0..WIDTH {
            fire_pixels[HEIGHT - 1][x] = rng.gen_range(180..255);
        }

        // Propagation du feu
        for y in (1..HEIGHT).rev() {
            for x in 0..WIDTH {
                let below = fire_pixels[y][x] as i16;
                let decay = rng.gen_range(0..COOLING) as i16;
                
                // Déplacement aléatoire du feu (effet de turbulence)
                let spread = rng.gen_range(0..SPREAD);
                let target_x = x.saturating_sub(spread);
                
                let new_value = below.saturating_sub(decay);
                fire_pixels[y - 1][target_x] = new_value.max(0) as u8;
            }
        }

        // Affichage
        print!("\x1B[H"); // Retour début écran
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let intensity = fire_pixels[y][x];
                let color = match intensity {
                    220..=255 => "\x1B[38;2;255;50;0m█",  // Rouge vif
                    180..=219 => "\x1B[38;2;255;128;0m█", // Orange intense
                    140..=179 => "\x1B[38;2;255;180;50m█", // Jaune orangé
                    100..=139 => "\x1B[38;2;200;200;50m█", // Jaune pâle
                    60..=99   => "\x1B[38;2;120;120;30m█", // Brun chaud
                    20..=59   => "\x1B[38;2;60;30;10m█",  // Cendre foncée
                    _         => " ",  // Absence de feu
                };
                print!("{}", color);
            }
            println!();
        }
        stdout().flush().unwrap();
        sleep(Duration::from_millis(80)); // Animation plus fluide
    }
}
