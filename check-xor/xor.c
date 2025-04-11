#include <stdio.h>
#include <stdlib.h>

int main() {
    FILE *file;
    unsigned char checksum = 0;
    int byte;

    // Open the input file for reading in binary mode
    file = fopen("input.txt", "rb");
    
    // Check if file was opened successfully
    if (file == NULL) {
        printf("Error opening file input.txt\n");
        return 1;
    }
    
    // Read each byte and XOR with the current checksum
    while ((byte = fgetc(file)) != EOF) {
        checksum ^= (unsigned char)byte;
    }
    
    // Close the file
    fclose(file);
    
    // Print the checksum in hexadecimal format
    printf("XOR Checksum: 0x%02X\n", checksum);
    
    return 0;
}