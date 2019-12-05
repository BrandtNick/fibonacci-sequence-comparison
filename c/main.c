#include<stdio.h> 
#include <time.h>
 
 unsigned int fibonacci(unsigned int n){
         if (n <= 1) {
                 return 1;
         }
         return fibonacci(n - 1) + fibonacci(n - 2);
 }
  
 void main(){ 
         clock_t t;
         t = clock();
         fibonacci(50);
         t = clock() - t;
         double time_taken = ((double)t)/CLOCKS_PER_SEC;
         printf("%.2f s\n", time_taken);
 }  
  
