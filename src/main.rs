mod vec;
mod color;
mod ray;



use vec::unit_vector;
use vec::Vec3 as Color;
use vec::dot;
use vec::Vec3;
use color::write_color;
use ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc: Vec3 = center - r.origin();

    let a =  r.direction().length_squared();
    let h = vec::dot(r.dir, oc);
    let c = oc.length_squared() - radius*radius;
    let discriminant = h*h - a*c;

    if discriminant < -1.0 {
        return -1.0;
    }

    else {
        return  (h - f64::sqrt(discriminant))/a;
    }

}   

fn ray_color(r: Ray) -> Color {
    let center = Vec3{e : [0.0, 0.0, -1.0]};

    let t = hit_sphere(center, 0.5, r);
    
    if t > 0.0 {
        let N = unit_vector(r.at(t) - center);
        return  0.5*(N +Vec3{e: [1.0, 1.0, 1.0]});
    }


    let unit_direction = vec::unit_vector(r.dir);

    let a = 0.5 * (1.0 + unit_direction.y());

    Color{e: [1.0, 1.0, 1.0]} * ((1.0-a)) + Color{e: [0.5, 0.7, 1.0]}*a

}

fn main() {
    // println!("Hello, world!");
    
    // let image_height : f64 = 256.0;
    let aspect_ratio: f64 = 16.0 / 9.0; // width/heigh
    let image_width : i32 = 400;


    let mut image_height: i32   = ((image_width as f64) / aspect_ratio) as  i32;
    
    if image_height < 1 {image_height = 1;}
    // image_height = (image_height < 1) ? 1 : image_height;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/image_height as f64);
    let camera_center = vec::Vec3::new();

    // is the horizontal vector across viewport
    let viewport_u: Vec3 = Vec3{e:[viewport_width, 0.0, 0.0]};
    let viewport_v: Vec3 = Vec3{e: [0.0, -viewport_height, 0.0]};

    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    let viewport_upper_left = camera_center 
                                    - Vec3{e: [0.0, 0.0, focal_length]}
                                    - viewport_u / 2.0 
                                    - viewport_v / 2.0;
    
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    

    print!("P3\n{image_width} {image_height}\n255\n");

    // eprintln!("{:?}",pixel_delta_u);

    for j in 0..(image_height) {

        eprint!("\rnumber of lines remaining: {} ", (image_height as i32 - j));

        for i in 0..(image_width) {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;

            let r = ray::Ray{orig:camera_center,
                    dir: ray_direction};
            
            let pixel_color = ray_color(r);
            write_color(pixel_color);

           }

    }

}
