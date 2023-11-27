use itertools::Itertools;
use std::f32::consts::PI;

fn main() {
    const PI2: f32 = 2.0 * PI;

    const WIDTH: usize = 30; // Screen width.
    const HEIGHT: usize = 30; // Screen height.
    const R1: f32 = 1.0;
    const R2: f32 = 2.0;
    const K2: f32 = 5.0;
    const K1: f32 = (WIDTH as f32) * K2 * 3.0 / (8.0 * (R1 + R2));
    let mut screen = Screen::new(WIDTH, HEIGHT, K1, K2);

    const GRANULARITY_S: usize = 100;
    const GRANULARITY_T: usize = 50;

    let s_values: Vec<_> = (0..GRANULARITY_S)
        .map(|si| PI2 * (si as f32) / (GRANULARITY_S as f32))
        .collect();

    let t_values: Vec<_> = (0..GRANULARITY_T)
        .map(|ti| PI2 * (ti as f32) / (GRANULARITY_T as f32))
        .collect();

    const STEP_A: f32 = 0.05;
    const STEP_B: f32 = 0.03;
    const SLEEP_MS: u64 = 20;

    let mut a = 0.0;
    let mut b = 0.0;

    let next_radian = |x, step| (x + step) % PI2;

    loop {
        screen.clear();

        s_values
            .iter()
            .map(|&s| {
                return t_values.iter().map(move |&t| {
                    let p = Point::new(R2 + R1 * t.cos(), R1 * t.sin(), 0.0)
                        .rot_y(s)
                        .rot_x(a)
                        .rot_z(b);

                    let np = Point::new(t.cos(), t.sin(), 0.0).rot_y(s).rot_x(a).rot_z(b);
                    let luminance = np.in_prod(&[0.0, 1.0, -1.0]);
                    return (p, luminance);
                });
            })
            .flatten()
            .for_each(|(point, luminance)| screen.set(&point, luminance));

        screen.draw();
        a = next_radian(a, STEP_A);
        b = next_radian(b, STEP_B);
        std::thread::sleep(std::time::Duration::from_millis(SLEEP_MS));
    }
}

struct Point {
    xyz: Vec<f32>,
}

impl Point {
    fn new(x: f32, y: f32, z: f32) -> Self {
        let xyz = vec![x, y, z];
        Self { xyz }
    }

    fn x(&self) -> f32 {
        self.xyz[0]
    }

    fn y(&self) -> f32 {
        self.xyz[1]
    }

    fn z(&self) -> f32 {
        self.xyz[2]
    }

    /// Rotate point around x-axis.
    fn rot_x(self, radian: f32) -> Self {
        let c = radian.cos();
        let s = radian.sin();
        let rot = vec![vec![1.0, 0.0, 0.0], vec![0.0, c, s], vec![0.0, -s, c]];
        let mut xyz = dot(&vec![self.xyz], &rot).unwrap();
        Self {
            xyz: xyz.pop().unwrap(),
        }
    }

    /// Rotate point around y-axis.
    fn rot_y(self, radian: f32) -> Self {
        let c = radian.cos();
        let s = radian.sin();
        let rot = vec![vec![c, 0.0, s], vec![0.0, 1.0, 0.0], vec![-s, 0.0, c]];
        let mut xyz = dot(&vec![self.xyz], &rot).unwrap();
        Self {
            xyz: xyz.pop().unwrap(),
        }
    }

    /// Rotate point around z-axis.
    fn rot_z(self, radian: f32) -> Self {
        let c = radian.cos();
        let s = radian.sin();
        let rot = vec![vec![c, s, 0.0], vec![-s, c, 0.0], vec![0.0, 0.0, 1.0]];
        let mut xyz = dot(&vec![self.xyz], &rot).unwrap();
        Self {
            xyz: xyz.pop().unwrap(),
        }
    }

    fn in_prod(&self, v: &[f32; 3]) -> f32 {
        self.xyz
            .iter()
            .zip(v.iter())
            .fold(0.0, |acc, (&a, &b)| acc + a * b)
    }
}

#[derive(Clone)]
struct Cell {
    z_index: f32,
    luminance: f32,
}

impl Cell {
    fn new() -> Self {
        Self {
            z_index: 0.0,
            luminance: 0.0,
        }
    }

    fn with_value(z_index: f32, value: f32) -> Self {
        Self {
            z_index,
            luminance: value,
        }
    }
}

struct Screen {
    width: usize,
    height: usize,
    buf: Vec<Vec<Cell>>,
    k1: f32,
    k2: f32,
}

impl Screen {
    fn new(width: usize, height: usize, k1: f32, k2: f32) -> Self {
        let buf = vec![vec![Cell::new(); width + 1]; height + 1];
        Self {
            width,
            height,
            buf,
            k1,
            k2,
        }
    }

    fn set(&mut self, p: &Point, l: f32) {
        if l <= 0.0 {
            return;
        }

        let ooz = 1.0 / (self.k2 + p.z());
        let xp = ((self.width as f32) / 2.0 + self.k1 * ooz * p.x()) as usize;
        let yp = ((self.height as f32) / 2.0 - self.k1 * ooz * p.y()) as usize;

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
                    .map(|x| ".,-~:;=!*#$@".chars().nth(x.luminance as usize).unwrap())
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
        asesrt_eq_f32(result.x(), 1.0);
        asesrt_eq_f32(result.y(), -3.0);
        asesrt_eq_f32(result.z(), 2.0);
    }

    #[test]
    fn point_rot_y() {
        let p = Point::new(1.0, 2.0, 3.0);
        let result = p.rot_y(PI / 2.0);
        asesrt_eq_f32(result.x(), -3.0);
        asesrt_eq_f32(result.y(), 2.0);
        asesrt_eq_f32(result.z(), 1.0);
    }

    #[test]
    fn point_rot_z() {
        let p = Point::new(1.0, 2.0, 3.0);
        let result = p.rot_z(PI / 2.0);
        asesrt_eq_f32(result.x(), -2.0);
        asesrt_eq_f32(result.y(), 1.0);
        asesrt_eq_f32(result.z(), 3.0);
    }

    #[test]
    fn screen_render() {
        let mut screen = Screen::new(5, 5, 1.0, 2.0);
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(5.0, 0.0, 1.0);
        screen.set(&p1, 1.0);
        screen.set(&p2, 1.4);
        let result = screen.render();
        let expected = "\
. . . . . .
. . . . . .
. . * . @ .
. . . . . .
. . . . . .
. . . . . .";
        assert_eq!(result, expected);
    }

    fn asesrt_eq_f32(a: f32, b: f32) {
        let d = a - b;
        assert!(d.abs() <= std::f32::EPSILON, "{a} != {b}");
    }
}
