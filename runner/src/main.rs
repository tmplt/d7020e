use failure::format_err;
use ktest::{read_ktest, KTEST};

use probe_rs::{
    config::registry::{Registry, SelectionStrategy},
    coresight::access_ports::AccessPortError,
    coresight::memory::MI,
    flash::download::{
        download_file, download_file_with_progress_reporting, FileDownloadError, Format,
    },
    probe::{stlink, DebugProbe, DebugProbeError, DebugProbeType, MasterProbe, WireProtocol},
    session::Session,
    target::info::{self, ChipInfo},
};

// le byte order
fn main() {
    println!("read ktest file");

    let ktest = read_ktest("test000001.ktest").unwrap();
    println!("ktest {:?}", ktest);

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

    let path_str = "../target/thumbv7em-none-eabihf/debug/examples/f401_ktest";
    // programming

    // print!("flashing...");
    // download_file(
    //     &mut session,
    //     std::path::Path::new(&path_str.to_string().as_str()),
    //     Format::Elf,
    //     &mm,
    // )
    // .map_err(|e| format_err!("failed to flash {}: {}", path_str, e))
    // .unwrap();

    // println!("... done");

    let cpu_info = session
        .target
        .core
        .reset_and_halt(&mut session.probe)
        .unwrap();
    println!("Core stopped at address 0x{:08x}", cpu_info.pc);

    let data = session.probe.read32(0x0000_0000).unwrap();
    println!("stack 0x{:08x}", data);

    let data = session.probe.read32(0x0000_0004).unwrap();
    println!("reset 0x{:08x}", data);

    run_to_halt(&mut session);

    break_step(&mut session);

    run_to_halt(&mut session);

    break_step(&mut session);

    run_to_halt(&mut session);

    // session
    //     .target
    //     .core
    //     .wait_for_core_halted(&mut session.probe)
    //     .unwrap();
    // println!("Core stopped at address 0x{:08x}", cpu_info.pc);

    // session
    //     .probe
    //     .write_block32(0x2000_0000, &[0x0123_4567, 0x89ab_cdef])
    //     .unwrap();

    // let mut r = [0u32; 2];
    // session.probe.read_block32(0x2000_0000, &mut r).unwrap();

    // println!("0x2000_0000 = 0x{:08x}", r[0]);
    // println!("0x2000_0004 = 0x{:08x}", r[1]);

    // let cpu_info = session.target.core.step(&mut session.probe).unwrap();
    // println!("Core stopped at address 0x{:08x}", cpu_info.pc);

    // for (name, data) in ktest.objects {
    //     println!("run {}", name);
    //     session.target.core.run(&mut session.probe).unwrap();

    //     session
    //         .target
    //         .core
    //         .wait_for_core_halted(&mut session.probe)
    //         .unwrap();

    //     let cpu_info = session.target.core.halt(&mut session.probe).unwrap();
    //     println!("Core stopped at address 0x{:08x}", cpu_info.pc);

    //     set_symbolic(&mut session, &data);
    // }

    // println!("done and run");
    // session.target.core.run(&mut session.probe).unwrap();

    // session
    //     .target
    //     .core
    //     .wait_for_core_halted(&mut session.probe)
    //     .unwrap();
    // println!("Core stopped at address 0x{:08x}", cpu_info.pc);
    // println!("breapoint reached");
}

fn read_pc(session: &mut Session) {
    // try to read the program counter
    let pc_value = session
        .target
        .core
        .read_core_reg(&mut session.probe, session.target.core.registers().PC)
        .unwrap();

    let mut instr16 = [0u8; 2];
    session.probe.read_block8(pc_value, &mut instr16).unwrap();

    println!(
        "instr16 {:?}, {:b}, {:b}, {:x}",
        instr16, instr16[0], instr16[1], instr16[1]
    );
}

fn break_step(session: &mut Session) {
    // try to read the program counter
    let pc_value = session
        .target
        .core
        .read_core_reg(&mut session.probe, session.target.core.registers().PC)
        .unwrap();

    // the bkpt() is a 16 bit instruction, increment pc by 16 bits
    let new_pc_value = pc_value + 0x2;
    session
        .target
        .core
        .write_core_reg(
            &mut session.probe,
            session.target.core.registers().PC,
            new_pc_value,
        )
        .unwrap();
}

fn run_to_halt(session: &mut Session) {
    // Continue running
    session.target.core.run(&mut session.probe).unwrap();
    println!("running");
    session
        .target
        .core
        .wait_for_core_halted(&mut session.probe)
        .unwrap();

    let cpu_info = session.target.core.halt(&mut session.probe).unwrap();
    println!("Run: Core stopped at address 0x{:08x}", cpu_info.pc);
    read_pc(session);
}
// index is the oject number
fn set_symbolic(session: &mut Session, data: &[u8]) {
    let r0 = session
        .target
        .core
        .read_core_reg(&mut session.probe, 0.into())
        .unwrap();

    println!("r0 0x{:08x}", r0);
    println!("object {:?}", data);
    session.target.core.step(&mut session.probe).unwrap();
    // let r0 = session.probe.write_block8(r0, data).unwrap();
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
