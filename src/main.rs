#[macro_use]
extern crate piston_window;
extern crate image;
extern crate ndarray;
extern crate ndarray_linalg;

use piston_window::*;
use std::process::Command;
use image::{ImageBuffer, Pixel, Rgba};
use std::intrinsics::{sqrtf32, sqrtf64};

pub struct Renderer {
    window: PistonWindow,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Renderer {
    fn new(title: &str, bounds: (u32, u32)) -> Renderer
    {
        Renderer {
            window: WindowSettings::new(title, [bounds.0, bounds.1])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V4_0)
        .build()
        .unwrap(),
            image: image::ImageBuffer::new(bounds.0, bounds.1)
        }
    }
    //grad A+(B-A)t
    fn draw_line_raw(&mut self, point1: (u32,u32), point2: (u32, u32), pixels: &mut Vec<u8>)
    {
        //let mut obj = self;
        let x: i32 = point1.0 as i32 - point2.0 as i32;
        let y: i32 = point1.1 as i32 - point2.1 as i32;

        let width = self.image.width();
        let height = self.image.height();

        let tg = (y as f32)/(x as f32);

        println!("{} {}", width, (width*height) as usize);

        //let mut pixels = Vec::with_capacity((width*height*4) as usize);

        //println!("{} {}", pixelscont.capacity(), (width*height) as u32);

        //pixels[(point1.0 + self.image.width()*point1.1) as usize] = image::Rgba([0, 0, 0, 0xff]);
        (*pixels)[(point1.0*4 + self.image.width()*point1.1*4) as usize] = 255u8;
        (*pixels)[(point1.0*4 + self.image.width()*point1.1*4 + 1) as usize] = 0u8;
        (*pixels)[(point1.0*4 + self.image.width()*point1.1*4 + 2) as usize] = 0u8;
        (*pixels)[(point1.0*4 + self.image.width()*point1.1*4 + 3) as usize] = 255u8;
        //pixels[point2.0 + self.image.width()*point2.1] = image::Rgba([0, 0, 0, 0xff]);
        (*pixels)[(point2.0*4 + self.image.width()*point1.1*4) as usize] = 255u8;
        (*pixels)[(point2.0*4 + self.image.width()*point1.1*4+1) as usize] = 0u8;
        (*pixels)[(point2.0*4 + self.image.width()*point1.1*4+2) as usize] = 0u8;
        (*pixels)[(point2.0*4 + self.image.width()*point1.1*4+3) as usize] = 255u8;

        if  !(-1f32 < tg && tg < 1f32)
        {
            if point1.1 < point2.1
            {
                for i in (point1.1..point2.1).step_by(1) {
                    let x = (point1.0 as i32 + ((1f32/tg)*(i as f32 - point1.1 as f32 ))as i32) as u32 ;
                    //pixels[x + self.image.width()*i] = image::Rgba([0, 0, 0, 0xff]);
                    (*pixels)[(x*4 + self.image.width()*i*4) as usize] = 0u8;
                    (*pixels)[(x*4 + self.image.width()*i*4+1) as usize] = 0u8;
                    (*pixels)[(x*4 + self.image.width()*i*4+2) as usize] = 0u8;
                    (*pixels)[(x*4 + self.image.width()*i*4+3) as usize] = 255u8;
                }
            }
            else
            {
                for i in (point2.1*4..point1.1*4).step_by(4)  {
                    let x = (point2.0 as i32 + ((1f32/tg)*(i as f32 - point2.1 as f32 ))as i32) as u32 ;
                    //pixels[x + self.image.width()*i] = image::Rgba([0, 0, 0, 0xff]);
                    (*pixels)[(x*4 + self.image.width()*i*4) as usize] = 0u8;
                    (*pixels)[(x*4 + self.image.width()*i*4+1) as usize] = 0u8;
                    (*pixels)[(x*4 + self.image.width()*i*4+2) as usize] = 0u8;
                    (*pixels)[(x*4 + self.image.width()*i*4+3) as usize] = 255u8;
                }
            }

        }
        else {
            if point1.0 < point2.0
            {
                // println!("2: {} {} {:?}", tg, point1.1, point1.0..point2.0);
                for i in point1.0..point2.0 {
                    let y = (point1.1 as i32 + ((tg) * (i as f32 - point1.0 as f32)) as i32) as u32;
                    //pixels[i + self.image.width()*y] = image::Rgba([0, 0, 0xff, 0xff]);
                    (*pixels)[(i*4 + self.image.width()*y*4) as usize] = 0u8;
                    (*pixels)[(i*4 + self.image.width()*y*4+1) as usize] = 0u8;
                    (*pixels)[(i*4 + self.image.width()*y*4+2) as usize] = 0u8;
                    (*pixels)[(i*4 + self.image.width()*y*4+3) as usize] = 255u8;
                }
            } else {
                for i in point2.0..point1.0 {
                    let y = (point2.1 as i32 + ((tg) * (i as f32 - point2.0 as f32)) as i32) as u32;
                    //pixels[i + self.image.width()*y] = image::Rgba([0, 0xff, 0, 0xff]);
                    (*pixels)[(i*4 + self.image.width()*y*4) as usize] = 0u8;
                    (*pixels)[(i*4 + self.image.width()*y*4+1) as usize] = 0u8;
                    (*pixels)[(i*4 + self.image.width()*y*4+2) as usize] = 0u8;
                    (*pixels)[(i*4 + self.image.width()*y*4+3) as usize] = 255u8;
                }
            }
        }
        //let c = image::ImageBuffer::from_vec(width as u32, height as u32, pixels);
        //(*self).image = c.unwrap();
        //println!("{:?}", pixels);

    }

