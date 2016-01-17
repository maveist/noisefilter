extern crate image;
extern crate rand;

use std::io;
use std::env;
use std::fs::File;
use std::path::Path;
use rand::distributions::{Normal, IndependentSample};

use rand::{thread_rng, Rng};

use image::{
    GenericImage,
    ImageBuffer,
    Luma,
    Pixel
};


fn main() {

    //gestion des menus via les arguments en entrée dans l'execution de la commande cargo run    
    let args: Vec<_> = env::args().collect();
    if ((args.len() == 2) && (args[1].contains("help"))) {
        println!("How to use noisefilter:");
        println!("noisefilter [OPTION] [PATH_FILE_IN] [PATH_FILE_OUT]");
        println!("Options :");
        println!("  f : filter");
        println!("  n : noise");
        println!("  s : salt and pepper");
        println!("  g : gaussian");
        println!("  r : calculate SNR");
        println!("Exemple : cargo run gf ./test1.png ./out_test1.png");
        println!("This does a gaussian filter on your image");
    }else if (args.len() == 4){
        //gestion des options
        let img_out;
        let path = args[3].clone();
        if((args[1].contains("f")) && ((args[1].contains("s")))){
            let w = get_int_value("Please enter a w value (positive integer)");
            let string = Path::new(&args[2]);
            let img = image::open(string).unwrap();
            let (width, height) = img.dimensions();
            let mat_img = generate_matrice_pix(img);
            img_out = median_filter(mat_img.clone(), w, width, height, path);
            if(args[1].contains("r")){
                let mat_img_out = generate_matrice_pix(img_out);
                let snr = get_snr(mat_img, mat_img_out);
                println!("SNR: {} ", snr);
            }
        }else if((args[1].contains("f")) && ((args[1].contains("g")))){
            let w = get_int_value("Please enter a w value (positive integer)");
            let deviance = get_float_value("Please enter a deviance (float value)");
            let string = Path::new(&args[2]);
            println!("{}", &args[2]);
            let img = image::open(string).unwrap();
            let (width, height) = img.dimensions();
            let mat_img = generate_matrice_pix(img);
            img_out = filter_gaussian(mat_img.clone(), w, deviance, width, height, path);
            if(args[1].contains("r")){
                let mat_img_out = generate_matrice_pix(img_out);
                let snr = get_snr(mat_img, mat_img_out);
                println!("SNR: {} ", snr);
            }
        }else if((args[1].contains("n")) && ((args[1].contains("s")))){
            let w = get_int_value("Please enter a w value (positive integer)");
            let noise_ratio = get_float_value("Please enter a ratio for noise (float value)");
            let string = Path::new(&args[2]);
            let img = image::open(string).unwrap();
            let (width, height) = img.dimensions();
            let mat_img = generate_matrice_pix(img);
            img_out = noise_salt_pepper(mat_img.clone(), noise_ratio, width, height, path);
            if(args[1].contains("r")){
                let mat_img_out = generate_matrice_pix(img_out);
                let snr = get_snr(mat_img, mat_img_out);
                println!("SNR: {} ", snr);
            }
        }else if((args[1].contains("n")) && ((args[1].contains("g")))){
            let deviance = get_float_value("Please enter a deviance value (positive float)");
            let string = Path::new(&args[2]);
            let img = image::open(string).unwrap();
            let (width, height) = img.dimensions();
            let mat_img = generate_matrice_pix(img);
            img_out = noise_gaussian(mat_img.clone(), deviance, width, height, path);
            if(args[1].contains("r")){
                let mat_img_out = generate_matrice_pix(img_out);
                let snr = get_snr(mat_img, mat_img_out);
                println!("SNR: {} ", snr);
            }
        }else{
            println!("Invalid command, more information : noisefilter -help");
        }
        
    }else{
        println!("Please enter option file and path, more information : noisefilter -help");
    }

}

