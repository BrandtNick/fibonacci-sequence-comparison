
const fibonacci = (n) => {
    if (n <= 1) return 1;
    return fibonacci(n - 1) + fibonacci(n - 2)
}
const start = Date.now();
fibonacci(50);
const end = (Date.now() - start) / 1000;
console.log(end.toFixed(2) + ' s');
