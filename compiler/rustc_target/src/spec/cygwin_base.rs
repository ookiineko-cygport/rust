use crate::spec::crt_objects;
use crate::spec::{cvs, Cc, DebuginfoKind, LinkerFlavor, Lld, SplitDebuginfo, TargetOptions};
use std::borrow::Cow;

pub fn opts() -> TargetOptions {
    let mut pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::No, Lld::No),
        &[
            // FIXME: Disable ASLR for now to fix relocation error
            "--disable-dynamicbase",
            "--enable-auto-image-base",
        ],
    );
    super::add_link_args(
        &mut pre_link_args,
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &[
            // Tell GCC to avoid linker plugins, because we are not bundling
            // them with Windows installer, and Rust does its own LTO anyways.
            "-fno-use-linker-plugin",
            "-Wl,--disable-dynamicbase",
            "-Wl,--enable-auto-image-base",
        ],
    );

    let cygwin_libs = &[
        "-lcygwin",
        "-lgcc",
        "-lcygwin",
        "-luser32",
        "-lkernel32",
    ];
    let mut late_link_args =
        TargetOptions::link_args(LinkerFlavor::Gnu(Cc::No, Lld::No), cygwin_libs);
    super::add_link_args(&mut late_link_args, LinkerFlavor::Gnu(Cc::Yes, Lld::No), cygwin_libs);
    // If any of our crates are dynamically linked then we need to use
    // the shared libgcc_s-dw2-1.dll. This is required to support
    // unwinding across DLL boundaries.
    let dynamic_unwind_libs = &["-lgcc_s"];
    let mut late_link_args_dynamic =
        TargetOptions::link_args(LinkerFlavor::Gnu(Cc::No, Lld::No), dynamic_unwind_libs);
    super::add_link_args(
        &mut late_link_args_dynamic,
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        dynamic_unwind_libs,
    );
    // If all of our crates are statically linked then we can get away
    // with statically linking the libgcc unwinding code. This allows
    // binaries to be redistributed without the libgcc_s-dw2-1.dll
    // dependency, but unfortunately break unwinding across DLL
    // boundaries when unwinding across FFI boundaries.
    let static_unwind_libs = &["-lgcc_eh", "-l:libpthread.a"];
    let mut late_link_args_static =
        TargetOptions::link_args(LinkerFlavor::Gnu(Cc::No, Lld::No), static_unwind_libs);
    super::add_link_args(
        &mut late_link_args_static,
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        static_unwind_libs,
    );

    TargetOptions {
        os: "cygwin".into(),
        vendor: "pc".into(),
        // FIXME(#13846) this should be enabled for cygwin
        function_sections: false,
        linker: Some("gcc".into()),
        dynamic_linking: true,
        dll_prefix: "".into(),
        dll_suffix: ".dll".into(),
        exe_suffix: ".exe".into(),
        families: cvs!["unix"],
        is_like_windows: true,
        allows_weak_linkage: false,
        pre_link_args,
        pre_link_objects: crt_objects::pre_mingw(),
        post_link_objects: crt_objects::post_mingw(),
        late_link_args,
        late_link_args_dynamic,
        late_link_args_static,
        abi_return_struct_as_int: true,
        emit_debug_gdb_scripts: false,
        requires_uwtable: true,
        eh_frame_header: false,
        // FIXME(davidtwco): Support Split DWARF on Cygwin - may require LLVM changes to
        // output DWO, despite using DWARF, doesn't use ELF..
        debuginfo_kind: DebuginfoKind::Pdb,
        supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
        ..Default::default()
    }
}
