use tiny_renderer::geometry::Array;
use tiny_renderer::mygl::triangle;
use tiny_renderer::tga_image::*;
use tiny_renderer::model::*;
fn main(){
    let mut img = TgaImage::new(2000,2000,TgaImage::RGB);
    let m = Model::new("african_head.obj");
    let light = Array::from(vec![0.,0.,-1.]);
    let mut world_coords = vec![Array::new(3);3];
    let mut zbuffer = vec![f64::MIN;img.get_width()*img.get_height()];
    for i in 0..m.nfaces(){
        let face = m.get_face(i);
        
        for j in 0..3{
            world_coords[j] = m.get_vert(face[j] as usize);
        }
        // to vector cross to get normal vectors
        let temp = (&world_coords[2]-&world_coords[0]).cross(&(&world_coords[1]-&world_coords[0]));
        let temp = temp.normalize();
        // normal vector point multiplication,If the result is 0, they are orthogonal
        let intensity = &temp*&light;
        // ugly shader ,just color mul one rate
        let shader = (intensity * 255.) as u8;
        // if not orthogonal , then draw the pixel

        for coord in &mut world_coords{
            coord.set(0, ((coord.get(0)+1.)*(img.get_width() as f64)/2.).round()+0.5);
            coord.set(1, ((coord.get(1)+1.)*(img.get_height() as f64)/2.).round()+0.5);
        }
        if intensity > 0.{
            triangle(&world_coords[0], &world_coords[1], &world_coords[2],zbuffer.as_mut_slice(), &mut img,&TgaColor::new(shader,shader,shader,255));
        }
    }
    img.write_tga_file("output.tga", true, false);
}