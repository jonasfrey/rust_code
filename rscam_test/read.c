#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>    
int main() {
    char byte;
    int fd = open("/dev/video1", O_RDWR);
    // write(fd, "X", 1);
    ssize_t size = read(fd, &byte, 1);
    printf("Read byte %c\n", byte);
    int n_max = 10;
    int n_i = 0; 
    while(n_i < n_max){
        sleep(0.50);
        f_read_bytes_in_loop();
        n_i+=1;
    }
    f_read_bytes_in_loop();
    f_read_bytes_in_loop();
    f_read_bytes_in_loop();
    f_read_bytes_in_loop();

    // f_read_bytes_in_loop();
    return 0;
}


void f_read_bytes_in_loop() {
    int n_max = 1;
    int n_i = 0; 
    char byte;
    while(n_i < n_max){
        int fd = open("/dev/video1", O_RDWR);
        ssize_t size = read(fd, &byte, 1);
        // printf("%c\n", byte);
        printf("%x", byte);
        n_i+=1;
        // close(fd);
    }
}