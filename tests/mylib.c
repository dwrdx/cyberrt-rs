// mylib.c
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>


void hello_from_c() {
    printf("Hello from C library updated!\n");
}

int add(int a, int b) {
    return a + b + 5;
}


void *thread_funct(void *a)
{
    const int loop=8;
    int x;

    for( x=0; x<loop; x++ )
    {
        printf("*\n");
        sleep(1);
    }

    return(NULL);
}

int proc()
{
    char buffer[BUFSIZ];
    int r;
    pthread_t thd;

    /* spawn the new thread */
    r = pthread_create( &thd, NULL, thread_funct, NULL);
    if( r!=0 )
    {
        perror("Thread");
        exit(1);
    }

    /* prompt for your name */
    printf("What is your name? ");
    fgets(buffer,BUFSIZ,stdin);
    printf("Hello, %s",buffer);

    return(0);
}