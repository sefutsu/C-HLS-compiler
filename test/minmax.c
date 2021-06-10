int max(int a, int b) {
  if(a < b) return b;
  else return a;
}
int min(int a, int b) {
  if(a < b) return a;
  else return b;
}

int minmax(int a, int b, int c, int d) {
  int e = max(a, b);
  int f = max(c, d);
  return min(e, f);
}