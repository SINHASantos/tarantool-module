use std::io;
use std::os::raw::{c_int, c_schar};

use luajit::ffi::{luaL_error, lua_State, lua_pushinteger};
use tester::{
    run_tests_console, ColorConfig, Options, OutputFormat, RunIgnored, ShouldPanic, TestDesc,
    TestDescAndFn, TestFn, TestName, TestOpts, TestType,
};

mod common;
mod test_box;
mod test_coio;
mod test_error;
mod test_fiber;
mod test_latch;
mod test_log;
mod test_transaction;
mod test_tuple;

macro_rules! tests {
    ($($func_name:expr,)*) => {
        vec![
            $(TestDescAndFn{
                desc: TestDesc{
                    name: TestName::StaticTestName(stringify!($func_name)),
                    ignore: false,
                    should_panic: ShouldPanic::No,
                    allow_fail: false,
                    test_type: TestType::UnitTest
                },
                testfn: TestFn::StaticTestFn($func_name)
            },)*
        ]
    }
}

fn run() -> Result<bool, io::Error> {
    run_tests_console(
        &TestOpts {
            list: false,
            filter: None,
            filter_exact: false,
            force_run_in_process: false,
            exclude_should_panic: false,
            run_ignored: RunIgnored::No,
            run_tests: true,
            bench_benchmarks: false,
            logfile: None,
            nocapture: false,
            color: ColorConfig::AutoColor,
            format: OutputFormat::Pretty,
            test_threads: Some(1),
            skip: vec![],
            time_options: None,
            options: Options::new(),
        },
        tests![
            test_fiber::test_fiber_new,
            test_fiber::test_fiber_new_with_attr,
            test_fiber::test_fiber_arg,
            test_fiber::test_fiber_cancel,
            test_fiber::test_fiber_wake,
            test_fiber::test_fiber_cond_signal,
            test_fiber::test_fiber_cond_broadcast,
            test_fiber::test_fiber_cond_timeout,
            test_box::test_space_get_by_name,
            test_box::test_space_get_system,
            test_box::test_index_get_by_name,
            test_box::test_box_insert,
            test_box::test_box_replace,
            test_box::test_box_delete,
            test_box::test_box_update,
            test_box::test_box_upsert,
            test_box::test_box_truncate,
            test_box::test_box_get,
            test_box::test_box_select,
            test_box::test_box_select_composite_key,
            test_box::test_box_len,
            test_box::test_box_random,
            test_box::test_box_min_max,
            test_box::test_box_count,
            test_box::test_box_extract_key,
            test_tuple::test_tuple_new_from_struct,
            test_tuple::test_tuple_field_count,
            test_tuple::test_tuple_size,
            test_tuple::test_tuple_into_struct,
            test_tuple::test_tuple_clone,
            test_tuple::test_tuple_compare,
            test_tuple::test_tuple_compare_with_key,
            test_error::test_error_last,
            test_coio::test_coio_accept,
            test_coio::test_coio_read_write,
            test_transaction::test_transaction_commit,
            test_transaction::test_transaction_rollback,
            test_log::test_log,
            test_latch::test_latch_lock,
            test_latch::test_latch_try_lock,
        ],
    )
}

#[no_mangle]
pub extern "C" fn luaopen_libtarantool_module_test_runner(l: *mut lua_State) -> c_int {
    match run() {
        Ok(is_success) => {
            unsafe { lua_pushinteger(l, (!is_success) as isize) };
            1
        }
        Err(e) => {
            unsafe { luaL_error(l, e.to_string().as_ptr() as *const c_schar) };
            0
        }
    }
}
