#include <stdio.h>

int main(){
    int x = 1;
    int* p = &x; // pにはxの「アドレス」が入る。xの値そのものではない。
    int* q = p;
    
    printf("x: %d\n", x); /* x : 2 */
    printf("p-1: %d\n", *p); /* p-1 : 1 */
    
    *q = 2; // qの値を変更(qはpのポインタ)

    printf("p-2: %d\n", *p); /* q-2 : 2 */

    printf("x: %d\n", x); /* x : 2 */

    return 0;
}