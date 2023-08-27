use tiny_renderer::FT;
use tiny_renderer::geometry::*;
use tiny_renderer::mygl::triangle;
use tiny_renderer::tga_image::*;
use tiny_renderer::model::*;
const IMG_WIDTH:usize = 2000;
const IMG_HEIGHT:usize = 2000;
const DEEPTH:usize = 255;
const FILENAME:&str = "african_head.obj";
const OUTPUTNAME:&str = "output.tga";
// All the data type float to integer must use round(),because f32 -> i32 will loss precision

fn matrix2array(src:Matrix<FT>)->Array<FT>{
    Array::from(vec![src.get(0, 0)/src.get(3, 0),src.get(1, 0)/src.get(3, 0),src.get(2, 0)/src.get(3, 0)])
}
fn array2matrix(src:&Array<FT>)->Matrix<FT>{
    let mut ret = Matrix::new(4,1);
    ret.set(0, 0, src.get(0));
    ret.set(1, 0, src.get(1));
    ret.set(2, 0, src.get(2));
    ret.set(3, 0, 1.);
    ret
}
// 将一个二维坐标转化为四维方阵
fn viewport(x:usize,y:usize,w:usize,h:usize)->Matrix<FT>{
    let mut ret = Matrix::<FT>::identity(4, 4,1f32);
    ret.set(0, 3, x as FT + w as FT/2.);
    ret.set(1, 3, y as FT + h as FT/2.);
    ret.set(2, 3, DEEPTH as FT / 2.);

    ret.set(0, 0, w as FT/2.);
    ret.set(1, 1, h as FT/2.);
    ret.set(2, 2, DEEPTH as FT/2.);
    ret
}
fn main(){
    let m = Model::new(FILENAME);
    let light = Array::from(vec![0.,0.,-1.]);
    let camare = Array::from(vec![0.,0.,10.]);
    let mut zbuffer = vec![f32::MIN;IMG_HEIGHT*IMG_WIDTH];
    let mut projection = Matrix::identity(4, 4,1f32);
    let vp = viewport(IMG_WIDTH/8, IMG_HEIGHT/8, IMG_WIDTH*3/4, IMG_HEIGHT*3/4);
    let mut img = TgaImage::new(IMG_WIDTH,IMG_HEIGHT,TgaImage::RGB);

    projection.set(3, 2, -1./camare.get(2));
    for i in 0..m.nfaces(){
        let triangle_coords = m.get_vert(i);
        let screen_coord0 = matrix2array(&(&vp*&projection)*&array2matrix(&triangle_coords[0]));
        let screen_coord1 = matrix2array(&(&vp*&projection)*&array2matrix(&triangle_coords[1]));
        let screen_coord2 = matrix2array(&(&vp*&projection)*&array2matrix(&triangle_coords[2]));
        // to vector cross to get normal vectors
        let temp = (&triangle_coords[2]-&triangle_coords[0]).cross(&(&triangle_coords[1]-&triangle_coords[0]));
        let temp = temp.normalize();
        // normal vector point multiplication,If the result is 0, they are orthogonal
        let intensity = &temp*&light;
        if intensity > 0.{

            let mut coord0 = Array::<i32>::new(3);
            let mut coord1 = Array::<i32>::new(3);
            let mut coord2 = Array::<i32>::new(3);
            for i in 0..3{
                coord0.set(i, screen_coord0.get(i).round() as i32);
                coord1.set(i, screen_coord1.get(i).round() as i32);
                coord2.set(i, screen_coord2.get(i).round() as i32);
            }
            let texture_coords = m.get_texture_coords(i);
            triangle([coord0,coord1,coord2],texture_coords,zbuffer.as_mut_slice(), &mut img,&m,1.);
        }
    }
    img.write_tga_file(OUTPUTNAME, true, false);
}