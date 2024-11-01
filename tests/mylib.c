// mylib.c
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>


void hello_from_c() {
    printf("Proc in C triggered!\n");
}

int add(int a, int b) {
    return a + b + 5;
}



int proc()
{
    hello_from_c();

    return(0);
}