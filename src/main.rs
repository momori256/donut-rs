use itertools::Itertools;
use std::f64::consts::PI;

struct Point {
    xyz: Vec<f64>,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        let xyz = vec![x, y, z];
        Self { xyz }
    }

    fn x(&self) -> f64 {
        self.xyz[0]
    }

    fn y(&self) -> f64 {
        self.xyz[1]
    }

    fn z(&self) -> f64 {
        self.xyz[2]
    }

    fn rot_x(self, radian: f64) -> Self {
        let c = radian.cos();
        let s = radian.sin();
        let rot = vec![vec![1.0, 0.0, 0.0], vec![0.0, c, s], vec![0.0, -s, c]];
        let mut xyz = dot(&vec![self.xyz], &rot).unwrap();
        Self {
            xyz: xyz.pop().unwrap(),
        }
    }

    fn rot_y(self, radian: f64) -> Self {
        let c = radian.cos();
        let s = radian.sin();
        let rot = vec![vec![c, 0.0, s], vec![0.0, 1.0, 0.0], vec![-s, 0.0, c]];
        let mut xyz = dot(&vec![self.xyz], &rot).unwrap();
        Self {
            xyz: xyz.pop().unwrap(),
        }
    }

    fn rot_z(self, radian: f64) -> Self {
        let c = radian.cos();
        let s = radian.sin();
        let rot = vec![vec![c, s, 0.0], vec![-s, c, 0.0], vec![0.0, 0.0, 1.0]];
        let mut xyz = dot(&vec![self.xyz], &rot).unwrap();
        Self {
            xyz: xyz.pop().unwrap(),
        }
    }

    fn in_prod(&self, v: &[f64; 3]) -> f64 {
        self.xyz
            .iter()
            .zip(v.iter())
            .fold(0.0, |acc, (&a, &b)| acc + a * b)
    }
}

#[derive(Clone)]
struct Cell {
    z_index: f64,
    value: f64,
}

impl Cell {
    fn new() -> Self {
        Self {
            z_index: 0.0,
            value: 0.0,
        }
    }

    fn with_value(z_index: f64, value: f64) -> Self {
        Self { z_index, value }
    }
}

struct Screen {
    width: usize,
    height: usize,
    buf: Vec<Vec<Cell>>,
    k1: f64,
    k2: f64,
}

impl Screen {
    fn new(width: usize, height: usize, k1: f64, k2: f64) -> Self {
        let buf = vec![vec![Cell::new(); width + 1]; height + 1];
        Self {
            width,
            height,
            buf,
            k1,
            k2,
        }
    }

    fn set(&mut self, p: &Point, l: f64) {
        if l <= 0.0 {
            return;
        }

        let ooz = 1.0 / (self.k2 + p.z());
        let xp = ((self.width as f64) / 2.0 + self.k1 * ooz * p.x()) as usize;
        let yp = ((self.height as f64) / 2.0 - self.k1 * ooz * p.y()) as usize;

        if ooz > self.buf[yp][xp].z_index {
            self.buf[yp][xp] = Cell::with_value(ooz, l * 8.0);
        }
    }

    fn clear(&mut self) {
        self.buf.iter_mut().for_each(|x| x.fill(Cell::new()));
    }

    fn draw(&self) {
        Self::clear_screen();
        Self::move_cursor(0, 0);
        println!("{}", self.render());
    }

    fn render(&self) -> String {
        self.buf
            .iter()
            .map(|v| {
                v.iter()
                    .map(|x| ".,-~:;=!*#$@".chars().nth(x.value as usize).unwrap())
                    .join(" ")
            })
            .join("\n")
    }

    fn move_cursor(x: usize, y: usize) {
        print!("\x1b[{y};{x}f");
    }

    fn clear_screen() {
        print!("\x1b[2J");
    }
}

fn main() {
    let width = 30;
    let height = 30;
    let r1 = 1.0;
    let r2 = 2.0;
    let k2 = 5.0;
    let k1 = (width as f64) * k2 * 3.0 / (8.0 * (r1 + r2));

    let mut screen = Screen::new(width, height, k1, k2);
    let mut a = 0.0_f64;
    let mut b = 0.0_f64;
    let step = 2.0 * PI / 100.0;

    loop {
        screen.clear();

        let mut s = 0.0_f64;
        while s < 2.0 * PI - step {
            let mut t = 0.0;
            while t < 2.0 * PI - step {
                let p = Point::new(r2 + r1 * t.cos(), r1 * t.sin(), 0.0);
                let p = p.rot_y(s).rot_x(a).rot_z(b);

                let np = Point::new(t.cos(), t.sin(), 0.0);
                let np = np.rot_y(s).rot_x(a).rot_z(b);
                let l = np.in_prod(&[0.0, 1.0, -1.0]);

                screen.set(&p, l);
                t += step;
            }
            s += step;
        }

        screen.draw();

        a += 0.05;
        b += 0.03;
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

fn dot<T>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>) -> Result<Vec<Vec<T>>, ()>
where
    T: Default + Copy + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    let ra = a.len();
    let cb = b[0].len();
    let ca = a[0].len();
    if ca != b.len() {
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

    #[test]
    fn point_rot_x() {
        let p = Point::new(1.0, 2.0, 3.0);
        let result = p.rot_x(PI / 2.0);
        println!("{}", result.z());
        asesrt_eq_f64(result.x(), 1.0);
        asesrt_eq_f64(result.y(), -3.0);
        asesrt_eq_f64(result.z(), 2.0);
    }

    #[test]
    fn point_rot_y() {
        let p = Point::new(1.0, 2.0, 3.0);
        let result = p.rot_y(PI / 2.0);
        asesrt_eq_f64(result.x(), -3.0);
        asesrt_eq_f64(result.y(), 2.0);
        asesrt_eq_f64(result.z(), 1.0);
    }

    #[test]
    fn point_rot_z() {
        let p = Point::new(1.0, 2.0, 3.0);
        let result = p.rot_z(PI / 2.0);
        asesrt_eq_f64(result.x(), -2.0);
        asesrt_eq_f64(result.y(), 1.0);
        asesrt_eq_f64(result.z(), 3.0);
    }

    fn asesrt_eq_f64(a: f64, b: f64) {
        let d = a - b;
        assert!(d.abs() <= std::f64::EPSILON, "{a} != {b}");
    }
}
