use tiny_renderer::FT;
use tiny_renderer::geometry::*;
use tiny_renderer::mygl::triangle;
use tiny_renderer::tga_image::*;
use tiny_renderer::model::*;
const IMG_WIDTH:usize = 800;
const IMG_HEIGHT:usize = 800;
const DEEPTH:usize = 255;
const FILENAME:&str = "african_head.obj";
const OUTPUTNAME:&str = "output.tga";
// All the data type float to integer must use round(),because f32 -> i32 will loss precision
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

fn lookat(eye:&Array<FT>,center:&Array<FT>,up:&Array<FT>)->Matrix<FT>{
    // 新的 i,j,k,向量，基于新center和eye计算得出，up是一个总是向上(y轴)的向量
    let z = (eye-center).normalize();
    let x = up.cross(&z).normalize();
    let y = &z.cross(&x).normalize();
    let mut ret = Matrix::<FT>::identity(4, 4, 1.);
    for i in 0..3{
        ret.set(0, i, x.get(i));
        ret.set(1, i, y.get(i));
        ret.set(2, i, z.get(i));
        ret.set(i, 3, -center.get(i));
    }
    ret
}
fn main(){
    let m = Model::new(FILENAME);
    let mut img = TgaImage::new(IMG_WIDTH,IMG_HEIGHT,TgaImage::RGB);

    // 光源坐标
    let light = Array::<FT>::from(vec![-1.,0.,1.]).normalize();
    // 相机坐标
    let camare = Array::from(vec![-1.,1.,3.]);
    // 坐标系原点
    let center = Array::<FT>::from(vec![0.,0.,0.]);
    // z缓存，记录像素点前后关系
    let mut zbuffer = vec![FT::MIN;IMG_HEIGHT*IMG_WIDTH];
    // 相机的位置不再是只位于z轴上
    let mut projection = Matrix::identity(4, 4,1f32);
    projection.set(3, 2, -1./(&camare-&center).norm());
    // 视线端口，用于将 单位正方形映射到屏幕长宽高上
    let vp = viewport(IMG_WIDTH/8, IMG_HEIGHT/8, IMG_WIDTH*3/4, IMG_HEIGHT*3/4);
    
    let modelview = lookat(&camare, &center, &Array::<f32>::from(vec![0.,1.,0.]));
    let z = &(&vp*&projection)*&modelview;
    for i in 0..m.nfaces(){
        let triangle_coords = m.get_vert(i);
        let mut intensity = [0f32;3];
        let screen_coord0 = Matrix::to_array(&z*&Array::to_matrix(&triangle_coords[0]));
        let screen_coord1 = Matrix::to_array(&z*&Array::to_matrix(&triangle_coords[1]));
        let screen_coord2 = Matrix::to_array(&z*&Array::to_matrix(&triangle_coords[2]));
        // normal vector point multiplication,If the result is 0, they are orthogonal
        let mut coord0 = Array::<i32>::new(3);
        let mut coord1 = Array::<i32>::new(3);
        let mut coord2 = Array::<i32>::new(3);
        for (j,item) in intensity.iter_mut().enumerate(){
            *item = &m.get_norm(i, j)*&light;
            coord0.set(j, screen_coord0.get(j).round() as i32);
            coord1.set(j, screen_coord1.get(j).round() as i32);
            coord2.set(j, screen_coord2.get(j).round() as i32);
        }
        let texture_coords = m.get_texture_coords(i);
        triangle([coord0,coord1,coord2],texture_coords,zbuffer.as_mut_slice(), &mut img,&m,intensity);
    }
    img.write_tga_file(OUTPUTNAME, true, false);
}