fn median_filter(mat_img : Vec<Vec<u8>>, w : u32, width :u32, height: u32, path: std::string::String) -> image::DynamicImage{   
    let mut img_buff = image::ImageBuffer::new(width, height);
    println!("Application medium filter");
    let mut x = 0;
    while(x+w <= width-1){
        let mut purcentage : f32 =  ((x as f32 + w as f32)/ width as f32)*100 as f32;
        println!("{} %", purcentage);
        let mut y = 0;
        while(y+w<= height-1){
            let medium = parse_matrice(x, y, w, mat_img.clone());
            for pixel_x in x..(x+w){
                for pixel_y in y..(y+w){
                   //let pixel = img.get_pixel(pixel_x, pixel_y);
                   let pixel = image::Luma([medium as u8]);
                    img_buff.put_pixel(pixel_x, pixel_y, pixel);
                }
            }
            y = y+w;
        }
        x = x+w;
    }
    
    println!("Génération image sortie en cours");
    let ref mut out = File::create(&Path::new(&path)).unwrap();
    let img_out = image::ImageLuma8(img_buff);
    let _ = img_out.save(out, image::PNG);
    return img_out;
}

fn conglo_filter(mat_img : Vec<Vec<u8>>, w : u32, width :u32, height: u32) -> image::DynamicImage{ 
    let mut img_buff = image::ImageBuffer::new(width, height);
    println!("Application conglo filter");
    let mut x = 0;
    while(x+1 <= width-16){
        let mut purcentage : f32 =  ((x as f32 + w as f32)/ width as f32)*100 as f32;
        println!("{} %", purcentage);
        let mut y = 0;
        while(y+w<= height-3){
            let medium = parse_matrice_c(x, y, w, mat_img.clone());
            for pixel_x in x..(x+w){
                for pixel_y in y..(y+w){
                    let pixel = image::Luma([medium as u8]);
                    img_buff.put_pixel(pixel_x, pixel_y, pixel);
                }
            }
            y = y+3;
        }
        x = x+3;
        
    }
    println!("Génération image sortie en cours");
    let path = "./image/out/out_conglo_filter1_.png";
    let ref mut out = File::create(&Path::new(path)).unwrap();
    let img_out = image::ImageLuma8(img_buff);
    let _ = img_out.save(out, image::PNG);
    return img_out;
}




fn parse_matrice_c(x :u32, y :u32, w :u32, mat_img : Vec<Vec<u8>>) -> u8{
    
    let mut vec: Vec<u8> = Vec::new();
    for x_cur in x..(x+w){
        for y_cur in y..(y+w){
            let x_test = x_cur as usize;
            let y_test = y_cur as usize;
            let val_tmp : Vec<u8> = mat_img[x_test].clone();
            vec.push(val_tmp[y_test]);
        }
    }
    vec.sort();
    let mut cmpt =0 as u32;
    let mut len = 0 as u32;
    for i_cur in 0..vec.len(){
        len = len +1;
        let i = vec[i_cur] as u32;
        cmpt = (cmpt + i);
    }
    return (cmpt/len )as u8;
}


fn parse_matrice(x :u32, y :u32, w :u32, mat_img : Vec<Vec<u8>>) -> u8{
    
    let mut vec: Vec<u8> = Vec::new();
    for x_cur in x..(x+w){
        for y_cur in y..(y+w){
            let x_test = x_cur as usize;
            let y_test = y_cur as usize;
            let val_tmp : Vec<u8> = mat_img[x_test].clone();
            vec.push(val_tmp[y_test]);
        }
    }
    vec.sort();
    return vec[((vec.len()-1)/2)];
}

fn generate_matrice_pix(img: image::DynamicImage) -> Vec<Vec<u8>>{
    println!("Génération matrice en cours");
    
    let (width, height) = img.dimensions();
    let mut mat_img = Vec::new();
    //let mut mat_img = vec![vec![..height], ..width];
    for x in 0..(width-1){
        let mut vec_tmp: Vec<u8> = Vec::new();
        for y in 0..(height-1){
            vec_tmp.push(img.get_pixel(x,y).to_luma().data[0]);
            //mat_img[x][y] = img.get_pixel(x,y).to_luma();
        }
        mat_img.push(vec_tmp);
    }
    println!("Génération matrice done");
    return mat_img;
}


//mat_img_a is the original image which will be compared by mat_img_b

fn get_snr(mat_img_a : Vec<Vec<u8>>, mat_img_b : Vec<Vec<u8>>) -> f64 {
    // P mat_img_a
    let mut p_signal : u32= 0;
    let mut p_noise : u32 = 0;
    for x in 0..(mat_img_a.len()){
        let mut vec_tmp : Vec<u8> = mat_img_a[x].clone();
        p_signal += vec_tmp.iter().fold(0,|p_signal, &val| p_signal + (val as u32).pow(2));
        vec_tmp = mat_img_b[x].clone();
        p_noise += vec_tmp.iter().fold(0,|p_noise, &val| p_noise + (val as u32).pow(2));
    }
    let s = (p_signal as f64 /p_noise as f64) as f64;
    let snr = s.log(10.0);
    return snr;
}


