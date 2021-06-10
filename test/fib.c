int fib(int n) {
  if(n < 2) return n;
  else {
    int a = 0;
    int b = 1;
    while(n > 1) {
      a = a + b;
      int tmp = a;
      a = b;
      b = tmp;
      n = n - 1;
    }
    return b;
  }
}
