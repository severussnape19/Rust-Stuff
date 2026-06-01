#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#define BUF_SIZE 1024

[[noreturn]] void errExit(const char *msg) {
    fprintf(stderr, "Fatal Error: %s %s", msg, (errno != 0) ? strerror(errno): "");
    exit(EXIT_FAILURE);
}

void write_from(int input_fd, int output_fd) {
    char buffer[BUF_SIZE];
    ssize_t num_read;
    while ((num_read = read(input_fd, buffer, BUF_SIZE)) > 0) {
        ssize_t num_written = 0;
        while (num_read > num_written) {
            ssize_t n = write(output_fd, buffer + num_written, num_read - num_written);
            if (n == -1) {
                errExit("write");
            }
            num_written += n;
        }
        if (num_read == -1) {
            errExit("read");
        }
    }
}

int main(int argc, char *argv[]) {
    //if (argc < 3 || strcmp(argv[2], "--help") || strcmp(argv[1], "cat")) {
    if (argc < 3) {
        fprintf(stderr, "Usage: %s <input-file> <output-file>", argv[0]);
        exit(EXIT_FAILURE);
    }

    int open_flags = O_CREAT | O_TRUNC | O_RDWR;
    mode_t mode_flags = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IXUSR;
    int input_fd = open(argv[1], open_flags, mode_flags);
    if (input_fd < 0)
        errExit("open");

    int output_fd = open(argv[2], open_flags, mode_flags);
    if (output_fd < 0)
        errExit("open");

    write_from(STDIN_FILENO, input_fd);

    if (lseek(input_fd, 0, SEEK_SET) == -1)
        errExit("lseek");

    write_from(input_fd, output_fd);

    if (lseek(output_fd, 0, SEEK_SET) == -1)
        errExit("lseek");

    write_from(output_fd, STDOUT_FILENO);
}
