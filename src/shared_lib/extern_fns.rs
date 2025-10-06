use std::sync::{LazyLock, Mutex};

use crate::{
    config_builder::ConfigBuilder,
    shared_lib::ffi::{types::{
            collections::ArrayStringKV, lib_types, std_types::ConstCharPtr
        }, utils::strings::{cchar_const_deallocate, cchar_to_string}}, types::config_param::ConfigParam
};

/// cbindgen:ignore
static CFG_BUILDERS: LazyLock<Mutex<Vec<Option<ConfigBuilder>>>> = LazyLock::new(|| {
    Mutex::new(Vec::new())
});

/// This function should be invoked before any other library routines.
/// Right at the moment it does nothing, but some initialization might be added in the future.
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_init() {

}

#[unsafe(no_mangle)]
pub extern "C" fn configtpl_new_config_builder() -> lib_types::CfgBuilderHandle {
    let builder = ConfigBuilder::new();
    let mut envs = CFG_BUILDERS.lock().unwrap();
    envs.push(Some(builder));
    (envs.len() - 1) as lib_types::CfgBuilderHandle
}

#[unsafe(no_mangle)]
pub extern "C" fn configtpl_build_from_files(env_handle: lib_types::CfgBuilderHandle, paths: ConstCharPtr,
                                              overrides: *const ArrayStringKV,
                                              ctx: *const ArrayStringKV) -> *const lib_types::BuildResult {
    let cfg_builders = CFG_BUILDERS.lock().unwrap();
    let cfg_builder = match cfg_builders.get(env_handle as usize) {
        Some(e) => match e {
            Some(e2) => Some(e2),
            None => None,
        },
        None => None,
    };
    let cfg_builder = match cfg_builder {
        Some(b) => b,
        None => return lib_types::BuildResult::new_error_invalid_handle().into(),
    };

    let overrides: Option<ConfigParam> = if overrides.is_null() {
        None
    } else {
        let overrides: ConfigParam = overrides.into();
        Some(overrides)
    };
    let ctx: Option<ConfigParam> = if ctx.is_null() {
        None
    } else {
        let ctx: ConfigParam = ctx.into();
        Some(ctx)
    };


    match cfg_builder.build_from_files(&cchar_to_string(paths), &overrides, &ctx) {
        Ok(v) => v.into(),
        Err(e) => lib_types::BuildResult::new_error_building(&e).into(),
    }

}

/// Deallocates memory of rendering result object
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_build_free_result(r: *mut lib_types::BuildResult) {
    if r.is_null() {
        return
    }

    let r_box = unsafe { Box::from_raw(r) };
    if r_box.error_msg.is_aligned() {
        cchar_const_deallocate(r_box.error_msg);
    }
    if r_box.output.data.is_aligned() {
        for i in 0..r_box.output.len {
            let data = unsafe { *r_box.output.data.offset(i as isize) };
            cchar_const_deallocate(data[0]);
            cchar_const_deallocate(data[1]);
        }

    }
}

/// Removes a config builder
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_free_config_builder(env_handle: lib_types::CfgBuilderHandle) {
    let mut envs = CFG_BUILDERS.lock().unwrap();
    envs[env_handle as usize] = None;
}

/// This function should be invoked after any other library routines.
/// /// Right at the moment it does nothing, but some final actions might be added in the future.
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_unload() {

}
