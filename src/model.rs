use crate::geometry::Array;
#[derive(Default)]
pub struct Model{
    verts:Vec<Array>, // 顶点数组，Array<3>
    tex_coord:Vec<Array>, //每个顶点的 tex 坐标数组 , Array<2>
    norms:Vec<Array>, //法线向量的每个顶点数组 ,Array<3>,
    facet_vrt:Vec<i32>,
    facet_tex:Vec<i32>, // 上述数组中的每三角形索引
    facet_nrm:Vec<i32>,
}
impl Model {
    pub fn new(filename:&'static str)->Model{
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;
        let f = File::open(filename).unwrap();
        let br = BufReader::new(f);

        //-----------------------------------------------------
        let mut verts = Vec::<Array>::new();
        let mut norms = Vec::<Array>::new();
        let mut tex_coord = Vec::<Array>::new();
        let mut facet_vrt = Vec::<i32>::new();
        let mut facet_tex = Vec::<i32>::new();
        let mut facet_nrm = Vec::<i32>::new();
        //-----------------------------------------------------


        for line in br.lines(){
            match line {
                Ok(s) =>{
                    if s.starts_with("v "){
                        let mut it = s.split(' ');
                        let mut v = Array::new(3);
                        it.next();
                        for i in 0..3{
                            v.set(i, it.next().unwrap().parse::<f64>().expect("Not double!"));
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
                            n.set(i, it.next().unwrap().parse::<f64>().expect("Not double!"));
                        }
                        norms.push(n);
                    }else if s.starts_with("vt ") {
                        let mut it = s.split(' ');
                        let mut uv = Array::new(2);
                        it.next();
                        for i in 0..2{
                            let mut temp = it.next().unwrap();
                            while temp.eq(" ") {
                                temp = it.next().unwrap();
                            }
                            uv.set(i, it.next().unwrap().parse::<f64>().expect("Not double!"));
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
        println!("v:{}  f:{}  vt:{}  vn:  {}",verts.len(),facet_vrt.len()/3,tex_coord.len(),norms.len());
        Model { verts, tex_coord, norms, facet_vrt, facet_tex, facet_nrm}
    }
    /// return Array<2>
    pub fn uv(&self,iface:usize,nthvert:usize)->Array{
        self.tex_coord[self.facet_tex[iface*3+nthvert] as usize].clone()
    }
    /// return Array<3>
    pub fn get_norm(&self,iface:usize,nthvert:usize)->Array{
        self.norms[self.facet_nrm[iface*3+nthvert] as usize].clone()
    }
    pub fn nfaces(&self)->usize{
        self.facet_vrt.len()/3
    }
    pub fn nverts(&self)->usize{
        self.verts.len()
    }
    pub fn get_face(&self,index:usize)->[i32;3]{
        [self.facet_vrt[index*3],self.facet_vrt[index*3+1],self.facet_vrt[index*3+2]]
    }
    pub fn get_vert(&self,index:usize)->Array{
        self.verts[index].clone()
    }
}