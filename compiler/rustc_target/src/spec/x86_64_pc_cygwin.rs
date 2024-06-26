use crate::spec::{Cc, LinkerFlavor, Lld, Target};

pub fn target() -> Target {
    let mut base = super::cygwin_base::opts();
    base.cpu = "x86-64".into();
    // FIXME: Disable ASLR for now to fix relocation error
    base.add_pre_link_args(
        LinkerFlavor::Gnu(Cc::No, Lld::No),
        &["-m", "i386pep", "--disable-high-entropy-va"],
    );
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64", "-Wl,--disable-high-entropy-va"]);
    base.max_atomic_width = Some(64);
    base.linker = Some("x86_64-pc-cygwin-gcc".into());

    Target {
        llvm_target: "x86_64-unknown-windows-cygnus".into(),
        pointer_width: 64,
        data_layout: "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .into(),
        arch: "x86_64".into(),
        options: base,
    }
}
