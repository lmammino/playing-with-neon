use neon::prelude::*;
use rayon::prelude::*;

fn sum(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a_raw = cx.argument::<JsNumber>(0)?;
    let a = a_raw.value(&mut cx);
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
    Ok(cx.number(a + b))
}

fn sum_square(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let numbers_raw = cx.argument::<JsArray>(0)?;
    let raw_numbers = numbers_raw.to_vec(&mut cx)?;
    let mut numbers: Vec<f64> = Vec::with_capacity(raw_numbers.len());
    for n in raw_numbers {
        let n_raw: Handle<JsNumber> = n.downcast(&mut cx).or_throw(&mut cx)?;
        let value = n_raw.value(&mut cx);
        numbers.push(value);
    }

    let total: f64 = numbers.par_iter().map(|n| n * n).sum();

    // let total: f64 = numbers.iter().sum();

    Ok(cx.number(total))
}

fn sum_square_n(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let how_many = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let values = (0..how_many).into_par_iter();

    let total: usize = values.map(|n| n * n).sum();
    Ok(cx.number(total as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("sum", sum)?;
    cx.export_function("sumSquare", sum_square)?;
    cx.export_function("sumSquareN", sum_square_n)?;
    Ok(())
}
