use std::mem::swap;

use crate::tga_image::TgaColor;
use crate::{tga_image::TgaImage, geometry::Array, model::Model};
use crate::FT;
/// 通过向量判断点是否位于三角形内
/// 使用向量叉乘
#[allow(dead_code)]
fn barycentric(triangle_point:[&Array<FT>;3],p:(usize,usize))->Array<FT>{
    let mut ret = Array::<FT>::new(3);
    let mut temp = Array::<FT>::new(3);
    ret.set(0, triangle_point[2].get(0) - triangle_point[0].get(0));
    ret.set(1, triangle_point[1].get(0) - triangle_point[0].get(0));
    ret.set(2, triangle_point[0].get(0) - p.0 as FT);
    temp.set(0, triangle_point[2].get(1) - triangle_point[0].get(1));
    temp.set(1, triangle_point[1].get(1) - triangle_point[0].get(1));
    temp.set(2, triangle_point[0].get(1) - p.1 as FT);
    let ret = ret.cross(&temp);
    if ret.get(2).abs()<1.{
        Array::from(vec![-1.,1.,1.])
    }else {
        Array::from(vec![1.-(ret.get(0)+ret.get(1))/ret.get(2),ret.get(1)/ret.get(2),ret.get(0)/ret.get(2)])
    }
}
/// 规划出要遍历的区域，减少无用遍历
/// 判断一个像素点是否位于三角形内，是的话就填充颜色
pub fn triangle(mut triangles:[Array<i32>;3],mut textures:[[i32;2];3],zbuffer:&mut [f32],img:&mut TgaImage,model:&Model,intensity:f32){
    if triangles[0].get(1) == triangles[1].get(1) && triangles[0].get(1) == triangles[2].get(1){return;}
    if triangles[0].get(1) > triangles[1].get(1) {triangles.swap(0, 1);textures.swap(0, 1)};
    if triangles[0].get(1) > triangles[2].get(1) {triangles.swap(0, 2);textures.swap(0, 2)};
    if triangles[1].get(1) > triangles[2].get(1) {triangles.swap(1, 2);textures.swap(1, 2)};

    let total_height = triangles[2].get(1)-triangles[0].get(1); // height2_0
    let height1_0 = triangles[1].get(1)-triangles[0].get(1);
    let height2_1 = triangles[2].get(1) - triangles[1].get(1);
    let new_color = |mut src:TgaColor|->TgaColor {
        for i in 0..3{
            src.set(i, (src.get(i) as f32 * intensity) as u8);
        }
        src
    };
    for y in 0..=total_height{

        let second_half = y > height1_0 || height1_0 == 0;
        let segment_height = if second_half { height2_1 } else { height1_0 };
        let alpha = y as f32 / (total_height as f32);// for point 2-0
        let beta = if second_half { (y - height1_0) as f32 / segment_height as f32 } else { y as f32 / segment_height as f32};//for point 1-2 1-0


        let mut point2_0 = Array::<i32>::new(3);
        let mut point1 = Array::<i32>::new(3);
        let mut uv2_0 = [textures[0][0] + ((textures[2][0]-textures[0][0]) as f32*alpha).round() as i32,
                                    textures[0][1] + ((textures[2][1]-textures[0][1]) as f32*alpha).round() as i32];
        let mut uv1 = if second_half {
            [textures[1][0] + ((textures[2][0]-textures[1][0]) as f32*beta).round() as i32,textures[1][1] + ((textures[2][1]-textures[1][1]) as f32*beta).round()as i32]
        }else{
            [textures[0][0] + ((textures[1][0]-textures[0][0]) as f32*beta).round() as i32,textures[0][1] + ((textures[1][1]-textures[0][1]) as f32*beta).round() as i32]
        };
        let temp = &triangles[2]-&triangles[0];
        for x in 0..3{
            point2_0.set(x, triangles[0].get(x)+(temp.get(x) as f32 * alpha).round() as i32);
        }
        if second_half {
            let temp = &triangles[2]-&triangles[1];
            for x in 0..3{
                point1.set(x,  triangles[1].get(x) + (temp.get(x) as f32 * beta).round() as i32);
            }
        }
        else {
            let temp = &triangles[1]-&triangles[0];
            for x in 0..3{
                point1.set(x,  triangles[0].get(x) + (temp.get(x) as f32 * beta).round() as i32);
            }
        }

        if point2_0.get(0) < point1.get(0){swap(&mut point2_0, &mut point1);swap(&mut uv2_0, &mut uv1)};
        for x in point1.get(0)..=point2_0.get(0){
            let dx = if point2_0.get(0) == point1.get(0) { 1. } else { (x - point1.get(0)) as f32 / (point2_0.get(0) - point1.get(0)) as f32 };
            let mut current_point = Array::<i32>::new(3);
            let real_uv = [uv1[0] + ((uv2_0[0]-uv1[0]) as f32 * dx).round() as i32,uv1[1] + ((uv2_0[1]-uv1[1]) as f32 * dx).round() as i32];
            let temp = &point2_0-&point1;
            for x in 0..3{
                current_point.set(x, point1.get(x) + (temp.get(x) as f32 * dx).round() as i32);
            }
            let index = current_point.get(0) as usize + current_point.get(1) as usize*img.get_width();
            if zbuffer[index] < current_point.get(2) as f32{
                zbuffer[index] = current_point.get(2) as f32;
                let color = new_color(model.get_diffuse(real_uv[0] as usize, real_uv[1] as usize));
                img.set(current_point.get(0) as usize, current_point.get(1) as usize,&color);
            }
        }
    }
    //println!("{}",draw_pixel);
    // let leftx = triangles[0].get(0).min(triangles[1].get(0)).min(triangles[2].get(0)).max(0.).round() as usize;
    // let rightx = triangles[0].get(0).max(triangles[1].get(0)).max(triangles[2].get(0)).min(img.get_width() as FT).round() as usize;
    // let topy = triangles[0].get(1).max(triangles[1].get(1)).max(triangles[2].get(1)).min(img.get_height() as FT).round() as usize;
    // let bottomy = triangles[0].get(1).min(triangles[1].get(1)).min(triangles[2].get(1)).max(0.).round() as usize;
    // let point_array = [&triangles[0],&triangles[1],&triangles[2]];
    // let mut z;
    // for y in bottomy..topy{
    //     for x in leftx..rightx{
    //         let bc_screen = barycentric(point_array, (x,y));
    //         if bc_screen.get(0) < 0. || bc_screen.get(1) < 0. || bc_screen.get(2) < 0.{
    //             continue;
    //         }
    //         z = 0.;
    //         z+=point_array[0].get(2)*bc_screen.get(0)+point_array[1].get(2)*bc_screen.get(1)+point_array[2].get(2)*bc_screen.get(2);
    //         if zbuffer[x+y*img.get_width()] < z{
    //             zbuffer[x+y*img.get_width()] = z;
    //             img.set(x, y, color);
    //         }
    //     }
    // }

}