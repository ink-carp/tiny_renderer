#![allow(dead_code)]
use crate::{geometry::Array, tga_image::{TgaImage, TgaColor}};
use crate::FT;
#[derive(Default)]
pub struct Model{
    verts:Vec<Array<FT>>, // 顶点数组，Array<3>
    tex_coord:Vec<Array<FT>>, //每个顶点的 tex 坐标数组 , Array<2>
    norms:Vec<Array<FT>>, //法线向量的每个顶点数组 ,Array<3>,
    facet_vrt:Vec<i32>,
    facet_tex:Vec<i32>, // 上述数组中的每三角形索引Sub
    facet_nrm:Vec<i32>,
    diffusemap:TgaImage, // 漫反射颜色纹理
    normalmap:TgaImage, // 法线贴图纹理
    specularmap:TgaImage, // 镜面反射贴图纹理
}
impl Model {
    pub fn new(filename:&'static str)->Model{
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;
        let f = File::open(filename).unwrap();
        let br = BufReader::new(f);

        //-----------------------------------------------------
        let mut verts = Vec::<Array::<FT>>::new();
        let mut norms = Vec::<Array::<FT>>::new();
        let mut tex_coord = Vec::<Array::<FT>>::new();
        let mut facet_vrt = Vec::<i32>::new();
        let mut facet_tex = Vec::<i32>::new();
        let mut facet_nrm = Vec::<i32>::new();
        let mut diffusemap = TgaImage::default();
        let mut normalmap = TgaImage::default();
        let specularmap = TgaImage::default();
        //-----------------------------------------------------

        println!("Reading Obj file {} ...",filename);
        for line in br.lines(){
            match line {
                Ok(s) =>{
                    if s.starts_with("v "){
                        let mut it = s.split(' ');
                        let mut v = Array::new(3);
                        it.next();
                        for i in 0..3{
                            v.set(i, it.next().unwrap().parse::<FT>().expect("Not double!"));
                        }
                        verts.push(v);
                    }else if s.starts_with("vn ") {
                        let mut it = s.split(' ');
                        let mut n = Array::new(3);
                        it.next();
                        let mut temp = it.next().unwrap();
                        while temp.eq(" ") {
                            temp = it.next().unwrap();
                        }
                        for i in 0..3{
                            n.set(i, it.next().unwrap().parse::<FT>().expect("Not double!"));
                        }
                        norms.push(n);
                    }else if s.starts_with("vt ") {
                        let mut it = s.split(' ');
                        let mut uv = Array::new(2);
                        it.next();
                        it.next();
                        for i in 0..2{
                            let temp = it.next().unwrap();
                            // while temp.eq(" ") {
                            //     temp = it.next().unwrap();
                            // }
                            uv.set(i, temp.parse::<FT>().expect("Not double!"));
                        }
                        tex_coord.push(uv);
                    }else if s.starts_with("f ") {
                        let mut it = s.split(' ');
                        it.next();
                        for _ in 0..3{
                            let temp = it.next().unwrap().split('/').collect::<Vec<&str>>();
                            facet_vrt.push(temp[0].parse::<i32>().expect("Not integer!")-1);
                            facet_tex.push(temp[1].parse::<i32>().expect("Not integer!")-1);
                            facet_nrm.push(temp[2].parse::<i32>().expect("Not integer!")-1);
                        }
                        //assert!(it.next().is_some(),"Error: the obj file is supposed to be triangulated");
                    }
                },
                Err(_)=>{break;}
            }
        };
        println!("Read from obj file Ok!\nv:{}  f:{}  vt:{}  vn:{}",verts.len(),facet_vrt.len()/3,tex_coord.len(),norms.len());
        Self::load_texture(filename, "_diffuse.tga",&mut diffusemap);
        Self::load_texture(filename, "_nm_tangent.tga", &mut normalmap);
        // Self::load_texture(filename, "_spec.tga", &mut specularmap);
        Model { verts, tex_coord, norms, facet_vrt, facet_tex, facet_nrm, diffusemap, normalmap, specularmap }
    }
    fn load_texture(filename:&'static str,suffix:&'static str,img:&mut TgaImage){
        let dot = filename.rfind('.').expect("No this character!");
        let texfile = format!("{}{}",&filename[0..dot],suffix);
        img.read_tga_file(&texfile);
        println!("texture file {} load ok.",texfile);
    }
    /// return Array<3>
    pub fn normal(&self,uvf:&Array<FT>)->Array<FT>{
        let c = self.normalmap.get((uvf.get(0) * (self.normalmap.get_width() as FT)) as usize, (uvf.get(1)*(self.normalmap.get_height() as FT)) as usize);
        let mut ret = Array::new(3);
        for i in 0..3{
            ret.set(i, c.get(2-i) as FT * 2. / 255. -1.);
        }
        ret
    }
    /// return Array<2>
    /// get the texture coords
    pub fn uv(&self,iface:usize,nthvert:usize)->Array<FT>{
        self.tex_coord[self.facet_tex[iface*3+nthvert] as usize].clone()
    }
    /// return Array<3>
    pub fn get_norm(&self,iface:usize,nthvert:usize)->Array<FT>{
        self.norms[self.facet_nrm[iface*3+nthvert] as usize].clone()
    }
    pub fn nfaces(&self)->usize{
        self.facet_vrt.len()/3
    }
    pub fn nverts(&self)->usize{
        self.verts.len()
    }
    pub fn get_vert(&self,index:usize)->[Array<FT>;3]{
        let ret1 = self.verts[self.facet_vrt[index*3] as usize].clone();
        let ret2 = self.verts[self.facet_vrt[index*3+1] as usize].clone();
        let ret3 = self.verts[self.facet_vrt[index*3+2] as usize].clone();
        [ret1,ret2,ret3]
    }
    /// get the diffusemap color
    pub fn get_diffuse(&self,x:usize,y:usize)->TgaColor{
        self.diffusemap.get(x, y)
    }
    /// get texture coords
    pub fn get_texture_coords(&self,index:usize)->[[i32;2];3]{
        let w = self.diffusemap.get_width();
        let h = self.diffusemap.get_height();
        let mut ret = [[0;2];3];
        for (i,element) in ret.iter_mut().enumerate(){
            let coord = &self.tex_coord[self.facet_tex[index*3+i] as usize];
            element[0] = (coord.get(0) * (w as FT)) as i32;
            element[1] = (coord.get(1) * (h as FT)) as i32;
        }
        ret
    }
}