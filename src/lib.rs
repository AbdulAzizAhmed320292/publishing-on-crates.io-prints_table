use std::io;
 pub fn user_input() -> u32 {
   println!("enter the number for which you want to print table : ");
   
   let mut my_variable = String::new();
   
   io::stdin().read_line (& mut  my_variable);
   
   let  my_variable:u32 = my_variable.trim().parse().unwrap();
   
   my_variable
}





pub fn prints_mytable(){

  let my_variable = user_input();
   
  for i in 0..10{
        println!("{}  x  {}  =  {}",my_variable,i,(my_variable*i));
    }

}