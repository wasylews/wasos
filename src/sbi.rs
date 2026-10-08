use core::arch::asm;

pub struct SbiRet {
    error: i64,
    value: i64,
}

pub fn sbi_call(
    arg0: i64,
    arg1: i64,
    arg2: i64,
    arg3: i64,
    arg4: i64,
    arg5: i64,
    fid: i64,
    eid: i64,
) -> SbiRet {
    let mut ret = SbiRet { error: 0, value: 0 };

    unsafe {
        asm!(
            "ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            lateout("a0") ret.error,
            lateout("a1") ret.value,
            options(nostack)
        );
    }

    ret
}
