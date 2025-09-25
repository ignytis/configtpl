use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use minijinja::Environment;

use crate::shared_lib::ffi::{
    types::{
        collections::Array,
        std_types::ConstCharPtr,
        lib_types,
    },
    utils::strings::{cchar_const_deallocate, cchar_to_string}
};

/// cbindgen:no-export
static ENVIRONMENTS: LazyLock<Mutex<Vec<Option<Environment>>>> = LazyLock::new(|| {
    Mutex::new(Vec::new())
});

/// This function should be invoked before any other library routines.
/// Right at the moment it does nothing, but some initialization might be added in the future.
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_init() {

}

#[unsafe(no_mangle)]
pub extern "C" fn configtpl_new_environment() -> lib_types::EnrironmentHandle {
    let mut envs = ENVIRONMENTS.lock().unwrap();
    let mut env = Environment::new();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
    envs.push(Some(env));
    (envs.len() - 1) as lib_types::EnrironmentHandle
}

#[unsafe(no_mangle)]
pub extern "C" fn configtpl_render(env_handle: lib_types::EnrironmentHandle, tpl: ConstCharPtr, context: *const Array<[ConstCharPtr; 2]>) -> *mut lib_types::RenderResult {
    let envs = ENVIRONMENTS.lock().unwrap();
    let env = match envs.get(env_handle as usize) {
        Some(e) => match e {
            Some(e2) => Some(e2),
            None => None,
        },
        None => None,
    };
    let env = match env {
        Some(e) => e,
        None => {
            let res = Box::new(lib_types::RenderResult::new_invalid_handle());
            return Box::into_raw(res);
        }
    };

    let mut ctx_rs: HashMap<String, String> = HashMap::new();
    if !context.is_null() {
        let context_deref = unsafe {*context};
        for i in 0..context_deref.len {
            let [k, v] = unsafe { *context_deref.data.offset(i as isize) }.map(|x| cchar_to_string(x));
            ctx_rs.insert(k, v);
        }
    }

    let res: lib_types::RenderResult = match env.render_str(cchar_to_string(tpl).as_str(), ctx_rs) {
        Ok(s) => s.into(),
        Err(e) => e.into(),
    };

    let res = Box::new(res);
    return Box::into_raw(res);
}

/// Deallocates memory of rendering result object
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_render_free_result(r: *mut lib_types::RenderResult) {
    if r.is_null() {
        return
    }

    let r_box = unsafe { Box::from_raw(r) };
    cchar_const_deallocate(r_box.output);
}

/// Removes environment
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_free_environment(env_handle: lib_types::EnrironmentHandle) {
    let mut envs = ENVIRONMENTS.lock().unwrap();
    envs[env_handle as usize] = None;
}

/// This function should be invoked after any other library routines.
/// /// Right at the moment it does nothing, but some final actions might be added in the future.
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_unload() {

}