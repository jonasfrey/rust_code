
// c file 
// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->
#include <stdio.h>

char bitmap[1000];

void BMPmake()
{
    // -- FILE HEADER -- //

    // bitmap signature
    bitmap[0] = 'B';
    bitmap[1] = 'M';

    // file size
    bitmap[2] = 66; // 40 + 14 + 12
    bitmap[3] = 0;
    bitmap[4] = 0;
    bitmap[5] = 0;

    // reserved field (in hex. 00 00 00 00)
    for(int i = 6; i < 10; i++) bitmap[i] = 0;

    // offset of pixel data inside the image
    for(int i = 10; i < 14; i++) bitmap[i] = 0;

    // -- BITMAP HEADER -- //

    // header size
    bitmap[14] = 40;
    for(int i = 15; i < 18; i++) bitmap[i] = 0;

    // width of the image
    bitmap[18] = 4;
    for(int i = 19; i < 22; i++) bitmap[i] = 0;

    // height of the image
    bitmap[22] = 1;
    for(int i = 23; i < 26; i++) bitmap[i] = 0;

    // reserved field
    bitmap[26] = 1;
    bitmap[27] = 0;

    // number of bits per pixel
    bitmap[28] = 24; // 3 byte
    bitmap[29] = 0;

    // compression method (no compression here)
    for(int i = 30; i < 34; i++) bitmap[i] = 0;

    // size of pixel data
    bitmap[34] = 12; // 12 bytes => 4 pixels
    bitmap[35] = 0;
    bitmap[36] = 0;
    bitmap[37] = 0;

    // horizontal resolution of the image - pixels per meter (2835)
    bitmap[38] = 0;
    bitmap[39] = 0;
    bitmap[40] = 0b00110000;
    bitmap[41] = 0b10110001;

    // vertical resolution of the image - pixels per meter (2835)
    bitmap[42] = 0;
    bitmap[43] = 0;
    bitmap[44] = 0b00110000;
    bitmap[45] = 0b10110001;

    // color pallette information
    for(int i = 46; i < 50; i++) bitmap[i] = 0;

    // number of important colors
    for(int i = 50; i < 54; i++) bitmap[i] = 0;

    // -- PIXEL DATA -- //
    for(int i = 54; i < 66; i++) bitmap[i] = 33;
}

void BMPwrite()
{
    FILE *file;
    file = fopen("bitmap.bmp", "w+");
    for(int i = 0; i < 66; i++)
    {
        fputc(bitmap[i], file);
    }
    fclose(file);
}

int main(int argc, char *argv[]) {
    BMPmake();
    BMPwrite();
    printf("done");

}