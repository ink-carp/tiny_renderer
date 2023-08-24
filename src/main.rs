use tiny_renderer::tga_image::*;
const SAVE_PATH:&str = "./output.tga";
fn main(){
    let mut img = TgaImage::new(200,200,TgaImage::RGB);
    let color = TgaColor::new(255,255,255,0);
    for x in 0..img.get_width(){
        img.set(x, img.get_height()/2 , &color);
    }
    img.write_tga_file(SAVE_PATH, true, false);
}