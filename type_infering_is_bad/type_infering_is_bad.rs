
// c file 
// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

fn main(){
    f_maybe();
}

fn f_bad() {
    // Statements here are executed when the compiled binary is called


    let n1 = 8u8; 
    
    let n2 = 8u16; 
    
    let mut a = Vec::new(); 
    
    // so if we cant push different types u8 and u16 to an array 
    // what the fuck is 'typeless' instantiation (let mut a = Vec::new();) of that array for ????
    a.push(n1);
    a.push(n2);
    
    println!("{:?}", a); 
    
}

fn f_maybe(){
    let n1 = 123; 
    let mut a = Vec::new();
    a.push(n1); // is the type of the array now u8 ? 

    let n2 = 1234123412; 
    a.push(n2); 

}

fn f_good(){
        
    let n1 = 82u8; 
    
    let mut a: Vec<u8> = Vec::new(); 
    // now the programmer sees on one line what the type of the array should be 
    
    a.push(n1);
    
    println!("{:?}", a);
}

fn f_stupid(){

    let n1 = 123u8; 
    
    let mut a = Vec::new(); 
    
    // since we push a u8 integer, the compiler now knows that the array/vector must hold u8 integer
    // but this is actually pretty misleading , since a developer must now take a look at multiple lines of code to see what datatype the array holds
    a.push(n1);
    
    println!("{:?}", a);

}


