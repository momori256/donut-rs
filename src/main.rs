use std::f64::consts::PI;

fn main() {
    let width = 30;
    let height = 30;
    // let ratio = width as f64 / height as f64;

    let scalex = |p: f64| (width as f64) / 2.0 * (p + 1.0);
    let scaley = |p: f64| (height as f64) / 2.0 * (-p + 1.0);

    let mut a = 0.0_f64;
    let mut b = 0.0_f64;
    loop {
        clear_screen();
        move_cursor(0, 0);

        let mut screen = vec![vec![0; width + 1]; height + 1];
        let step = 2.0 * PI / 100.0;
        let mut t = 0.0;

        let ca = a.cos();
        let sa = a.sin();
        let cb = b.cos();
        let sb = b.sin();

        while t < 2.0 * PI - step {
            let x_base = t.cos();
            let y_base = t.sin();
            let z_base = 0.0_f64;

            let x = x_base * cb - y_base * ca * sb;
            let y = x_base * sb + y_base * ca * cb;
            let z = y_base * sa;

            // println!("{t} {x}, {y}");
            let x = scalex(x);
            let y = scaley(y);
            // println!("{}, {}", x as usize, y as usize);
            let x = if x >= 0.0 { x + 0.5 } else { x - 0.5 } as usize;
            let y = if y >= 0.0 { y + 0.5 } else { y - 0.5 } as usize;
            screen[y][x] = 1;
            if t.abs() < step {
                screen[y][x] = 2;
            }
            t += step;
        }

        let screen = screen
            .iter()
            .map(|v| {
                v.iter()
                    .map(|&x| match x {
                        1 => "@",
                        2 => "!",
                        _ => " ",
                    })
                    .join(" ")
            })
            .join("\n");
        println!("{screen}");

        a += 0.07;
        b += 0.03;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn move_cursor(x: usize, y: usize) {
    print!("\x1b[{y};{x}f");
}

fn clear_screen() {
    print!("\x1b[2J");
}

use itertools::Itertools;
