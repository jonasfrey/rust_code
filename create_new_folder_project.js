var arguments = process.argv

var node_binary = arguments.shift()
var script_path_filename = arguments.shift()

if(arguments.length == 0){
    console.warn('no foldername specified, do it with:')
    console.info(`${node_binary.split('/').pop()} ${script_path_filename.split('/').pop()} the_folder_name_here`)
    process.exit(1)
}

var folder_name = arguments.shift()

var fs = require('fs');
var dir = './'+folder_name;

if (!fs.existsSync(dir)){
    
    fs.mkdirSync(dir);
   
}else{
    //console.log(`directory : ${folder_name} already exists`)
}


 
var makefile_content = `
${folder_name}: ${folder_name}.rs
\trustc ${folder_name}.rs

clean:
\trm -f ${folder_name}

`;
var c_file_content = `
// c file 
// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
`
var c_file_filename = dir+".rs"
var makefile_filename = "Makefile"
if (!fs.existsSync(dir+'/'+makefile_filename)){

    fs.writeFile(dir+'/'+makefile_filename, makefile_content, function(err) {
        if(err) {
            return console.log(err);
        }
        console.log("The "+dir+'/'+makefile_filename+" file was saved!");
    }); 

}
if (!fs.existsSync(dir+'/'+c_file_filename)){

    fs.writeFile(dir+'/'+c_file_filename, c_file_content, function(err) {
        if(err) {
            return console.log(err);
        }
        console.log("The "+dir+'/'+c_file_filename+" file was saved!");
    }); 

}

console.log('done')