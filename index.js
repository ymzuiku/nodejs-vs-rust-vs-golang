const rs = require('./index.node');

function fibonacci(n) {
  if (n == 0 || n == 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

async function start() {
  console.time("rs");
  rs.fib32(42);
  console.timeEnd("rs");
  await new Promise(res=>setTimeout(res, 200));
  
  console.time("rs");
  rs.fib64(42);
  console.timeEnd("rs");
  await new Promise(res=>setTimeout(res, 200));

  console.time("js");
  fibonacci(42);
  console.timeEnd("js");

  console.time("js");
  fibonacci(42.0);
  console.timeEnd("js");
}

start();