use crate::spec::{LinkArgs, LinkerFlavor, PanicStrategy, TargetOptions};
use crate::spec::{RelocModel, TlsModel};

pub fn opts() -> TargetOptions {
    let mut args = LinkArgs::new();
    args.insert(
        LinkerFlavor::Gcc,
        vec![
            "-Wl,--as-needed".to_string(),
            "-Wl,-z,noexecstack".to_string(),
            "-Wl,-e,_main".to_string(),
            "-Wl,-z,noseparate-code".to_string(),
            "-Wl,-z,max-page-size=0x200000".to_string(),
        ],
    );

    let mut late_link_args = LinkArgs::new();
    late_link_args.insert(LinkerFlavor::Gcc, vec!["-lc".to_string()]);

    TargetOptions {
        os: "plan9".to_string(),
        os_family: Some("plan9".to_string()),
        executables: true,
        dynamic_linking: false,
        panic_strategy: PanicStrategy::Abort,
        has_elf_tls: false,
        // singlethread: true,
        tls_model: TlsModel::LocalExec,
        relocation_model: RelocModel::Static,
        linker_is_gnu: true,
        pre_link_args: args,
        late_link_args,
        ..Default::default()
    }
}
