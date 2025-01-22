
use image::{ ImageBuffer,RgbImage};
use noise::{NoiseFn, Perlin};
use rand::random;

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 1000;
const SAME: f64 = 0.1;

fn main() {

    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let noise = WorldNoise::new(60.0);

    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            img.set_rgb(x, y, get_color(&noise,x, y));
        }
    }

    img.save("World.png").unwrap();
}

fn get_color(noise: &WorldNoise,x: u32,y: u32) -> [u8; 3]{
    let value = noise.get_value(x as f64, y as f64);

    let result: [u8; 3] = 
    if isvalue(value, 1) {[27,36,71]}
    else if isvalue(value, 2) {[40,53,147]}
    else if isvalue(value, 3) {[43,78,149]}
    else if isvalue(value, 4) {[1,87,155]}
    else if isvalue(value, 5) {[2,119,189]}
    else if isvalue(value, 6) {[2,136,209]}
    else if isvalue(value, 7) {[3,155,229]}
    else if isvalue(value, 8) {[0,188,212]}
    else if isvalue(value, 9) {[38,198,218]}
    else if isvalue(value, 10) {[77,208,225]}
    else if isvalue(value, 11) {[128,222,234]}
    else if isvalue(value, 12) {[178,139,120]}
    else if isvalue(value, 13) {[246,216,150]}
    else if isvalue(value, 14) {[196,241,42]}
    else if isvalue(value, 15) {[143,208,50]}
    else if isvalue(value, 16) {[97,165,63]}
    else if isvalue(value, 17) {[71,114,56]}
    else if isvalue(value, 18) {[41,63,33]}
    else if isvalue(value, 19) {[50,50,50]}
    else if isvalue(value, 20) {[89,87,87]}
    else if isvalue(value, 21) {[128,123,122]}
    else if isvalue(value, 22) {[166,158,154]}
    else {
        //println!("{}, {}",x,y);
        [236,235,231]
    };
    
    if value > 1. {
        println!("{value}");
    }

    result
}

fn isvalue(value: f64,step: i32) -> bool{
    value >= max_value(step-1) && value < max_value(step)
}

fn max_value(step: i32) -> f64{
    step as f64/23.0
}

trait ImageTool {
    fn set_rgb(&mut self,_x: u32,_y: u32,_rgb: [u8; 3]){
        
    }
}

impl ImageTool for RgbImage {
    fn set_rgb(&mut self,x: u32,y: u32,rgb: [u8; 3]) {
        *(self.get_pixel_mut(x, y)) = image::Rgb(rgb);
    }
}

struct WorldNoise{
    main_zoom: f64,
    zoom2: f64,
    zoom3: f64,
    noise: Perlin,
    noise2: Perlin,
    noise3: Perlin
}

const E:f64 = 2.71828182846;

impl WorldNoise {
    
    fn new(zoom: f64) -> Self{
        WorldNoise{
            main_zoom: zoom,
            zoom2:zoom/2.0,
            zoom3:zoom/4.0,
            noise: Perlin::new(random()),
            noise2: Perlin::new(random()),
            noise3: Perlin::new(random())
        }
    }

    fn get_value(&self, x: f64, y: f64) -> f64{
        
        if x >= WIDTH as f64*SAME && x <= WIDTH as f64*(1.0 -SAME){
            return self.get_noise_value(x, y);
        }
        else{

            let (add_x,smooth_x) = if x < WIDTH as f64*SAME {
                (x,WIDTH as f64 + x)
            }else {
                (x - WIDTH as f64,x)
            };

            let smooth = smooth( add_x/(WIDTH as f64*SAME));
            //println!("{}",smooth);

            let main_value = self.get_noise_value(x, y);
            let smooth_value = self.get_noise_value(smooth_x, y);

            main_value*(1.0-smooth)+smooth_value*smooth
            //main_value
            
        }

    }

    fn get_noise_value(&self,x: f64,y: f64) -> f64{

        let mut main_value = self.noise.get([x/ self.main_zoom,y/self.main_zoom,0.0]) - 1.0;
        main_value = E.powf(-main_value*main_value);
        
        let mut value2 = (self.noise2.get([x/ self.zoom2,y/self.zoom2,0.0]) +1.0)/2.0;
        value2 = eioq(value2);

        let mut value3 = (self.noise3.get([x/ self.zoom3,y/self.zoom3,0.0]) +1.0)/2.0;
        value3 = eioq(value3);

        let result = (main_value*2.0+value2*1.0)/3.0*0.85+value3*0.15;

        let sea_noise_zoom = self.main_zoom*2.0;
        let mut sea_value = f64::atan(10.0*self.noise.get([x/ sea_noise_zoom,y/sea_noise_zoom,12.34]))/2.0+0.5;

        if sea_value > 1.05{
            sea_value = 1.05;
        }else if sea_value <= 0.0{
            sea_value = 0.0;
        }

        f64::min((sea_value*0.7+0.3)*result, 1.0)


    }

}

fn smooth(num: f64) -> f64{
    (1.0 - num.powi(2)).powi(3)
}

//easeInOutQuint
fn eioq(num: f64) -> f64{
    if num <0.5{
        return 16.0*num.powi(5);
    }
    1.0 - (-2.0*num+2.0).powi(5)/2.0

}