    fn draw(&mut self, point1: (u32,u32), point2: (u32, u32))
    {

        let width = self.image.width();
        let height = self.image.height();

        let mut pixels = Vec::with_capacity((width*height*4) as usize);
        for i in 0..pixels.capacity() {
            pixels.push(0u8);
        }
        self.draw_line_raw(point1, point2, &mut pixels);
        let c = image::ImageBuffer::from_vec(width as u32, height as u32, pixels);
        (*self).image = c.unwrap();
    }

    fn paint_gradient_triangle_raw(&mut self, sorted_points: ((u32,u32,u8,u8,u8), (u32,u32,u8,u8,u8), (u32,u32,u8,u8,u8)), pixels: &mut Vec<u8>)
    {
        let mut x1;
        let mut x2;
        let mut x: i32 = (sorted_points.1).0 as i32 - (sorted_points.0).0 as i32;
        let mut y: i32 = (sorted_points.1).1 as i32 - (sorted_points.0).1 as i32;
        let tg1 = 1f32/((y as f32)/(x as f32));
        println!("{}", tg1);

        x = (sorted_points.2).0 as i32 - (sorted_points.0).0 as i32;
        y = (sorted_points.2).1 as i32 - (sorted_points.0).1 as i32;
        let tg2 = 1f32/((y as f32)/(x as f32));

        x = (sorted_points.1).0 as i32 - (sorted_points.2).0 as i32;
        y = (sorted_points.1).1 as i32 - (sorted_points.2).1 as i32;
        let tg3 = 1f32/((y as f32)/(x as f32));

        for i in (sorted_points.2).1..(sorted_points.1).1 {
            x1 = ((sorted_points.2).0 as i32 + (tg3*(i as f32 - (sorted_points.2).1 as f32 )) as i32) as u32;
            x2 = ((sorted_points.2).0 as i32 + (tg2*(i as f32 - (sorted_points.2).1 as f32 )) as i32) as u32;

            //println!("{} {} {}", x1,x2 , i);
            if(x1 < x2)
            {
                for j in x1+1..x2 {
                    let distance = unsafe { sqrtf64(((i - (sorted_points.0).1) * (i-(sorted_points.0).1) + ((sorted_points.0).0-j) * ((sorted_points.0).0-j)) as f64) };
                    println!("{}", distance);
                    let red = 0u8;
                    let green = 0u8;
                    let blue = 0u8;


                    (*pixels)[(j*4 + self.image.width()*i*4) as usize] = red;
                    (*pixels)[(j*4 + self.image.width()*i*4+1) as usize] = green;
                    (*pixels)[(j*4 + self.image.width()*i*4+2) as usize] = blue;
                    (*pixels)[(j*4 + self.image.width()*i*4+3) as usize] = 255u8;
                }
            }
            else
            {
                for j in x2+1..x1 {
                    (*pixels)[(j*4 + self.image.width()*i*4) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+1) as usize] = 255u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+2) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+3) as usize] = 255u8;
                }
            }
        }


        fn paint_triangle_raw(&mut self, sorted_points: ((u32,u32), (u32,u32), (u32,u32)), pixels: &mut Vec<u8>)
    {
        let mut x1;
        let mut x2;
        let mut x: i32 = (sorted_points.1).0 as i32 - (sorted_points.0).0 as i32;
        let mut y: i32 = (sorted_points.1).1 as i32 - (sorted_points.0).1 as i32;
        let tg1 = 1f32/((y as f32)/(x as f32));
        println!("{}", tg1);

        x = (sorted_points.2).0 as i32 - (sorted_points.0).0 as i32;
        y = (sorted_points.2).1 as i32 - (sorted_points.0).1 as i32;
        let tg2 = 1f32/((y as f32)/(x as f32));

        x = (sorted_points.1).0 as i32 - (sorted_points.2).0 as i32;
        y = (sorted_points.1).1 as i32 - (sorted_points.2).1 as i32;
        let tg3 = 1f32/((y as f32)/(x as f32));

        for i in (sorted_points.2).1..(sorted_points.1).1 {
            x1 = ((sorted_points.2).0 as i32 + (tg3*(i as f32 - (sorted_points.2).1 as f32 )) as i32) as u32;
            x2 = ((sorted_points.2).0 as i32 + (tg2*(i as f32 - (sorted_points.2).1 as f32 )) as i32) as u32;

            //println!("{} {} {}", x1,x2 , i);
            if(x1 < x2)
            {
                for j in x1+1..x2 {
                    (*pixels)[(j*4 + self.image.width()*i*4) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+1) as usize] = 255u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+2) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+3) as usize] = 255u8;
                }
            }
            else
            {
                for j in x2+1..x1 {
                    (*pixels)[(j*4 + self.image.width()*i*4) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+1) as usize] = 255u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+2) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+3) as usize] = 255u8;
                }
            }
        }

        for i in (sorted_points.1).1..(sorted_points.0).1 {
            let x1 = ((sorted_points.2).0 as i32 + (tg2*(i as f32 - (sorted_points.2).1 as f32 )) as i32) as u32;
            let x2 = ((sorted_points.1).0 as i32 + ((tg1*(i as f32 - (sorted_points.1).1 as f32)) as i32)) as u32; // x0 + ky
            println!("{} {} {}", x1, x2 ,i);
            if(x1 < x2)
            {
                for j in x1+1..x2 {
                    (*pixels)[(j*4 + self.image.width()*i*4) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+1) as usize] = 255u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+2) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+3) as usize] = 255u8;
                }
            }
            else
            {
                for j in x2+1..x1 {
                    (*pixels)[(j*4 + self.image.width()*i*4) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+1) as usize] = 255u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+2) as usize] = 0u8;
                    (*pixels)[(j*4 + self.image.width()*i*4+3) as usize] = 255u8;
                }
            }
        }
    }

    fn draw_triangle_raw(&mut self, points: ((u32,u32), (u32,u32), (u32,u32)))
    {
        let width = self.image.width();
        let height = self.image.height();
        let mut pixels = Vec::with_capacity((width*height*4) as usize);
        for i in 0..pixels.capacity() {
            pixels.push(0u8);
        }

        if (points.0).1 > (points.1).1
        {
            // 0 > 1
            if(points.1).1 > (points.2).1
            {
                // 0 > 1 > 2
                self.paint_triangle_raw(((points.0),(points.1),(points.2)), &mut pixels);
            }
            else {
                // 0 > 1 && 1 <= 2
                if (points.0).1 > (points.2).1
                {
                    self.paint_triangle_raw(((points.0), (points.2), (points.1)), &mut pixels);
                }
                else
                {
                    self.paint_triangle_raw(((points.2), (points.0), (points.1)), &mut pixels);
                }

            }
        }
        else
        {
            // 0 <= 1
            if (points.0).1 > (points.2).1
            {
                // 2 < 0 <= 1
                self.paint_triangle_raw(((points.1),(points.0),(points.2)), &mut pixels);
            }
            else
            {
                // 0 <= 2 && 0 <= 1
                if (points.1).1 > (points.2).1
                {
                    self.paint_triangle_raw(((points.1),(points.2),(points.0)), &mut pixels);
                }
                else
                {
                    // 0 <= 2 && 1 <= 2 && 0 <= 2
                    self.paint_triangle_raw(((points.2),(points.1),(points.0)), &mut pixels);
                }
            }
        }

        self.draw_line_raw(points.0,points.1, &mut pixels);
        self.draw_line_raw(points.0,points.2, &mut pixels);
        self.draw_line_raw(points.1,points.2, &mut pixels);

        let c = image::ImageBuffer::from_vec(width as u32, height as u32, pixels);
        (*self).image = c.unwrap();
    }

    fn draw_line(&mut self, point1: (u32,u32), point2: (u32, u32))
    {
        self.image.put_pixel(point1.0, point1.1, image::Rgba([0xff, 0, 0, 0xff]));
        self.image.put_pixel(point2.0, point2.1, image::Rgba([0xff, 0, 0, 0xff]));
        let x: i32 = point1.0 as i32 - point2.0 as i32;
        let y: i32 = point1.1 as i32 - point2.1 as i32;


        let tg = (y as f32)/(x as f32);
        //println!("{}", 1f32/tg);

        if  !(-1f32 < tg && tg < 1f32)
        {
            if point1.1 < point2.1
            {
                for i in point1.1..point2.1 {
                    let x = (point1.0 as i32 + ((1f32/tg)*(i as f32 - point1.1 as f32 ))as i32) as u32 ;
                    self.image.put_pixel(x, i, image::Rgba([0, 0, 0, 0xff]));
                }
            }
            else
            {
                for i in point2.1..point1.1 {
                    let x = (point2.0 as i32 + ((1f32/tg)*(i as f32 - point2.1 as f32 ))as i32) as u32 ;
                    self.image.put_pixel(x, i, image::Rgba([0, 0, 0, 0xff]));
                }
            }

        }
        else
        {
            if point1.0 < point2.0
            {
               // println!("2: {} {} {:?}", tg, point1.1, point1.0..point2.0);
                for i in point1.0..point2.0 {
                    let y = (point1.1 as i32 + ((tg) * (i as f32 - point1.0 as f32 )) as i32) as u32;
                        self.image.put_pixel(i, y, image::Rgba([0, 0, 0, 0xff]));
                }
            }
            else
            {
                for i in point2.0..point1.0 {
                    let y = (point2.1 as i32 + ((tg)*(i as f32 - point2.0 as f32 ))as i32) as u32 ;
                        self.image.put_pixel(i, y, image::Rgba([0, 0, 0, 0xff]));
                }
            }
        }

    }

    fn paint_triangle(&mut self, sorted_points: ((u32,u32), (u32,u32), (u32,u32)))
    {
        let mut x1;
        let mut x2;
        let mut x: i32 = (sorted_points.1).0 as i32 - (sorted_points.0).0 as i32;
        let mut y: i32 = (sorted_points.1).1 as i32 - (sorted_points.0).1 as i32;
        let tg1 = 1f32/((y as f32)/(x as f32));
        println!("{}", tg1);

        x = (sorted_points.2).0 as i32 - (sorted_points.0).0 as i32;
        y = (sorted_points.2).1 as i32 - (sorted_points.0).1 as i32;
        let tg2 = 1f32/((y as f32)/(x as f32));

        x = (sorted_points.1).0 as i32 - (sorted_points.2).0 as i32;
        y = (sorted_points.1).1 as i32 - (sorted_points.2).1 as i32;
        let tg3 = 1f32/((y as f32)/(x as f32));

        for i in (sorted_points.2).1..(sorted_points.1).1 {
            x1 = ((sorted_points.2).0 as i32 + (tg3*(i as f32 - (sorted_points.2).1 as f32 )) as i32) as u32;
            x2 = ((sorted_points.2).0 as i32 + (tg2*(i as f32 - (sorted_points.2).1 as f32 )) as i32) as u32;

            //println!("{} {} {}", x1,x2 , i);
            if(x1 < x2)
            {
                for j in x1+1..x2 {
                    self.image.put_pixel(j, i, image::Rgba([0, 0xff, 0, 0xff]));
                }
            }
            else
            {
                for j in x2+1..x1 {
                    self.image.put_pixel(j, i, image::Rgba([0, 0xff, 0, 0xff]));
                }
            }
        }

        for i in (sorted_points.1).1..(sorted_points.0).1 {
            let x1 = ((sorted_points.2).0 as i32 + (tg2*(i as f32 - (sorted_points.2).1 as f32 )) as i32) as u32;
            let x2 = ((sorted_points.1).0 as i32 + ((tg1*(i as f32 - (sorted_points.1).1 as f32)) as i32)) as u32; // x0 + ky
            println!("{} {} {}", x1, x2 ,i);
            if(x1 < x2)
            {
                for j in x1+1..x2 {
                    self.image.put_pixel(j, i, image::Rgba([0, 0xff, 0, 0xff]));
                }
            }
            else
            {
                for j in x2+1..x1 {
                    self.image.put_pixel(j, i, image::Rgba([0, 0xff, 0, 0xff]));
                }
            }
        }
    }

    fn draw_triangle(&mut self, points: ((u32,u32), (u32,u32), (u32,u32)))
    {

        if (points.0).1 > (points.1).1
        {
            // 0 > 1
            if(points.1).1 > (points.2).1
            {
                // 0 > 1 > 2
                self.paint_triangle(((points.0),(points.1),(points.2)));
            }
            else {
                // 0 > 1 && 1 <= 2
                if (points.0).1 > (points.2).1
                {
                    self.paint_triangle(((points.0), (points.2), (points.1)));
                }
                else
                {
                    self.paint_triangle(((points.2), (points.0), (points.1)));
                }

            }
        }
        else
        {
            // 0 <= 1
            if (points.0).1 > (points.2).1
            {
                // 2 < 0 <= 1
                self.paint_triangle(((points.1),(points.0),(points.2)));
            }
            else
            {
                // 0 <= 2 && 0 <= 1
                if (points.1).1 > (points.2).1
                {
                    self.paint_triangle(((points.1),(points.2),(points.0)));
                }
                else
                {
                    // 0 <= 2 && 1 <= 2 && 0 <= 2
                    self.paint_triangle(((points.2),(points.1),(points.0)));
                }
            }
        }
        self.draw_line(points.0,points.1);
        self.draw_line(points.0,points.2);
        self.draw_line(points.1,points.2);
    }

    fn Render(&mut self)
    {
        //self.draw_line((0,0),(319,239));
        //self.draw_triangle(((0,0), (160,120), (0,60)));

        let mut texture_context = TextureContext {
            factory: self.window.factory.clone(),
            encoder: self.window.factory.create_command_buffer().into()
        };

        let texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &self.image,
            &TextureSettings::new(),
        ).unwrap();

        // The window event loop.
        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |context, graphics, _| {
                clear([1.0; 4], graphics);
                image(&texture,
                      context.transform,
                      graphics);
            });
        }
        let _ = Command::new("pause").status();
    }
}

