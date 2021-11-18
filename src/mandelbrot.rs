extern crate image;

#[derive(Clone)]
pub struct ImageSize {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Parameters {
    pub size:       ImageSize,
    pub position:   Position,
    pub scale:      f32,
    pub iterations: u32,
}

use std::{sync::mpsc,
          thread};

use image::{ImageBuffer,
            Rgb};

fn mandel(dx: f32, dy: f32, max: u32) -> u32 {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;

    let mut a2: f32 = 0.0;
    let mut b2: f32 = 0.0;
    let mut i: u32 = 0;
    // f(z) = z^2 + c
    while a2 + b2 < 4.0 && i != max {
        //println!("{} + {}i", a, b);
        a2 = a * a;
        b2 = b * b;

        b = 2.0 * a * b + dy;
        a = a2 - b2 + dx;

        i += 1;
    }
    i
}

fn gen(
    x1: u32, x2: u32, y1: u32, y2: u32, parameters: Parameters,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let imgx = parameters.size.x;
    let imgy = parameters.size.x;
    let posx = parameters.position.x;
    let posy = parameters.position.y;
    let scale = (10.0_f64).powf(parameters.scale.into()) as f32;
    let iterations = parameters.iterations;
    // generate the fractal
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for x in x1..x2 {
        print!("{:.2}%\r", (x as f32 / x2 as f32) * 100.0);
        for y in y1..y2 {
            //let dx: f32 = (x as f32 / imgx as f32) as f32;
            //let dy: f32 = (y as f32 / imgy as f32) as f32;
            let dx: f32 = (x as f32 - (imgx / 2) as f32) / (scale * imgx as f32) + posx;
            let dy: f32 = (y as f32 - (imgy / 2) as f32) / (scale * imgy as f32) + posy;

            let i = mandel(dx, dy, iterations) as u8;
            //let i = julia(-1.1086391524242, 0.25949259547294, dx, dy, 1, iterations) as u8;

            //let mut f: u8 = 0;

            //f = ((i % 100) * 255) as u8;
            //println!("{}, {}: \n i == {}, f == {}", dx, dy, i, f);

            let pixel = imgbuf.get_pixel_mut(x, y);
            //let image::Rgb(data) = *pixel;
            if i == iterations as u8 {
                *pixel = image::Rgb([1, 1, 1]);
            } else {
                *pixel = image::Rgb([i, 0, i]);
            }
        }
    }
    println!("{}, {}, {}, {}", x1, x2, y1, y2);
    imgbuf
}

pub fn spawn(tx: mpsc::Sender<ImageBuffer<Rgb<u8>, Vec<u8>>>, n: u32, parameters: &Parameters) {
    let imgx = parameters.size.x;
    let imgy = parameters.size.x;
    let mut xm: u32 = 0;
    let mut ym: u32 = 0;
    let mut min = 100;
    let mut sx = imgx;
    let mut sy = imgy;
    for i in 1..n {
        if n == 1 {
            break
        }
        println!("{i}/{n}");
        println!("{n}%{i} = {}", n % i);
        if n % i == 0 {
            println!("{ym}x{xm}, {}", ((n / i) as i32) - 4);
            if ((n / i) as i32) - 4 < min {
                xm = i;
                ym = n / i;
                min = ((n / i) as i32) - 4;
            }
            println!("{}, {}", xm, ym);
        }
        sx = imgx / xm;
        sy = imgy / ym;
    }
    let mut threads = vec![];
    let mut count = 0;
    for x in 0..xm {
        for y in 0..ym {
            //println!("{}, {}, {}, {}", x * sx, x * sx + sx, y * sy, y * sy + sy);
            //gen(x * imgx / xm, imgx / xm, 0, imgy / 2);
            {
                count += 1;
                let s = tx.clone();
                let p = parameters.clone();
                threads.push(thread::spawn(move || {
                    let f = gen(x * sx, x * sx + sx, y * sy, y * sy + sy, p);
                    println!("thread {count} done");
                    s.send(f).unwrap();
                }));
            }
        }
    }
    //use image::io::Reader as ImageReader;

    // for img in rx {
    //     //println!("recieved! {recv:?}");

    //     for (x, y, p) in img.enumerate_pixels() {
    //         let pixel = (*imgbuf).get_pixel_mut(x, y);
    //         let image::Rgb(data) = *p;
    //         if data[0] > 0 || data[1] > 0 || data[2] > 0 {
    //             //*pixel = image::Rgb([255, 0, 255]);
    //             *pixel = *p;
    //         }
    //     }
    //     //imgbuf.save("fractal.png").unwrap();

    //     count -= 1;
    //     if count <= 0 {
    //         return
    //     }
    // }

    //let mut image = image::ImageBuffer::new(imgx, imgy);
    //for (x, y, p) in imgbuf.enumerate_pixels_mut() {
    //    let pixel = image.get_pixel_mut(x, y);
    //    let image::Rgb(data) = *p;
    //    if data[0] > 0 || data[1] > 0 || data[2] > 0 {
    //        *pixel = *p;
    //    }
    //    *pixel = image::Rgb([0, 0, 0]);
    //}
    //imgbuf.save("fractal.png").unwrap();

    // for (_, _, p) in imgbuf.enumerate_pixels_mut() {
    //     *p = image::Rgb([255, 255, 255]);
    // }
    // for i in 0..imgx {
    //     for j in 0..i {
    //         let pixel = imgbuf.get_pixel_mut(j, i);
    //         *pixel = image::Rgb([0, 0, 255]);
    //     }
    // }
    // imgbuf.save("fractal.png").unwrap();
    // use std::time;
    // thread::sleep(time::Duration::from_millis(4000));

    // for thread in threads {
    //     thread.join().unwrap();
    //     println!("thread received");
    //     let mut img = rx.recv().unwrap();
    //     for (x, y, p) in img.enumerate_pixels_mut() {
    //         let pixel = imgbuf.get_pixel_mut(x, y);
    //         let image::Rgb(data) = *p;
    //         if data[0] > 0 || data[1] > 0 || data[2] > 0 {
    //             *pixel = *p;
    //         }
    //         *pixel = image::Rgb([0, 0, 0]);
    //     }
    //     imgbuf.save("fractal.png").unwrap();
    //     //imgbuf.save("fractal.png").unwrap();
    //     for i in 0..imgx {
    //         for j in 0..i {
    //             let pixel = imgbuf.get_pixel_mut(j, i);
    //             *pixel = image::Rgb([255, 255, 0]);
    //         }
    //     }
    // }
}
/*
fn julia(a: f32, b: f32, ca: f32, cb: f32, i: i32, max: i32) -> i32 {
    //println!("a:{}, b:{}, i:{}, max:{}", a, b, i, max);
    //let f: f32 = a.powf(2.0) + b.powf(2.0);
    let a2 = a * a;
    let b2 = b * b;
    if a2 + b2 > 4.0 {
        return i - 1;
    } else if i == max {
        return max;
    }
    julia(a2 - b2 + ca, 2.0 * a * b + cb, ca, cb, i + 1, max)
}
*/
