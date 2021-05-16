use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0 = cx.argument::<JsString>(0)?;
    let str = arg0.value(&mut cx);
    Ok(cx.string(format!("hello node by rust {}", &str)))
}

fn make_an_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let n = 100f64;
    let s = cx.string("make-an");
    let arg0 = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let v1 = cx.number(n + arg0);
    let v2 = cx.number(arg0);
    let b = cx.boolean(arg0 > 50f64);

    let arr: Handle<JsArray> = cx.empty_array();
    arr.set(&mut cx, 0, v1)?;
    arr.set(&mut cx, 1, s)?;
    arr.set(&mut cx, 2, b)?;
    arr.set(&mut cx, 3, v2)?;

    Ok(arr)
}

fn fibonacci32(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    return fibonacci32(n - 1) + fibonacci32(n - 2);
}

fn fibonacci64(n: f64) -> f64 {
    if n == 0f64 || n == 1f64 {
        return n;
    }
    return fibonacci64(n - 1f64) + fibonacci64(n - 2f64);
}

fn fib32(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let _n = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let n:i32 = _n.to_string().parse().unwrap();
    let out = f64::from(fibonacci32(n));
    return Ok(cx.number(out));
}

fn fib64(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let out = fibonacci64(n);
    return Ok(cx.number(out));
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // _fibonacci(45);
    cx.export_function("hello", hello)?;
    cx.export_function("makeAnArray", make_an_array)?;
    cx.export_function("fib32", fib32)?;
    cx.export_function("fib64", fib64)?;
    Ok(())
}
