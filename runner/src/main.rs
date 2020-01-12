use failure::format_err;
use ktest::{read_ktest, KTEST};

use probe_rs::{
    config::registry::{Registry, SelectionStrategy},
    coresight::access_ports::AccessPortError,
    flash::download::{
        download_file, download_file_with_progress_reporting, FileDownloadError, Format,
    },
    probe::{stlink, DebugProbe, DebugProbeError, DebugProbeType, MasterProbe, WireProtocol},
    session::Session,
    target::info::{self, ChipInfo},
};

fn main() {
    let mut probe = open_probe();
    println!("probe connected");

    let strategy = SelectionStrategy::ChipInfo(ChipInfo::read_from_rom_table(&mut probe).unwrap());
    println!("strategy {:?}", strategy);

    let strategy = SelectionStrategy::TargetIdentifier("stm32f411".into());

    let registry = Registry::from_builtin_families();

    let target = registry.get_target(strategy).unwrap();
    println!("target {:?}", target);

    let mut session = Session::new(target, probe);

    let mm = session.target.memory_map.clone();

    let path_str = "../target/thumbv7em-none-eabihf/debug/examples/f401_break";
    // programming
    download_file(
        &mut session,
        std::path::Path::new(&path_str.to_string().as_str()),
        Format::Elf,
        &mm,
    )
    .map_err(|e| format_err!("failed to flash {}: {}", path_str, e))
    .unwrap();

    // session.probe.target_reset().unwrap();
    let cpu_info = session
        .target
        .core
        .reset_and_halt(&mut session.probe)
        .unwrap();
    println!("Core stopped at address 0x{:08x}", cpu_info.pc);

    let mut data = [0u8; 4];
    session
        .target
        .core
        .read_block8(&mut session.probe, 0x0000_0000, &mut data)
        .unwrap();

    println!("stack {:?}, 0x{:08x}", data, u32::from_le_bytes(data));
    let mut data = [0u8; 4];
    session
        .target
        .core
        .read_block8(&mut session.probe, 0x0000_0004, &mut data)
        .unwrap();

    println!("reset {:?}, 0x{:08x}", data, u32::from_le_bytes(data));

    let cpu_info = session.target.core.step(&mut session.probe).unwrap();
    println!("Core stopped at address 0x{:08x}", cpu_info.pc);

    println!("run");
    session.target.core.run(&mut session.probe).unwrap();

    session
        .target
        .core
        .wait_for_core_halted(&mut session.probe)
        .unwrap();

    let cpu_info = session.target.core.halt(&mut session.probe).unwrap();
    println!("Core stopped at address 0x{:08x}", cpu_info.pc);
}

fn open_probe() -> MasterProbe {
    let mut devs = stlink::tools::list_stlink_devices();
    // just pick the first one
    let device = devs.get(0).unwrap();
    println!("device {:?}", device);
    let mut link = stlink::STLink::new_from_probe_info(&device).unwrap();

    link.attach(Some(WireProtocol::Swd)).unwrap();

    MasterProbe::from_specific_probe(link)
}
