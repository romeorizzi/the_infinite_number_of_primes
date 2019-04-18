#include <stdio.h>
#include <stdlib.h>


int make_a_common_multiple_of_n_naturals(int n, int *nats);

int make_a_natural_bigger_than_and_not_divisible_by_any_of(int n, int *nats);

int make_a_new_prime_not_in_the_list(int n, int *partial_list_of_primes);


int main() {
    // checkpoint
    printf("%d\n", 0);
    // read n
    static int n;
    fflush(stdout);
    scanf("%d", &n);
    // for i to n {...}
    static int *nat;
    nat = malloc(n * sizeof(*nat));
    for (int i = 0; i < n; i++) {
        // read nat[i]
        fflush(stdout);
        scanf("%d", &nat[i]);
    }
    // call res1 = make_a_common_multiple_of_n_naturals(n, nat)
    static int res1;
    res1 = make_a_common_multiple_of_n_naturals(n, nat);
    // write res1
    printf("%d\n", res1);
    // read n1
    static int n1;
    fflush(stdout);
    scanf("%d", &n1);
    // for i to n1 {...}
    static int *nat1;
    nat1 = malloc(n1 * sizeof(*nat1));
    for (int i = 0; i < n1; i++) {
        // read nat1[i]
        fflush(stdout);
        scanf("%d", &nat1[i]);
    }
    // call res2 = make_a_natural_bigger_than_and_not_divisible_by_any_of(n1, nat1)
    static int res2;
    res2 = make_a_natural_bigger_than_and_not_divisible_by_any_of(n1, nat1);
    // write res2
    printf("%d\n", res2);
    // read n3
    static int n3;
    fflush(stdout);
    scanf("%d", &n3);
    // for i to n3 {...}
    static int *p;
    p = malloc(n3 * sizeof(*p));
    for (int i = 0; i < n3; i++) {
        // read p[i]
        fflush(stdout);
        scanf("%d", &p[i]);
    }
    // call res3 = make_a_new_prime_not_in_the_list(n3, p)
    static int res3;
    res3 = make_a_new_prime_not_in_the_list(n3, p);
    // write res3
    printf("%d\n", res3);
    // exit
    exit(0);
}
