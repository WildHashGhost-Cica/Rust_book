fn main(){
    
        let x = 5;
        let y = x;                       //copy not move 

        let s1 = String::from("hello");
        println!(" x is = {}", x);
        println!(" x is = {}", y);
        println!(" s1 = {}", s1);
        let s2 = s1.clone();             //clone 
        println!(" s2 = {}", s2);

        
        
    
    
   
}