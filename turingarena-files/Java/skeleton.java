import java.util.Scanner;

abstract class Skeleton {
    private static final Scanner in = new Scanner(System.in);


    abstract int make_a_common_multiple_of_n_naturals(int n, int nats[]);

    abstract int make_a_natural_bigger_than_and_not_divisible_by_any_of(int n, int nats[]);

    abstract int make_a_new_prime_not_in_the_list(int n, int partial_list_of_primes[]);

    public static void main(String[] args) {
        Solution __solution = new Solution();

        // checkpoint
        System.out.printf("%d\n", 0);
        // read n
        int n;
        System.out.flush();
        n = in.nextInt();
        // for i to n {...}
        int[] nat;
        nat = new int[n];
        for (int i = 0; i < n; i++) {
            // read nat[i]
            System.out.flush();
            nat[i] = in.nextInt();
        }
        // call res1 = make_a_common_multiple_of_n_naturals(n, nat)
        int res1;
        res1 = __solution.make_a_common_multiple_of_n_naturals(n, nat);
        // write res1
        System.out.printf("%d\n", res1);
        // read n1
        int n1;
        System.out.flush();
        n1 = in.nextInt();
        // for i to n1 {...}
        int[] nat1;
        nat1 = new int[n1];
        for (int i = 0; i < n1; i++) {
            // read nat1[i]
            System.out.flush();
            nat1[i] = in.nextInt();
        }
        // call res2 = make_a_natural_bigger_than_and_not_divisible_by_any_of(n1, nat1)
        int res2;
        res2 = __solution.make_a_natural_bigger_than_and_not_divisible_by_any_of(n1, nat1);
        // write res2
        System.out.printf("%d\n", res2);
        // read n3
        int n3;
        System.out.flush();
        n3 = in.nextInt();
        // for i to n3 {...}
        int[] p;
        p = new int[n3];
        for (int i = 0; i < n3; i++) {
            // read p[i]
            System.out.flush();
            p[i] = in.nextInt();
        }
        // call res3 = make_a_new_prime_not_in_the_list(n3, p)
        int res3;
        res3 = __solution.make_a_new_prime_not_in_the_list(n3, p);
        // write res3
        System.out.printf("%d\n", res3);
        // exit
        System.exit(0);
    }
}
