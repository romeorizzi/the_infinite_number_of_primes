import os as _os

def main(_solution):
    # checkpoint
    print(0)
    # read n
    print(end="", flush=True)
    [n] = map(int, input().split())
    # for i to n {...}
    nat = [None] * n
    for i in range(n):
        # read nat[i]
        print(end="", flush=True)
        [nat[i]] = map(int, input().split())
    # call res1 = make_a_common_multiple_of_n_naturals(n, nat)
    res1 = int(_solution.make_a_common_multiple_of_n_naturals(n, nat))
    # write res1
    print(res1)
    # read n1
    print(end="", flush=True)
    [n1] = map(int, input().split())
    # for i to n1 {...}
    nat1 = [None] * n1
    for i in range(n1):
        # read nat1[i]
        print(end="", flush=True)
        [nat1[i]] = map(int, input().split())
    # call res2 = make_a_natural_bigger_than_and_not_divisible_by_any_of(n1, nat1)
    res2 = int(_solution.make_a_natural_bigger_than_and_not_divisible_by_any_of(n1, nat1))
    # write res2
    print(res2)
    # read n3
    print(end="", flush=True)
    [n3] = map(int, input().split())
    # for i to n3 {...}
    p = [None] * n3
    for i in range(n3):
        # read p[i]
        print(end="", flush=True)
        [p[i]] = map(int, input().split())
    # call res3 = make_a_new_prime_not_in_the_list(n3, p)
    res3 = int(_solution.make_a_new_prime_not_in_the_list(n3, p))
    # write res3
    print(res3)
    # exit
    print(end="", flush=True)
    _os._exit(0)


if __name__ == '__main__':
    import sys
    import runpy
    
    if len(sys.argv) != 2:
        print("Usage: {} <solution>".format(sys.argv[0]))
    
    class Wrapper: pass 
    solution = Wrapper()
    solution.__dict__ = runpy.run_path(sys.argv[1])
    main(solution)

