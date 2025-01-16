unsigned long long int fibonacci(int n) {
    if (n-- <= 0) {
        return 0;
    }
    unsigned long long int t1 = 0, t2 = 1, t;
    while (n--) {
        t = t1 + t2;
        t1 = t2;
        t2 = t;
    }
    return t2;
}