fn draw_line(point1: (u32,u32), point2: (u32, u32))
{
    let _opengl = OpenGL::V3_2;
    let (width, height) = (320, 240);
    let _bounds = (1u32, 2u32);

    let mut window: PistonWindow =
        WindowSettings::new("Printer", [width, height])
            .exit_on_esc(true)
            .graphics_api(_opengl)
            .build()
            .unwrap();

    // Since we cant manipulate pixels directly, we need to manipulate the pixels on a canvas.
    // Only issue is that sub-pixels exist (which is probably why the red pixel looks like a smear on the output image)
    let mut canvas = image::ImageBuffer::new(width, height);
    canvas.put_pixel(point1.0, point1.1, image::Rgba([0xff, 0, 0, 0xff]));
    canvas.put_pixel(point2.0, point2.1, image::Rgba([0xff, 0, 0, 0xff]));
    let x: i32 = point1.0 as i32 - point2.0 as i32;
    let y: i32 = point1.1 as i32 - point2.1 as i32;


    let tg = (y as f32)/(x as f32);

    if  !(-1f32 < tg && tg < 1f32)
    {
        if point1.1 < point2.1
        {
            for i in point1.1..point2.1 {
                let x = (point1.0 as i32 + ((1f32/tg)*(i as f32))as i32) as u32 ;
                canvas.put_pixel(x, i, image::Rgba([0, 0, 0, 0xff]));
            }
        }
        else
        {
            for i in point2.1..point1.1 {
                let x = (point2.0 as i32 + ((1f32/tg)*(i as f32))as i32) as u32 ;
                canvas.put_pixel(x, i, image::Rgba([0, 0, 0, 0xff]));
            }
        }

    }
    else
    {
        if point1.0 < point2.0
        {
            println!("2: {} {} {:?}", tg, point1.1, point1.0..point2.0);
            for i in point1.0..point2.0 {
                let y = (point1.1 as i32 + ((tg) * (i as f32)) as i32) as u32;
                println!("{}", y);
                if y < 240
                {
                    canvas.put_pixel(i, y, image::Rgba([0, 0, 0, 0xff]));
                }
            }
        }
        else
        {
            for i in point2.0..point1.0 {
                let y = (point2.1 as i32 + ((tg)*(i as f32))as i32) as u32 ;
                if y < 240
                {
                    canvas.put_pixel(i, y, image::Rgba([0, 0, 0, 0xff]));
                }
            }
        }
    }



    // Transform into a texture so piston can use it.g
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    let texture: G2dTexture = Texture::from_image(
        &mut texture_context,
        &canvas,
        &TextureSettings::new(),
    ).unwrap();

    // The window event loop.
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);
            image(&texture,
                  context.transform,
                  graphics);
        });
    }
    let _ = Command::new("pause").status();
}

fn main() {
    //draw_line((0,0),(319,239))
    let mut render = Renderer::new("Printer", (1280,720));
    render.draw_triangle_raw(((0,0), (1279,719), (640,640)));
    //render.draw((0,0),(319,239));
    render.Render();
}