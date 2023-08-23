use crate::{tga_image::{TgaImage, TgaColor}, geometry::Array};
/// 通过向量判断点是否位于三角形内
/// 使用向量叉乘
fn barycentric(triangle_point:[(i32,i32);3],_p:(i32,i32))->Array{
    let mut ret = Array::new(3);
    let mut temp = Array::new(3);
    ret.set(0, triangle_point[2].0 as f64 - triangle_point[0].0 as f64);
    ret.set(1, triangle_point[1].0 as f64 - triangle_point[0].0 as f64);
    ret.set(2, triangle_point[0].0 as f64 - _p.0 as f64);
    temp.set(0, triangle_point[2].1 as f64 - triangle_point[0].1 as f64);
    temp.set(1, triangle_point[1].1 as f64 - triangle_point[0].1 as f64);
    temp.set(2, triangle_point[0].1 as f64 - _p.1 as f64);
    let ret = ret.cross(&temp);
    if ret.get(2).abs()<1.{
        Array::from(vec![-1.,1.,1.])
    }else {
        Array::from(vec![1.-(ret.get(0)+ret.get(1))/ret.get(2),ret.get(1)/ret.get(2),ret.get(0)/ret.get(2)])
    }
}
/// 规划出要遍历的区域，减少无用遍历
/// 判断一个像素点是否位于三角形内，是的话就填充颜色
pub fn triangle(point1:(i32,i32),point2:(i32,i32),point3:(i32,i32),img:&mut TgaImage,color:&TgaColor){
    let leftx = point1.0.min(point2.0).min(point3.0).max(0);
    let rightx = point1.0.max(point2.0).max(point3.0).min(img.get_width() as i32);
    let topy = point1.1.max(point2.1).max(point3.1).min(img.get_height() as i32);
    let bottomy = point1.1.min(point2.1).min(point3.1).max(0);
    let point_array = [point1,point2,point3];
    for y in bottomy..topy{
        for x in leftx..rightx{
            let bc_screen = barycentric(point_array, (x,y));
            if bc_screen.get(0) < 0. || bc_screen.get(1) < 0. || bc_screen.get(2) < 0.{
                continue;
            }
            img.set(x as usize, y as usize, color);
        }
    }
}