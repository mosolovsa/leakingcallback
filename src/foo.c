#include <stdio.h>

void Fl_awake_callback(void (*callback)(void *), void *data) {
    printf("Callback address: %p, data address: %p\n", callback, data);
    (*callback)(data);
}