use crate::geometry::Array;
#[derive(Default)]
pub struct Model{
    verts:Vec<Array>, // 顶点数组，Array<3>
    facet_vrt:Vec<i32>,
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
        let mut facet_vrt = Vec::<i32>::new();
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
                    }else if s.starts_with("f ") {
                        let mut it = s.split(' ');
                        it.next();
                        for _ in 0..3{
                            let temp = it.next().unwrap().split('/').collect::<Vec<&str>>();
                            facet_vrt.push(temp[0].parse::<i32>().expect("Not integer!")-1);
                        }
                        //assert!(it.next().is_some(),"Error: the obj file is supposed to be triangulated");
                    }
                },
                Err(_)=>{break;}
            }
        };
        println!("v:{}   f:{}",verts.len(),facet_vrt.len()/3);
        Model { verts,facet_vrt}
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