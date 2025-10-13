use std::sync::{LazyLock, Mutex};

use crate::{
    config_builder::ConfigBuilder,
    shared_lib::ffi::{types::{
            collections::ArrayStringKV,
            lib_types,
            std_types::ConstCharPtr
        }, utils::strings::{cchar_const_deallocate, cchar_to_string}}, types::config_param::ConfigParam
};

/// cbindgen:ignore
static CFG_BUILDERS: LazyLock<Mutex<Vec<Option<ConfigBuilder>>>> = LazyLock::new(|| {
    Mutex::new(Vec::new())
});

/// Creates a new instance of configuration builder
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_configbuilder_new() -> lib_types::CfgBuilderHandle {
    let builder = ConfigBuilder::new();
    let mut envs = CFG_BUILDERS.lock().unwrap();
    envs.push(Some(builder));
    (envs.len() - 1) as lib_types::CfgBuilderHandle
}

#[unsafe(no_mangle)]
pub extern "C" fn configtpl_configbuilder_build_from_files(env_handle: lib_types::CfgBuilderHandle, paths: ConstCharPtr,
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

/// Deallocates memory of configuration builder result
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_configbuilder_result_free(r: *const lib_types::BuildResult) {
    if r.is_null() {
        return
    }

    let mut r_box = unsafe { Box::from_raw(r as *mut lib_types::BuildResult) };
    if r_box.error_msg.is_aligned() {
        cchar_const_deallocate(r_box.error_msg);
    }
    r_box.output.free_contents();
}

/// Deallocates memory of configuration builder instance
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_configbuilder_free(env_handle: lib_types::CfgBuilderHandle) {
    let mut envs = CFG_BUILDERS.lock().unwrap();
    envs[env_handle as usize] = None;
}
