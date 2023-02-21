# (C)hittyfier
Very simple rust program which takes in a (correctly) written C program and Shittyfies the source code.

# Example
~~~c
#include <stdio.h>

int main() {
    printf("Hello, World!\n");
    return 0;
}
~~~
turns into
~~~c
#define ___________________ return
#define _________________ "Hello, World!\n"
#define _____________ (
#define ___________ int
#define _______________ {
#define __________________ ;
#define _____________________ }
#define ________________ printf
#define ____________________ 0
#define ______________ )
#define ____________ main
#include <stdio.h>
___________
____________
_____________
______________
_______________
________________
_____________
_________________
______________
__________________
___________________
____________________
__________________
_____________________
~~~

# Usage
Cargo:
~~~
cargo run --release <filepath to a c file>
~~~
Exectuable:
~~~
./Chittyfier.exe <filepath to a c file>
~~~
After running the program a output.c file is created to the working directory.
