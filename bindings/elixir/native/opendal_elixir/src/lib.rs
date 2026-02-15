use rustler::{Env, Term, NifResult, ResourceArc, OwnedBinary, Encoder};
use opendal::Operator as AsyncOperator;
use opendal::blocking::Operator as BlockingOperator;
use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime")
    })
}

struct OpenDALOperator {
    op: BlockingOperator,
}

#[rustler::nif(schedule = "DirtyIo")]
fn new<'a>(env: Env<'a>, scheme_str: &str, map: HashMap<String, String>) -> NifResult<Term<'a>> {
    let async_op = match AsyncOperator::via_iter(scheme_str, map) {
        Ok(o) => o,
        Err(e) => return Ok((rustler::types::atom::error(), e.to_string()).encode(env)),
    };

    let rt = get_runtime();
    let _guard = rt.enter();

    let op = match BlockingOperator::new(async_op) {
        Ok(o) => o,
        Err(e) => return Ok((rustler::types::atom::error(), e.to_string()).encode(env)),
    };

    Ok((rustler::types::atom::ok(), ResourceArc::new(OpenDALOperator { op })).encode(env))
}

#[rustler::nif(schedule = "DirtyIo")]
fn write<'a>(env: Env<'a>, resource: ResourceArc<OpenDALOperator>, path: String, content: String) -> NifResult<Term<'a>> {
    match resource.op.write(&path, content) {
        Ok(_) => Ok(rustler::types::atom::ok().to_term(env)),
        Err(e) => Ok((rustler::types::atom::error(), e.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn read<'a>(env: Env<'a>, resource: ResourceArc<OpenDALOperator>, path: String) -> NifResult<Term<'a>> {
    match resource.op.read(&path) {
        Ok(content) => {
            let content_bytes = content.to_vec();
            let mut binary = OwnedBinary::new(content_bytes.len()).unwrap();
            binary.as_mut_slice().copy_from_slice(&content_bytes);
            Ok((rustler::types::atom::ok(), binary.release(env)).encode(env))
        },
        Err(e) => Ok((rustler::types::atom::error(), e.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn delete<'a>(env: Env<'a>, resource: ResourceArc<OpenDALOperator>, path: String) -> NifResult<Term<'a>> {
    match resource.op.delete(&path) {
        Ok(_) => Ok(rustler::types::atom::ok().to_term(env)),
        Err(e) => Ok((rustler::types::atom::error(), e.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn stat<'a>(env: Env<'a>, resource: ResourceArc<OpenDALOperator>, path: String) -> NifResult<Term<'a>> {
    match resource.op.stat(&path) {
        Ok(_) => Ok(rustler::types::atom::ok().to_term(env)),
        Err(e) => Ok((rustler::types::atom::error(), e.to_string()).encode(env)),
    }
}

rustler::init!("Elixir.OpenDAL.Native", load = load);

fn load(env: Env, _info: Term) -> bool {
    let _ = rustler::resource!(OpenDALOperator, env);
    true
}
