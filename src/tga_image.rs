#![allow(non_camel_case_types)]
type uint8_t = u8;
type uint16_t = u16;
/// Synchronization with C's memory alignment
#[derive(Default)]
#[repr(C,packed)]
struct TGAHeader {
    idlength:uint8_t,
    colormaptype:uint8_t,
    datatypecode:uint8_t,
    colormaporigin:uint16_t,
    colormaplength:uint16_t,
    colormapdepth:uint8_t,
    x_origin:uint16_t,
    y_origin:uint16_t,
    width:uint16_t,
    height:uint16_t,
    bitsperpixel:uint8_t,
    imagedescriptor:uint8_t
}
#[derive(Default)]
pub struct TgaColor{
    rgba:[u8;4],
    size:usize,
}
impl TgaColor {
    pub fn new(red:u8,green:u8,blue:u8,alpha:u8)->Self{
        Self{rgba:[blue,green,red,alpha],size:4}
    }
    /// index should less than 4
    pub fn get(&self,index:usize)->u8{
        self.rgba[index]
    }
}
#[derive(Default)]
pub struct TgaImage{
    data:Vec<u8>,
    pixel_size:usize,
    width:usize,
    height:usize
}
impl TgaImage {
    pub const GRAYSCALE:usize = 1;
    pub const RGB:usize = 3;
    pub const RGBA:usize = 4;

