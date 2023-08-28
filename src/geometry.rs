#![allow(dead_code)]
use std::ops::*;
#[derive(Clone,Debug)]
pub struct Array<T>(Vec<T>);

/// Vector mul 
impl<T> std::ops::Mul for &Array<T> where T:Mul<Output = T>+AddAssign+Default+Copy{
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = T::default();
        for index in 0..self.0.len(){
            ret += self.0[index]*rhs.0[index];
        }
        ret
    }
}
impl<T> std::ops::Add for &Array<T> where T:Add<Output = T>+Clone+AddAssign+Default+Mul<Output = T>+Div<Output = T>+Sub<Output = T>+Copy{
    type Output = Array<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = Array::<T>::with_capacity(self.0.len());
        for index in 0..self.0.len(){
            ret.0.push(self.0[index]+rhs.0[index]);
        }
        ret
    }
}
impl<T> std::ops::Add<T> for &Array<T> where T:Add<Output = T> + Clone+Default+Mul<Output = T>+AddAssign+Div<Output = T>+Sub<Output = T>+Copy{
    type Output = Array<T>;
    fn add(self, rhs: T) -> Self::Output {
        let mut ret = Array::<T>::with_capacity(self.0.len());
        for index in 0..self.0.len(){
            ret.0.push(self.0[index]+rhs);
        }
        ret
    }
}
impl<T> std::ops::Sub for &Array<T> where T:Sub<Output = T>+Clone+Default+Mul<Output = T>+AddAssign+Div<Output = T>+Copy{
    type Output = Array<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = Array::<T>::with_capacity(self.0.len());
        for index in 0..self.0.len(){
            ret.0.push(self.0[index]-rhs.0[index]);
        }
        ret
    }
}
impl<T> std::ops::Div<T> for &Array<T> where T:Div<Output = T>+Clone+Mul<Output = T>+Default+AddAssign+Sub<Output = T>+Copy{
    type Output = Array<T>;
    fn div(self, rhs:T) -> Self::Output {
        let mut ret = Array::<T>::with_capacity(self.0.len());
        for index in 0..self.0.len(){
            ret.0.push(self.0[index]/rhs);
        }
        ret
    }
}
impl<T> From<Vec<T>> for Array<T> {
    fn from(value:Vec<T>) -> Self {
        Self(value)
    }
}
impl<T> Array<T> where T:Default+Clone+Mul<Output = T>+AddAssign+Div<Output = T>+Sub<Output = T>+Copy{
    pub fn new(len:usize)->Self{
        Self(vec![T::default();len])
    }
    pub fn with_capacity(size:usize)->Self{
        Self(Vec::<T>::with_capacity(size))
    }
    pub fn embed(&self,rhs:&Self,fill:T)->Self{
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
        let mut ret = Self::with_capacity(self.0.len());
        for i in 0..self.0.len(){
            ret.0[i] = rhs.0[i];
        }
        ret
    }
    pub fn cross(&self,rhs:&Self)->Self{
        assert!(self.0.len() == 3,"Type not match!");
        Array::from(vec![self.get(1)*rhs.get(2) - self.get(2)*rhs.get(1),self.get(2)*rhs.get(0)-self.get(0)*rhs.get(2),self.get(0)*rhs.get(1)-self.get(1)*rhs.get(0)])
    }
    // expose to other crate to handle data

    /// Users should ensure that indexes are not out of range
    pub fn set(&mut self,x:usize,num:T){
        self.0[x] = num;
    }
    /// Users should ensure that indexes are not out of range
    pub fn get(&self,x:usize)->T{
        self.0[x]
    }
}
impl Array<f32> {
    pub fn norm(&self)->f32{
        assert!(self.0.len() == 2 || self.0.len() == 3,"Array length not match!");
        (self * self).sqrt()
    }
    pub fn normalize(&self)->Self{
        assert!(self.0.len() == 2 || self.0.len() == 3,"Array length not match!");
        self/(self.norm())
    }
    /// 向量转为齐次坐标
    pub fn to_matrix(src:&Array<f32>)->Matrix<f32>{
        let mut ret = Matrix::new(4,1);
        ret.set(0, 0, src.get(0));
        ret.set(1, 0, src.get(1));
        ret.set(2, 0, src.get(2));
        ret.set(3, 0, 1.);
        ret
    }
}
#[derive(Debug)]
pub struct Matrix<T> where T:Default+Clone+Mul<Output = T>+AddAssign+Div<Output = T>+Sub<Output = T>+Copy{
    cols:usize,
    rows:Vec<Array<T>>,
}
impl<T> std::ops::Mul for &Matrix<T> where T:Default+Clone+Mul<Output = T>+AddAssign+Div<Output = T>+Sub<Output = T>+Add<Output = T>+Copy{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.cols == rhs.rows.len(),"Matrix multiply should cols = rows!");
        let mut ret = Matrix::new(self.rows.len(),rhs.cols);
        for i in 0..self.rows.len(){
            for j in 0..rhs.cols{
                ret.set(i, j, T::default());
                for k in 0..self.cols{
                    ret.set(i, j, ret.get(i, j)+self.get(i, k)*rhs.get(k, j));
                }
            }
        }
        ret
    }
}
impl<T> Matrix<T> where T:Default+Clone+Mul<Output = T>+AddAssign+Div<Output = T>+Sub<Output = T>+Copy{
    pub fn new(nrows:usize,ncols:usize)->Self{
        Self { cols: ncols, rows: vec![Array::new(ncols) ;nrows]}
    }
    pub fn identity(nrows:usize,ncols:usize,fill:T)->Self{
        let mut ret = Self::new(nrows, ncols);
        for index in 0..ncols{
            ret.rows[index].set(index, fill);
        }
        ret
    }
    /// get one column of matrix
    pub fn get_col(&self,index:usize)->Array<T>{
        assert!(index >= self.cols,"The maximum index length is exceeded!");
        let mut ret = Array::with_capacity(self.rows.len());
        for i in 0..self.rows.len(){
            ret.set(i, self.get(i, index));
        }
        ret
    } 
    pub fn set_col(&mut self,index:usize,arr:Array<T>){
        assert!(index >= self.cols,"The maximum index length is exceeded!");
        for i in 0..self.rows.len(){
            self.set(i, index, arr.get(i));
        }
    }
    pub fn get(&self,y:usize,x:usize)->T{
        self.rows[y].get(x)
    }
    pub fn set(&mut self,y:usize,x:usize,num:T){
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
    pub fn to_array(src:Self)->Array<T>{
        Array::from(vec![src.get(0, 0)/src.get(3, 0),src.get(1, 0)/src.get(3, 0),src.get(2, 0)/src.get(3, 0)])
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