use crate::{tga_image::{TgaImage, TgaColor}, geometry::Array};
/// 通过向量判断点是否位于三角形内
/// 使用向量叉乘
fn barycentric(triangle_point:[&Array;3],p:(usize,usize))->Array{
    let mut ret = Array::new(3);
    let mut temp = Array::new(3);
    ret.set(0, triangle_point[2].get(0) - triangle_point[0].get(0));
    ret.set(1, triangle_point[1].get(0) - triangle_point[0].get(0));
    ret.set(2, triangle_point[0].get(0) - p.0 as f64);
    temp.set(0, triangle_point[2].get(1) - triangle_point[0].get(1));
    temp.set(1, triangle_point[1].get(1) - triangle_point[0].get(1));
    temp.set(2, triangle_point[0].get(1) - p.1 as f64);
    let ret = ret.cross(&temp);
    if ret.get(2).abs()<1.{
        Array::from(vec![-1.,1.,1.])
    }else {
        Array::from(vec![1.-(ret.get(0)+ret.get(1))/ret.get(2),ret.get(1)/ret.get(2),ret.get(0)/ret.get(2)])
    }
}
/// 规划出要遍历的区域，减少无用遍历
/// 判断一个像素点是否位于三角形内，是的话就填充颜色
pub fn triangle(point1:&Array,point2:&Array,point3:&Array,zbuffer:&mut [f64],img:&mut TgaImage,color:&TgaColor){
    let leftx = point1.get(0).min(point2.get(0)).min(point3.get(0)).max(0.).round() as usize;
    let rightx = point1.get(0).max(point2.get(0)).max(point3.get(0)).min(img.get_width() as f64).round() as usize;
    let topy = point1.get(1).max(point2.get(1)).max(point3.get(1)).min(img.get_height() as f64).round() as usize;
    let bottomy = point1.get(1).min(point2.get(1)).min(point3.get(1)).max(0.).round() as usize;
    let point_array = [point1,point2,point3];
    let mut z;
    for y in bottomy..topy{
        for x in leftx..rightx{
            let bc_screen = barycentric(point_array, (x,y));
            if bc_screen.get(0) < 0. || bc_screen.get(1) < 0. || bc_screen.get(2) < 0.{
                continue;
            }
            z = 0.;
            z+=point_array[0].get(2)*bc_screen.get(0)+point_array[1].get(2)*bc_screen.get(1)+point_array[2].get(2)*bc_screen.get(2);
            if zbuffer[x+y*img.get_width()] < z{
                zbuffer[x+y*img.get_width()] = z;
                img.set(x, y, color);
            }
        }
    }
}