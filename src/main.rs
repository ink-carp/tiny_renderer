use tiny_renderer::tga_image::*;
use tiny_renderer::model::*;
fn main(){
    let mut img = TgaImage::new(2000,2000,TgaImage::RGB);
    let color = TgaColor::new(255,255,255,0);
    let m = Model::new("african_head.obj");
    for i in 0..m.nfaces(){
        let face = m.get_face(i);
        for j in 0..3{
            let arr0 = m.get_vert(face[j] as usize);
            let arr1 = m.get_vert(face[(j+1)%3] as usize);
            let point1 = (((arr0.get(0)+1.)*(img.get_width() as f64)/2. - 1.) as usize, ((arr0.get(1)+1.)*(img.get_height() as f64)/2. - 1.) as usize);
            let point2 = (((arr1.get(0)+1.)*(img.get_width() as f64)/2. - 1.) as usize, ((arr1.get(1)+1.)*(img.get_height() as f64)/2. - 1.) as usize);
            img.draw_line(point1, point2, &color);
        }
    }
    img.write_tga_file("output.tga", true, false);
}