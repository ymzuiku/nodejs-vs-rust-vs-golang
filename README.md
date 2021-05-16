# fibonacci 运行效率对比

nodejs 运行的代码:

```js
function fibonacci(n) {
  if (n == 0 || n == 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}
```

rust 运行的代码:

```js
fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

```

go 运行的代码:

```go
func fibonacci(n int32) int32 {
	if n == 0 || n == 1 {
		return n
	}
	return fib(n-1) + fib(n-2)
}
```

以上代码执行 `fibonacci(42)` (运算 8.66988873 亿次)的开销：

| 环境                         | 数据类型               | 平均耗时     |
| ---------------------------- | ---------------------- | ------------ |
| nodejs                       | number(相当于 float64) | 3.32s~3.903s |
| golang                       | int32                  | 3.533771625s |
| golang                       | float64                | 5.038523542s |
| rust                         | i32                    | 0.894s       |
| rust                         | f64                    | 2.052s       |
| rust N-API(node 中调用 rust) | i32                    | 0.888013s    |
| rust N-API(node 中调用 rust) | f64                    | 2.048s       |

结论，仅仅是简单计算，nodejs 竟然 和 golang 没什么区别, nodejs 的运算时间经常有浮动，不如另两个稳定. 可以看出，rust 编译成 N-API，在 node 中调用，计算性能没有损失。

nodejs 中，为了防止阻塞，涉及 CPU 密集型的任务尽量使用 worker_threads 执行。