fn filter_convo(img: image::DynamicImage){
    let kernel = [0.0f32, 1.0, 0.0,
              1.0, 8.0, 1.0,
              0.0, 1.0, 0.0];
            
    let filtered = img.filter3x3(&kernel);
    let path = "./image/out/kernel1_.png";
    let ref mut out = File::create(&Path::new(path)).unwrap();
    let img_out = filtered;

    let _ = img_out.save(out, image::PNG);
}
fn filter_convo_rec(img: image::DynamicImage)->image::DynamicImage{
    let kernel = [0.0f32, 1.0, 0.0,
                 1.0, 8.0, 1.0,
                 0.0, 1.0, 0.0];
    let filtered = img.filter3x3(&kernel);
    return filtered;
}


fn filter_gaussian (mat_img : Vec<Vec<u8>>, w: u32, deviance: f64, width :u32, height: u32, path : std::string::String) -> image::DynamicImage{
    println!("W convo : {}", w);
    let mut img_buff = image::ImageBuffer::new(width, height);
    println!("Application gaussian filter");
    let mut x = 0; 
    let nucleus_x : Vec<f64> = nucleus_x(0, 0, w, deviance, mat_img.clone());
    let nucleus_y : Vec<f64> = nucleus_y(0, 0, w, deviance, mat_img.clone());
    println!("Nucleus x:");
    let mut sum = 0.0;
    for nucc_x in nucleus_x.clone(){
        for nucc_y in nucleus_y.clone(){
            sum = sum + nucc_y as f64*nucc_x as f64;
        }
    }
    while(x+w <= width-1){
        let mut purcentage : f32 =  ((x as f32 + w as f32)/ width as f32)*100 as f32;
        println!("{} %", purcentage);
        let mut y = 0;
        while(y+w<= height-1){
            let mut nume = 0.0;
            for pixel_x in x..(x+w){
                let mut nuc_x =nucleus_x[(pixel_x - x) as usize];
                for pixel_y in y..(y+w){
                    nume = nume + mat_img[pixel_x as usize][pixel_y as usize] as f64 * nuc_x * nucleus_y[(pixel_y - y) as usize] as f64;
                }
            }
            let pixel_value = nume as f64/sum as f64;
            let pixel = image::Luma([pixel_value as u8]);
            img_buff.put_pixel(x, y, pixel);
            y = y+1;
        }
        x = x+1;
    }
    println!("Generating out picture in progress");
    let ref mut out = File::create(&Path::new(&path)).unwrap();
    let img_out = image::ImageLuma8(img_buff);
    let _ = img_out.save(out, image::PNG);
    println!("Done.");
    return img_out;
}
    
fn nucleus_x(x :u32, y: u32 , w :u32, deviance : f64, mat_img : Vec<Vec<u8>>) -> Vec<f64>{
    let PI = std::f64::consts::PI;
    let mut vec: Vec<f64> = Vec::new();
    let x0 : u32 = (w+1)/2;
    let y0 : u32 = (w+1)/2;
    for x_cur in x..(x+w){
            let m = (1.0/(2.0*PI).sqrt())*deviance;
            let n = ((x_cur as f64 - x0 as f64) ).powf(2.0);
            let o = 2.0*((deviance as f64).powf(2.0));
            vec.push(m*(((-n)/o).exp()));
    }

    return vec;
}

fn nucleus_y(x :u32, y: u32 , w :u32, deviance : f64, mat_img : Vec<Vec<u8>>) -> Vec<f64>{
    let PI = std::f64::consts::PI;
    let mut vec: Vec<f64> = Vec::new();
    let x0 : u32 = (w+1)/2;
    let y0 : u32 = (w+1)/2;
    for y_cur in y..(y+w){
            let m = (1.0/(2.0*PI).sqrt())*deviance;
            let n = ((y_cur as f64 - y0 as f64) ).powf(2.0);
            let o = 2.0*((deviance as f64).powf(2.0));
            vec.push(m*(((-n)/o).exp()));
    }
   
    return vec;
}

