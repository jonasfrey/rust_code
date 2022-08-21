

#include <stdio.h>
#include <time.h>
#include <stdint.h>

struct timeval {
   time_t      tv_sec;   // Number of whole seconds of elapsed time
   long int    tv_usec;  // Number of microseconds of rest of elapsed time minus tv_sec. Always less than one million
};
struct input_event {
      struct timeval time;
      unsigned short type;
      unsigned short code;
      unsigned int value; 
};


void f_simple_read_file(){
    FILE *fp;
    //                        try out every number 0-50  !!!!
    char * s_path_name_file_name = "/dev/input/event18";


    // r: Opens an existing text file for reading purpose.
    // w: Opens a text file for writing. If it does not exist, then a new file is created. Here your program will start writing content from the beginning of the file.
    // a: Opens a text file for writing in appending mode. If it does not exist, then a new file is created. Here your program will start appending content in the existing file content.
    // r+: Opens a text file for both reading and writing.
    // w+: Opens a text file for both reading and writing. It first truncates the file to zero length if it exists, otherwise creates a file if it does not exist.
    // a+: Opens a text file for both reading and writing. It creates the file if it does not exist. The reading will start from the beginning but writing can only be appended.

   fp = fopen(s_path_name_file_name, "r");
//    fprintf(fp, "This is testing for fprintf...\n");
//    fputs("This is testing for fputs...\n", fp);
    char s_char = fgetc(fp);
    printf("char is %c", s_char);
   fclose(fp);
}
void f_print_devices(){
    int n_enought = 100000;
    char s_buffer[n_enought];
    char * s_path_name_file_name = "/proc/bus/input/devices";
    
    FILE *file_pointer;
    file_pointer = fopen(s_path_name_file_name, "r");

    int n_sizet = fread(
        &s_buffer,
        1, 
        n_enought, 
        file_pointer);
    printf("s_buffer %s", s_buffer);

}

void f_print_dev_input_event(){
    int n_enought = sizeof(struct input_event);
    char s_buffer[n_enought];
    char * s_path_name_file_name = "/dev/input/event18";
    
    FILE *file_pointer;
    file_pointer = fopen(s_path_name_file_name, "r");
    struct input_event o_input_event;

    int n_sizet = fread(
        &o_input_event,
        1, 
        n_enought, 
        file_pointer);
    printf("---------------------------\n");
    printf("o_input_event.time.tv_sec %li\n", o_input_event.time.tv_sec);
    printf("o_input_event.time.tv_usec %li\n", o_input_event.time.tv_usec);
    printf("o_input_event.type %i\n", o_input_event.type);
    printf("o_input_event.code %i\n", o_input_event.code);
    printf("o_input_event.value %i\n", o_input_event.value);

}
void main(){

    printf("hello\n");
    
    int n_i = 0; 
    while(n_i < 1000){
        f_print_dev_input_event();
    }

    // f_print_devices();
    // f_simple_read_file();
}

//gcc main.c -o main