use itertools::Itertools;
use std::f64::consts::PI;

fn main() {
    let width = 30;
    let height = 30;
    let r1 = 1.0;
    let r2 = 2.0;
    let k2 = 5.0;
    let k1 = (width as f64) * k2 * 3.0 / (8.0 * (r1 + r2));
    // let ratio = width as f64 / height as f64;

    let scalex = |p: f64| (width as f64) / 6.0 * (p + r1 + r2);
    let scaley = |p: f64| (height as f64) / 6.0 * (-p + r1 + r2);

    let mut a = 0.0_f64;
    let mut b = 0.0_f64;
    loop {
        let mut screen = vec![vec![(0.0, 0.0); width + 1]; height + 1];
        let step = 2.0 * PI / 100.0;

        let ca = a.cos();
        let sa = a.sin();
        let cb = b.cos();
        let sb = b.sin();

        let mut s = 0.0_f64;
        while s < 2.0 * PI - step {
            let cs = s.cos();
            let ss = s.sin();

            let mut t = 0.0;
            while t < 2.0 * PI - step {
                let ct = t.cos();
                let st = t.sin();

                let x0 = r2 + r1 * ct;
                let y0 = r1 * st;
                let z0 = 0.0_f64;

                let x = x0 * (cs * cb + ss * sa * sb) - y0 * ca * sb;
                let y = x0 * (cs * sb - ss * sa * cb) + y0 * ca * cb;
                let z = k2 + x0 * ss * ca + y0 * sa;
                let ooz = 1.0 / z;

                let xp = ((width as f64) / 2.0 + k1 * ooz * x) as usize;
                let yp = ((height as f64) / 2.0 - k1 * ooz * y) as usize;
                // let nx0 = ct;
                // let ny0 = st;
                // let nz0 = 0;

                // let nx = nx0 * (cs * cb + ss * sa * sb) - ny0 * ca * sb;
                // let ny = nx0 * (cs * sb - ss * sa * cb) + ny0 * ca * cb;
                // let nz = nx0 * ss * ca + ny0 * sa;

                let l = cs * ct * sb - ss * ct * ca - st * sa + cb * (st * ca - ss * ct * sa);
                // let l = ny - nz;
                // println!("{l}, {x}, {y}, {z}, {xp}, {yp}");
                // if l <= 0.0 {
                //     continue;
                // }

                // let xp = scalex(x);
                // let yp = scaley(y);

                // println!("{}, {}", x as usize, y as usize);
                // let x = if xp >= 0.0 { xp + 0.5 } else { xp - 0.5 } as usize;
                // let y = if yp >= 0.0 { yp + 0.5 } else { yp - 0.5 } as usize;

                if l > 0.0 {
                    if ooz > screen[yp][xp].0 {
                        screen[yp][xp] = (ooz, l * 8.0);
                    }
                }

                // println!("{t} {x}, {y}");
                t += step;
            }
            s += step;
        }

        let screen = screen
            .iter()
            .map(|v| {
                v.iter()
                    .map(|&x| " ,-~:;=!*#$@".chars().nth(x.1 as usize).unwrap())
                    // .map(|&x| " @".chars().nth(x.1 as usize).unwrap())
                    // .map(|&x| match x {
                    //     1 => "@",
                    //     2 => "@",
                    //     _ => " ",
                    // })
                    .join(" ")
            })
            .join("\n");

        clear_screen();
        move_cursor(0, 0);
        println!("{screen}");

        a += 0.05;
        b += 0.03;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

fn move_cursor(x: usize, y: usize) {
    print!("\x1b[{y};{x}f");
}

fn clear_screen() {
    print!("\x1b[2J");
}
