#include <fcntl.h>
#include <get_num.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <errno.h>
#include <ctype.h>
#include <string.h>

[[noreturn]] void errExit(const char *msg) {
    int saved_errno = errno;
    fprintf(stderr, "%s %s\n", msg, (saved_errno != 0) ? strerror(saved_errno) : "");
    exit(EXIT_FAILURE);
}

int main(int argc, char *argv[])  {
    if (argc < 3 || strcmp(argv[1], "--help") == 0) {
        fprintf(stderr, "Usage Error: %s {r<length>|R<length>|w<string>|s<offset>}...\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    int open_flags = O_RDWR | O_CREAT;
    mode_t mode_flags = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
    int fd = open(argv[1], open_flags, mode_flags);
    if (fd < 0) {
        errExit("open");
    }

    for (int ap = 2; ap < argc; ap++) {
        switch (argv[ap][0]) {
            case 'r': case 'R':  {
                size_t len = getLong(&argv[ap][1], GN_ANY_BASE, argv[ap]);
                unsigned char *buf = malloc(len);
                if (buf == NULL) {
                   errExit("malloc");
                }

                ssize_t num_read = read(fd, buf, len);
                if (num_read < 0) {
                    errExit("read");
                }

                if (num_read == 0) {
                    printf("%s: EOF\n", argv[ap]);
                } else {
                    printf("%s: ", argv[ap]);
                    for (int i = 0; i < num_read; i++) {
                        if (argv[ap][0] == 'r') {
                            printf("%c", isprint(buf[i]) ? buf[i] : '?');
                        } else {
                            printf("%02x ", buf[i]);
                        }
                    }
                }

                free(buf);
                break;
            };

            case 'w': { /* write stirng at current offset */
                ssize_t num_written = write(fd, &argv[ap][1], strlen(&argv[ap][1]));
                if (num_written < 0) {
                    errExit("write");
                }
                printf("%s: wrote %ld bytes\n", argv[ap], (long) num_written);
                break;
            };

            case 's': {
                size_t offset = getLong(&argv[ap][1], GN_ANY_BASE, argv[ap]);
                if (lseek(fd, offset, SEEK_SET) == -1) {
                    errExit("lseek");
                }
                printf("Seeked successfully!");
                break;
            };

            case 'S': {
                off_t offset = lseek(fd, 0, SEEK_CUR);
                if  (offset != -1) {
                    printf("Current position: %ld\n", offset);
                } else {
                    errExit("lseek");
                }
                break;
            }
        }
    }
    exit(EXIT_SUCCESS);
}
