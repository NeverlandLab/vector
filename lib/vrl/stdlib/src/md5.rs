use ::value::Value;
use md5::Digest;
use vrl::prelude::*;

fn md5(value: Value) -> Resolved {
    let value = value.try_bytes()?;
    Ok(hex::encode(md5::Md5::digest(&value)).into())
}

#[derive(Clone, Copy, Debug)]
pub struct Md5;

impl Function for Md5 {
    fn identifier(&self) -> &'static str {
        "md5"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[Parameter {
            keyword: "value",
            kind: kind::ANY,
            required: true,
        }]
    }

    fn examples(&self) -> &'static [Example] {
        &[Example {
            title: "md5",
            source: r#"md5("foobar")"#,
            result: Ok("3858f62230ac3c915f300c664312c63f"),
        }]
    }

    fn compile(
        &self,
        _state: (&mut state::LocalEnv, &mut state::ExternalEnv),
        _ctx: &mut FunctionCompileContext,
        mut arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        Ok(Box::new(Md5Fn { value }))
    }

    fn symbol(&self) -> Option<(&'static str, usize)> {
        // TODO
        None
    }
}

#[derive(Debug, Clone)]
struct Md5Fn {
    value: Box<dyn Expression>,
}

impl Expression for Md5Fn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        md5(value)
    }

    fn type_def(&self, _: (&state::LocalEnv, &state::ExternalEnv)) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn vrl_fn_md5(value: &mut Value, result: &mut Resolved) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        md5 => Md5;

        md5 {
            args: func_args![value: "foo"],
            want: Ok(value!("acbd18db4cc2f85cedef654fccc4a4d8")),
            tdef: TypeDef::bytes().infallible(),
        }
    ];
}
