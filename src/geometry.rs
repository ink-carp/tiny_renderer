#![allow(dead_code)]
#[derive(Clone)]
pub struct Array(Vec<f64>);
// trait implement only can change the rhs
// can not implement a function let vec * vec = f64

/// Vector mul 
impl std::ops::Mul for &Array {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = 0f64;
        for index in 0..self.0.len(){
            ret += self.0[index]*rhs.0[index];
        }
        ret
    }
}
impl std::ops::Mul<f64> for &Array {
    type Output = Array;
    fn mul(self, rhs:f64) -> Self::Output {
        let mut ret = Array::with_capacity(self.0.len());
        for index in 0..self.0.len(){
            ret.0.push(self.0[index]*rhs);
        }
        ret
    }
}
impl std::ops::Add<f64> for &Array {
    type Output = Array;
    fn add(self, rhs: f64) -> Self::Output {
        let mut ret = Array::with_capacity(self.0.len());
        for index in 0..self.0.len(){
            ret.0.push(self.0[index]+rhs);
        }
        ret
    }
}
impl std::ops::Sub for &Array {
    type Output = Array;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = Array::with_capacity(self.0.len());
        for index in 0..self.0.len(){
            ret.0.push(self.0[index]-rhs.0[index]);
        }
        ret
    }
}
impl std::ops::Div<f64> for &Array {
    type Output = Array;
    fn div(self, rhs: f64) -> Self::Output {
        let mut ret = Array::with_capacity(self.0.len());
        for index in 0..self.0.len(){
            ret.0.push(self.0[index]/rhs);
        }
        ret
    }
}
impl From<Vec<f64>> for Array {
    fn from(value:Vec<f64>) -> Self {
        Self(value)
    }
}
impl Array {
    pub fn new(len:usize)->Self{
        Self(vec![0f64;len])
    }
    pub fn with_capacity(size:usize)->Self{
        Self(Vec::<f64>::with_capacity(size))
    }
    pub fn embed(&self,rhs:&Self,fill:f64)->Self{
        let mut ret = Self::with_capacity(self.0.len());
        for i in 0..self.0.len(){
            if i < rhs.0.len(){
                ret.0.push(rhs.0[i]);
            }else {
                ret.0.push(fill);
            }
        }
        ret
    }
    /// The user should ensure that the length of the input is less than self length
    pub fn proj(&self,rhs:&Self)->Self{
        let mut ret = Array::with_capacity(self.0.len());
        for i in 0..self.0.len(){
            ret.0[i] = rhs.0[i];
        }
        ret
    }
    pub fn norm(&self)->f64{
        assert!(self.0.len() == 2 || self.0.len() == 3,"Array length not match!");
        (self * self).sqrt()
    }
    pub fn normalize(&self)->Self{
        assert!(self.0.len() == 2 || self.0.len() == 3,"Array length not match!");
        self/self.norm()
    }
    pub fn cross(&self,rhs:&Self)->Self{
        assert!(self.0.len() == 3,"Type not match!");
        Array::from(vec![self.get(1)*rhs.get(2) - self.get(2)*rhs.get(1),self.get(2)*rhs.get(0)-self.get(0)*rhs.get(2),self.get(0)*rhs.get(1)-self.get(1)*rhs.get(0)])
    }
    // expose to other crate to handle data

    /// Users should ensure that indexes are not out of range
    pub fn set(&mut self,x:usize,num:f64){
        self.0[x] = num;
    }
    /// Users should ensure that indexes are not out of range
    pub fn get(&self,x:usize)->f64{
        self.0[x]
    }
}

pub struct Matrix{
    cols:usize,
    rows:Vec<Array>,
}
impl Matrix {
    pub fn new(nrows:usize,ncols:usize)->Self{
        Self { cols: ncols, rows: vec![Array::new(ncols) ;nrows]}
    }
    pub fn identity(nrows:usize,ncols:usize)->Self{
        let mut ret = Self::new(nrows, ncols);
        let max_len = if ncols >= nrows{nrows}else{ncols};
        for index in 0..max_len{
            ret.rows[index].set(index, 1.);
        }
        ret
    }
    /// get one column of matrix
    pub fn get_col(&self,index:usize)->Array{
        assert!(index >= self.cols,"The maximum index length is exceeded!");
        let mut ret = Array::with_capacity(self.rows.len());
        for i in 0..self.rows.len(){
            ret.set(i, self.get(i, index));
        }
        ret
    } 
    pub fn set_col(&mut self,index:usize,arr:Array){
        assert!(index >= self.cols,"The maximum index length is exceeded!");
        for i in 0..self.rows.len(){
            self.set(i, index, arr.get(i));
        }
    }
    pub fn get(&self,y:usize,x:usize)->f64{
        self.rows[y].get(x)
    }
    pub fn set(&mut self,y:usize,x:usize,num:f64){
        self.rows[y].set(x, num);
    }
    /// Removes the specified row and column
    fn get_minor(&self,row:usize,col:usize)->Self{
        let mut ret = Self::new(self.rows.len()-1, self.cols);
        for y in 0..self.rows.len()-1{
            for x in 0..self.cols-1{
                let real_y = if y==row {y+1}else{y};
                let real_x = if x==col {x+1}else{x};
                ret.rows[y].set(x,self.get(real_y, real_x));
            }
        }
        ret
    }
    fn cofactor(&self,_row:usize,_col:usize)->f64{
        todo!()
    }

}

#[cfg(test)]
mod test{
    use crate::geometry::Array;

    #[test]
    fn test_mul(){
        let a1 = Array::from(vec![1.,2.,3.]);
        let rhs = Array::from(vec![3.,2.,1.]);
        let result = &a1 * &rhs;
        assert!(result.eq(&10.));
    }
}