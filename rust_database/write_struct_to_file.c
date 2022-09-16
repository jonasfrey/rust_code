
// C program for writing
// struct to file
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
 
// typedef enum
// {
//     button_left = 1,   // 0 0 0 1              
//     button_right = 2,  // 0 0 1 0
//     button_up = 4,     // 0 1 0 0 
//     button_down = 8,   // 1 0 0 0  
// } Button_pressed; 


// a struct to read and write
struct O_person
{
    int n_id;
    char s_name[20];
    char s_email[20];
    // char n_button_pressed;
};

  
   
int main ()
{
    f_write_to_file();
    // f_read_from_file();
}
int f_read_from_file ()
{
   FILE *infile;
    struct O_person input;
     
    // Open A_o_person.dat for reading
    infile = fopen ("A_o_person.dat", "r");
    if (infile == NULL)
    {
        fprintf(stderr, "\nError opening file\n");
        exit (1);
    }
     
    // read file contents till end of file
    while(fread(&input, sizeof(struct O_person), 1, infile))
        printf ("id = %d name = %s %s\n", input.n_id,
        input.s_name, input.s_email);
 
    // close file
    fclose (infile);
 
    return 0;
}

int f_write_to_file ()
{
    FILE *outfile;
     
    // open file for writing
    outfile = fopen ("A_o_person.dat", "w");
    if (outfile == NULL)
    {
        fprintf(stderr, "\nError opened file\n");
        exit (1);
    }


    int n_i = 0; 
    int n_max = 999999;
    
    while(n_i < n_max){
        struct O_person input1 = {n_i, "some body", "some@body.com"};
        struct O_person input2 = {n_i, "other body", "other@body.com"};
        
        // write struct to file
        fwrite (&input1, sizeof(struct O_person), 1, outfile);
        fwrite (&input2, sizeof(struct O_person), 1, outfile);

        n_i+=1;
    }


    if(fwrite != 0)
        printf("contents to file written successfully !\n");
    else
        printf("error writing file !\n");
 
    // close file
    fclose (outfile);
 
    return 0;
}

