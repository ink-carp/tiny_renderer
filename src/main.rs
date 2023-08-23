use tiny_renderer::geometry::Array;
use tiny_renderer::mygl::triangle;
use tiny_renderer::tga_image::*;
use tiny_renderer::model::*;
fn main(){
    let mut img = TgaImage::new(2000,2000,TgaImage::RGB);
    let color = TgaColor::new(255,255,255,0);
    let m = Model::new("african_head.obj");
    let light = Array::from(vec![0.,0.,-1.]);
    let mut screen_coords :[(i32,i32);3] = [(0,0);3];
    let mut world_coords = vec![Array::new(3);3];
    for i in 0..m.nfaces(){
        let face = m.get_face(i);
        
        for j in 0..3{
            let arr = m.get_vert(face[j] as usize);
            screen_coords[j] = (((arr.get(0)+1.)*(img.get_width() as f64)/2.) as i32,((arr.get(1)+1.)*(img.get_height() as f64)/2.) as i32); 
            world_coords[j] = arr;
        }
        let temp = (&world_coords[2]-&world_coords[0]).cross(&(&world_coords[1]-&world_coords[0]));
        let temp = temp.normalize();
        let intensity = &temp*&light;
        let shader = (intensity * 255.) as u8;
        if intensity > 0.{
            triangle(screen_coords[0], screen_coords[1], screen_coords[2], &mut img,&TgaColor::new(shader,shader,shader,255));
        }
    }
    img.write_tga_file("./output.tga", true, false);
}