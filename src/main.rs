use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;


fn map(val: f32, min: f32, max: f32, from: f32, to: f32) -> f32 {
    (val - min) * (to - from) / (max - min) + from
}

#[derive(Clone)]
struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Pixel {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    fn to_u8(&self) -> [u8; 4] {
        [
            map( self.r, 0.0, 1.0, 0.0, 255.0 ) as u8,
            map( self.g, 0.0, 1.0, 0.0, 255.0 ) as u8,
            map( self.b, 0.0, 1.0, 0.0, 255.0 ) as u8,
            map( self.a, 0.0, 1.0, 0.0, 255.0 ) as u8
        ]
    }


    fn from_hex(col: u32) -> Pixel {
        let r = map((col >> 16) as f32, 0.0, 255.0, 0.0, 1.0);
        let g = map((col >>  8) as f32, 0.0, 255.0, 0.0, 1.0);
        let b = map((col >>  0) as f32, 0.0, 255.0, 0.0, 1.0);

        Pixel::new(r,g,b)
    }
}


fn save_image(pixels: Vec<Pixel>, width: u32, height: u32) {
    let path = Path::new("output.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let pixels: Vec<_> = pixels
        .iter()
        .map(|p| p.to_u8())
        .collect();
    let pixels: Vec<u8> = pixels
        .iter()
        .flat_map(|p| p.iter())
        .map(|v| v.clone())
        .collect();


    let mut encoder = png::Encoder::new(w, width, height); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
     let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels[0..pixels.len()]).unwrap(); // Save
}

fn main() {
    let width =  2000i32;
    let height = 2000i32;

    let pallete = [
        Pixel::from_hex(0x000000),
        Pixel::from_hex(0x9B1D1D),
        Pixel::from_hex(0x3D2A2A),
        Pixel::from_hex(0x635C5C),
        Pixel::from_hex(0xFFFFFF),
    ];

    let mut pixels = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let scale = 0.5;
            let x_off = 0.15; // -0.5;
            let y_off = 0.25; // -0.5;
            let x = map(x as f32, 0.0, width as f32,  -(scale / 2.0) + x_off, scale / 2.0 + x_off);
            let y = map(y as f32, 0.0, height as f32, -(scale / 2.0) + y_off, scale / 2.0 + y_off);

            let mut za = x;
            let mut zb = y;

            let iterations = 1000;
            let mut amount = 0;
            for n in 0..iterations {
                amount = n;
                // c = −0.7269 + 0.1889i
                let res = f(za, zb, -0.7269, 0.1889);
                za = res.0;
                zb = res.1;

                if za*za + zb*zb > 100.0 {
                    break;
                }
            }

            let c_index = map(amount as f32, 0.0, iterations as f32, 0.0, (pallete.len() as f32) - 1.0 ) as usize;
            let pixel = pallete[c_index].clone();
            pixels.push(pixel);
        }
    }

    save_image(pixels, width as u32, height as u32);
}

fn f(za: f32, zb: f32, ca: f32, cb: f32) -> (f32, f32) {
    let zaa = za * za;
    let zbb = zb * zb;

    (zaa - zbb + ca, 2.0 * za * zb + cb)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let v = 0.0;
        let r = map(v, 0.0, 1.0, 0.0, 255.0);
        assert_eq!(r, 0.0);

        let v = 1.0;
        let r = map(v, 0.0, 1.0, 0.0, 255.0);
        assert_eq!(r, 255.0);

        let v = 0.5;
        let r = map(v, 0.0, 1.0, 0.0, 255.0);
        assert_eq!(r, 127.0);
    }

    #[test]
    fn hex_test() {
        let p = Pixel::from_hex(0xff0000);
        println!("{:?}", p.to_u8());
        // panic!();
        println!("{}", asdsa);
    }
}
