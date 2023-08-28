#[derive(Debug,Clone)]
pub struct Queue<T> {
    data: Vec<T>,
}
//VecStack
impl <T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: Vec::new() }
    }
    pub fn size(&self)->usize{
        self.data.len()
    }
    pub fn is_empty(&self)->bool{
        self.data.is_empty()
    }
    pub fn peek(&self)->Option<&T>{
        self.data.last()
    }
    pub fn enqueue(&mut self,t:T){
        self.data.push(t)
    }
    pub fn dequeue(&mut self)-> Result<T, &str>{
        if self.is_empty() {
            Err("empty")
        }else {
            Ok(self.data.remove(0))
        }
    }
}