fn get_int_value(sentence : &str) -> u32{
    println!("{}", sentence);
    let mut w = 0;
    let reader: io::Stdin = io::stdin();
    let mut str_w: String = String::new();
    let result: Result<usize, io::Error> = reader.read_line(&mut str_w);
    if result.is_err() 
    {
        println!("Erreur à la saisie.");
    }else{
        let trimmed = str_w.trim();
        match trimmed.parse::<u32>() {
            Ok(i) =>  w = i,
            Err(..) => println!("this was not an integer: {}", trimmed)
        };
    }
    return w;
}

fn get_float_value(sentence : &str) -> f64{
    println!("{}", sentence);
    let mut deviance = 0.0;
    let reader: io::Stdin = io::stdin();
    let mut str_w: String = String::new();
    let result: Result<usize, io::Error> = reader.read_line(&mut str_w);
    if result.is_err() 
    {
        println!("Erreur à la saisie.");
    }else{
        let trimmed = str_w.trim();
        match trimmed.parse::<f64>() {
            Ok(i) =>  deviance = i,
            Err(..) => println!("this was not an flaot value: {}", trimmed)
        };
    }
    return deviance;
}

fn noise_salt_pepper(mat_img : Vec<Vec<u8>>, noise_ratio : f64, width: u32, height : u32, path : std::string::String) -> image::DynamicImage{
    let mut cpt = 0;
    let mut img_buff = image::ImageBuffer::new(width, height);
    for x in 0..width-1{
        for y in 0..height-1{
            let purcentage : f32 = (cpt as f32 /(width as f32 *height as f32)*100 as f32) as f32;
            println!("{} %", purcentage);
            let pixel_put = image::Luma([noise_make_salt_pepper(mat_img[x as usize][y as usize], noise_ratio)]);
            img_buff.put_pixel(x, y, pixel_put);
            cpt = cpt +1;
        }
    }
    println!("Generating out picture in progress");
    let ref mut out = File::create(&Path::new(&path)).unwrap();
    let img_out = image::ImageLuma8(img_buff);
    let _ = img_out.save(out, image::PNG);
    println!("Done.");
    return img_out;
}

fn noise_make_salt_pepper(pix : u8, noise_ratio : f64 ) -> u8{
    if ((rand::random::<u8>()%100) > (100 - noise_ratio as u8)) {
        if rand::random(){
            let noise = 0 as u8;
            return noise;
        }else{
            let noise = 255;
           // let pixel_noise = image::Luma([noise as u8]);
            return noise;
        }
    }else{
        return pix;
    }
}

fn noise_gaussian(mat_img : Vec<Vec<u8>>, deviance : f64, width: u32, height : u32, path : std::string::String) -> image::DynamicImage{
    let PI = std::f64::consts::PI;
    let mut rng = thread_rng();
    let mean = average_pixel_value(mat_img.clone(), width, height);
    let normal_distri = Normal::new(0.0, deviance);
    println!("moyenne grey level: {}", mean);
    let mut img_buff = image::ImageBuffer::new(width, height);
    let mut cpt = 0;
    for x in (0..width-1){
        for y in (0..height-1){
            let purcentage : f32 = (cpt as f32 /(width as f32 *height as f32)*100 as f32) as f32;
            let mut grey_level = mat_img[x as usize][y as usize];
            let mut pixel_value = 0.0;
          //  if rand::random(){
                let noise = (normal_distri.ind_sample(&mut rand::thread_rng())) as f64;
              //  println!("noise pick: {}", noise);
                if((noise as f64 + grey_level as f64) > 255.0){ // verify if u8 is not overflowed
                    pixel_value = 255.0;
                }else{
                    pixel_value = noise  + grey_level as f64;
                }
          //  }else{
           //     pixel_value = grey_level as f64;
          //  }
            let pixel_put = image::Luma([pixel_value as u8]);
            img_buff.put_pixel(x, y, pixel_put);
            cpt = cpt+1;
        }
    }
    println!("Generating out picture in progress");
    let ref mut out = File::create(&Path::new(&path)).unwrap();
    let img_out = image::ImageLuma8(img_buff);
    let _ = img_out.save(out, image::PNG);
    println!("Done.");
    return img_out;
}

fn average_pixel_value(mat_img : Vec<Vec<u8>>, width : u32, height: u32) -> f64 {
    let mut sum = 0; 
    for x in 0..(mat_img.len()){
        let mut vec_tmp : Vec<u8> = mat_img[x].clone();
        sum += vec_tmp.iter().fold(0,|sum, &val| sum + val as u32);
    }
    let average = sum as f64/(width*height) as f64;
    return average;
}