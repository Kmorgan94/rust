use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S32".into(),
        llvm_target: "riscv32".into(),
        pointer_width: 32,
        arch: "riscv32".into(),

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            cpu: "generic-rv32".into(),
            llvm_abiname: "ilp32e".into(),
            max_atomic_width: Some(32),
            atomic_cas: false,
            features: "+e,+forced-atomics".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
