__attribute__((section(".start"))) int main() {
  volatile register int x = 0xD3AD;
  volatile register int y = 0x1234;
  volatile register int temp = y;
  y = x;
  x = temp;
  if (x > y) {
    x = 1;
  } else {
    y = 3;
  }
}