    pub fn new(width:usize,height:usize,pixel_size:usize)->Self{
        Self { data: vec![0;width*height*pixel_size], pixel_size, width, height }
    }
    pub fn set(&mut self,x:usize,y:usize,color:&TgaColor){
        if !self.data.is_empty() && x<self.width && y <self.height{
            let position = (y*self.width+x)*self.pixel_size;
            self.data[position..(position+self.pixel_size)].copy_from_slice(&color.rgba[0..self.pixel_size]);
        }
    }
    pub fn get(&self,x:usize,y:usize)->Result<TgaColor,& 'static str>{
        if !self.data.is_empty() && x<self.width && y<self.height{
            let mut ret = TgaColor{rgba:[0;4],size:self.pixel_size};
            let position = (y*self.width+x)*self.pixel_size;
            ret.rgba[..ret.size].copy_from_slice(&self.data[position..(position+ret.size)]);
            Ok(ret)
        }else {
            Err("Index invalid!")
        }
    }
    /// Image flip vertical
    /// Y inside the picture goes from top to bottom
    /// This function change Y from bottom to top
    fn flip_vertically(&mut self){
        let half = self.height>>1;
        for x in 0..self.width{
            for y in 0..half{
                for align in 0..self.pixel_size{
                    self.data.swap((y*self.width+x)*self.pixel_size+align, ((self.height-1-y)*self.width+x)*self.pixel_size+align);
                }
            }
        }
    }
    /// unused,now is just write
    pub fn read_tga_file(&mut self,filename:&str){
        use std::fs::File;
        use std::io::Read;
        let mut f = File::open(filename).expect("Open file error!");
        let mut th = TGAHeader::default();
        assert_eq!(f.read(unsafe{serialize_raw_mut(&mut th)}).unwrap(),std::mem::size_of::<TGAHeader>());
        self.width = th.width as usize;
        self.height = th.height as usize;
        self.pixel_size = th.bitsperpixel as usize >> 3;
        if self.pixel_size != Self::GRAYSCALE || self.pixel_size != Self::RGB || self.pixel_size != Self::RGBA{
            panic!("Wrong format of tga!");
        }
        self.data = vec![0;self.height*self.width*self.pixel_size];
        if th.datatypecode == 3 || th.datatypecode == 2{
            assert_eq!(f.read(&mut self.data).unwrap(),self.data.len());
        }
        if th.imagedescriptor != 0x20{
            self.flip_vertically();
        }
    }
    /// Writes bytes to a file in the specified format
    pub fn write_tga_file(&mut self,filename:&'static str,flip:bool,rle:bool){
        use std::fs::File;
        use std::io::Write;
        
        let developer_area_ref:[uint8_t;4] = [0;4];
        let extension_area_ref:[uint8_t;4] = [0;4];
        let footer:[uint8_t;18] = [b'T',b'R',b'U',b'E',b'V',b'I',b'S',b'I',b'O',b'N',b'-',b'X',b'F',b'I',b'L',b'E',b'.',b'\0'];
        let img = File::create(filename);
        let mut img = img.expect("Create file failed!");
        let mut header = TGAHeader{bitsperpixel:(self.pixel_size as u8) << 3,height:self.height as uint16_t,width:self.width as uint16_t,..Default::default()};
        
        header.datatypecode = if Self::GRAYSCALE == self.pixel_size{
            if rle {11}else {3}
        }else if rle{10}
        else {2};
        header.imagedescriptor = if flip{0x00}else{0x20};
        
        assert!(img.write_all(unsafe{serialize_raw(&header)}).is_ok());
        assert!(img.write_all(&self.data).is_ok());
        assert!(img.write_all(&developer_area_ref).is_ok());
        assert!(img.write_all(&extension_area_ref).is_ok());
        assert!(img.write_all(&footer).is_ok());
    }
    pub fn draw_line(&mut self,mut point1:(usize,usize),mut point2:(usize,usize),color:&TgaColor){
        if point1.0 >= self.width || point1.1 >= self.height || point2.0 >= self.width || point2.1 >= self.height{
            println!("{:?} , {:?}",point1,point2);
            panic!("The coordinates are out of range");
        }
        let mut steep = false;
        if point1.0.abs_diff(point2.0) < point1.1.abs_diff(point2.1){
            std::mem::swap(&mut point1.0, &mut point1.1);
            std::mem::swap(&mut point2.0, &mut point2.1);
            steep = true;
        }
        if point1.0>point2.0{
            std::mem::swap(&mut point1.0, &mut point2.0);
            std::mem::swap(&mut point1.1, &mut point2.1);
        }
        let dx = point2.0 as i32 - point1.0 as i32;
        let dy = point2.1 as i32 - point1.1 as i32;
        let mut error = 0i32;
        let derror = dy.abs()*2;
        let mut y = point1.1;
        for x in point1.0..point2.0{
            if steep{
                self.set(y, x, color);
            }else {
                self.set(x, y, color);
            }
            error+=derror;
            if error > dx{
                if point2.1 > point1.1{
                    y+=1;
                }else {
                    y=y.saturating_sub(1);
                };
                error-=dx*2;
            }
        }
    }
    pub fn get_width(&self)->usize{
        self.width
    }
    pub fn get_height(&self)->usize{
        self.height
    }
}
/// Converts structs to byte arrays
unsafe fn serialize_raw<T:Sized>(src:&T) -> &[u8]{
    std::slice::from_raw_parts((src as *const T) as *const u8,std::mem::size_of::<T>())
}
/// Converts structs to mutable byte arrays
unsafe fn serialize_raw_mut<T:Sized>(src:&mut T)->&mut [u8]{
    std::slice::from_raw_parts_mut((src as * mut T) as * mut u8, std::mem::size_of::<T>())
}
#[allow(dead_code)]
fn create_a_file()->bool{
    use std::fs::File;
    let img = File::create("./target.tga");
    img.is_ok()
}
#[cfg(test)]
mod test{
    #[test]
    fn create_file_test(){
        use crate::tga_image::create_a_file;
        assert!(create_a_file());
    }
    #[test]
    fn serialize_test(){
        use crate::tga_image::serialize_raw;
        #[repr(C)]
        struct Temp{
            x:u8,
            y:u16
        }
        let temp = Temp{x:10,y:20};
        // align by 16 bits,will fill a byte with 0
        assert_eq!(unsafe {serialize_raw(&temp)},[10u8,0u8,20u8,0u8]);
    }
}