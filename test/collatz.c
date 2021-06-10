unsigned collatz(unsigned n) {
  unsigned m = 0;
  while(n > 1) {
    if(n & 1) {
      n = n * 3 + 1;
    } else {
      n = n / 2;
    }
    if(n > m) {
      m = n;
    } else {}
  }
  return m;
}
