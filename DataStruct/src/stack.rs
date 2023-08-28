#[derive(Debug,Clone)]
pub struct Stack<T> {
    data: Vec<T>,
}
//Stack
impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack {data: Vec::new()}
    }
    pub fn size(&self)->usize{
        self.data.len()
    }
    pub fn is_empty(&self)->bool{
        self.data.is_empty()
    }
    pub fn push(&mut self,t:T){
        self.data.push(t)
    }
    pub fn pop(&mut self)->Option<T>{
        self.data.pop()
    }
    pub fn peek(&self)->Option<&T>{
        self.data.last()
    }
}



