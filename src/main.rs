use itertools::Itertools;
use std::f64::consts::PI;

fn main() {
    let width = 30;
    let height = 30;
    let r1 = 1.0;
    let r2 = 2.0;
    let k2 = 5.0;
    let k1 = (width as f64) * k2 * 3.0 / (8.0 * (r1 + r2));

    let mut screen = vec![vec![(0.0, 0.0); width + 1]; height + 1];
    let mut a = 0.0_f64;
    let mut b = 0.0_f64;
    loop {
        let step = 2.0 * PI / 100.0;

        let ca = a.cos();
        let sa = a.sin();
        let cb = b.cos();
        let sb = b.sin();

        screen.iter_mut().for_each(|r| r.fill((0.0, 0.0)));

        let mut s = 0.0_f64;
        while s < 2.0 * PI - step {
            let cs = s.cos();
            let ss = s.sin();

            let mut t = 0.0;
            while t < 2.0 * PI - step {
                let ct = t.cos();
                let st = t.sin();

                let rot_s = vec![vec![cs, 0.0, ss], vec![0.0, 1.0, 0.0], vec![-ss, 0.0, cs]];
                let rot_a = vec![vec![1.0, 0.0, 0.0], vec![0.0, ca, sa], vec![0.0, -sa, ca]];
                let rot_b = vec![vec![cb, sb, 0.0], vec![-sb, cb, 0.0], vec![0.0, 0.0, 1.0]];

                let xyz = vec![vec![r2 + r1 * ct, r1 * st, 0.0]];
                let xyz = dot(&xyz, &rot_s).unwrap();
                let xyz = dot(&xyz, &rot_a).unwrap();
                let xyz = dot(&xyz, &rot_b).unwrap();

                let x = xyz[0][0];
                let y = xyz[0][1];
                let z = k2 + xyz[0][2];
                let ooz = 1.0 / z;

                let xp = ((width as f64) / 2.0 + k1 * ooz * x) as usize;
                let yp = ((height as f64) / 2.0 - k1 * ooz * y) as usize;

                let nxyz = vec![vec![ct, st, 0.0]];
                let nxyz = dot(&nxyz, &rot_s).unwrap();
                let nxyz = dot(&nxyz, &rot_a).unwrap();
                let nxyz = dot(&nxyz, &rot_b).unwrap();
                let l = dot(&nxyz, &vec![vec![0.0], vec![1.0], vec![-1.0]]).unwrap()[0][0];

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
                    .map(|&x| ".,-~:;=!*#$@".chars().nth(x.1 as usize).unwrap())
                    .join(" ")
            })
            .join("\n");

        clear_screen();
        move_cursor(0, 0);
        println!("{screen}");

        a += 0.05;
        b += 0.03;
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

fn move_cursor(x: usize, y: usize) {
    print!("\x1b[{y};{x}f");
}

fn clear_screen() {
    print!("\x1b[2J");
}

fn dot<T>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>) -> Result<Vec<Vec<T>>, ()>
where
    T: Default + Copy + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    let ra = a.len();
    let cb = b[0].len();
    let ca = a[0].len();
    if ca != b.len() {
        println!("{ca}, {}", b.len());
        return Err(());
    }

    let mut result = vec![vec![T::default(); cb]; ra];

    for i in 0..ra {
        for j in 0..cb {
            let mut acc = T::default();
            for k in 0..ca {
                acc += a[i][k] * b[k][j];
            }
            result[i][j] = acc;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_r2c2_r2c2() {
        let a = vec![vec![2, 5], vec![4, 7]];
        let b = vec![vec![1, 3], vec![6, 9]];
        let result = dot(&a, &b);
        let x = vec![vec![32, 51], vec![46, 75]];
        assert_eq!(result, Ok(x));
    }

    #[test]
    fn dot_r1c3_r3c2() {
        let a = vec![vec![2, 4, 6]];
        let b = vec![vec![7, 5], vec![3, 4], vec![6, 2]];
        let result = dot(&a, &b);
        let x = vec![vec![62, 38]];
        assert_eq!(result, Ok(x));
    }

    #[test]
    fn dot_r1c2_r2c2_r2c2() {
        let a = vec![vec![1, 2]];
        let b = vec![vec![1, 2], vec![1, 2]];
        let c = vec![vec![1, 2], vec![1, 2]];
        let result = dot(&dot(&a, &b).unwrap(), &c);
        let x = vec![vec![9, 18]];
        assert_eq!(result, Ok(x));
    }
}
