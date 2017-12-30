extern crate ansi_term;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate hid;
extern crate ut181a;

mod error;
use error::*;
mod cli;
mod display_measurement;

use std::path::Path;

use clap::ArgMatches;

use ut181a::{Dmm, Mode, Range};
use cli::clap_app;

use display_measurement::{display_measurement, format_duration};

const VENDOR_ID: u16 = 0x10C4;
const PRODUCT_ID: u16 = 0xEA80;

fn open_dmm<'a>(manager: &hid::Manager, cli: &ArgMatches) -> Result<Dmm> {
    let mut devices = manager.find(Some(VENDOR_ID), Some(PRODUCT_ID));
    if let Some(path) = cli.value_of("device") {
        if cli.is_present("verbose") {
            println!("Open device at path '{}'.", path);
        }
        let p = Path::new(path);
        for device in devices {
            if device.path() == p {
                return Dmm::new(device.open()?).map_err(Error::from);
            }
        }
    } else {
        if cli.is_present("verbose") {
            println!("Open first found device.");
        }
        if let Some(device) = devices.next() {
            return Dmm::new(device.open()?).map_err(Error::from);
        }
    }
    Err(ErrorKind::DmmIsNotFound.into())
}

fn set_mode(dmm: &mut Dmm, mode: Mode, verbose: bool) -> Result<()> {
    if verbose {
        println!("Sending 'SET MODE {}' command to DMM.", mode);
    }
    dmm.set_mode(mode).map_err(Error::from)
}

fn run() -> Result<()> {
    let cli = clap_app().get_matches();

    let manager = hid::init()?;

    let verbose = cli.is_present("verbose");
    match cli.subcommand() {
        ("list-devices", _) => for device in manager.find(Some(VENDOR_ID), Some(PRODUCT_ID)) {
            println!("Found DMM at path '{}'.", device.path().to_string_lossy());
        },
        ("hold", _) => {
            let mut dmm = open_dmm(&manager, &cli)?;
            dmm.monitor_off()?;
            if verbose {
                println!("Sending 'HOLD' command to DMM.");
            }
            dmm.toggle_hold()?;
        }
        (cmd @ "min-max-mode", Some(sub_matches)) => {
            let mut dmm = open_dmm(&manager, &cli)?;
            dmm.monitor_off()?;
            match sub_matches.subcommand() {
                ("on", _) => {
                    if verbose {
                        println!("Sending 'MIN/MAX ON' command to DMM.");
                    }
                    dmm.set_min_max_mode(true)?;
                }
                ("off", _) => {
                    if verbose {
                        println!("Sending 'MIN/MAX OFF' command to DMM.");
                    }
                    dmm.set_min_max_mode(false)?;
                }
                (subcmd, _) => {
                    return Err(ErrorKind::UnknownCliCommand(format!("{} {}", cmd, subcmd)).into());
                }
            }
        }
        (cmd @ "save", Some(sub_matches)) => {
            let mut dmm = open_dmm(&manager, &cli)?;
            dmm.monitor_off()?;
            match sub_matches.subcommand() {
                ("store", _) => {
                    if verbose {
                        println!("Sending 'SAVE' command to DMM.");
                    }
                    dmm.save_measurement()?;
                }
                ("count", _) => {
                    if verbose {
                        println!("Sending 'GET SAVE COUNT' command to DMM.");
                    }
                    let count = dmm.get_saved_measurement_count()?;

                    println!("Save count: {}", count);
                }
                ("read", Some(read_matches)) => {
                    let n = read_matches
                        .value_of("INDEX")
                        .ok_or("Undefined index")?
                        .parse::<u16>()?;
                    if verbose {
                        println!("Sending 'READ SAVE AT {}' command to DMM.", n);
                    }
                    let save = dmm.get_saved_measurement(n)?;
                    println!("{}", save.0.format("%Y-%m-%d %H:%M:%S"));
                    display_measurement(&save.1)?;
                }
                ("delete-all", _) => {
                    if verbose {
                        println!("Sending 'DELETE ALL SAVE' command to DMM.");
                    }
                    dmm.delete_all_saved_measurement()?;
                }
                ("delete", Some(delete_matches)) => {
                    let index = delete_matches
                        .value_of("INDEX")
                        .ok_or("Undefined entry index")?
                        .parse::<u16>()?;
                    if verbose {
                        println!("Sending 'DELETE SAVE #{}' command to DMM.", index);
                    }
                    dmm.delete_saved_measurement(index)?;
                }
                (subcmd, _) => {
                    return Err(ErrorKind::UnknownCliCommand(format!("{} {}", cmd, subcmd)).into());
                }
            }
        }
        (cmd @ "record", Some(sub_matches)) => {
            let mut dmm = open_dmm(&manager, &cli)?;
            dmm.monitor_off()?;
            match sub_matches.subcommand() {
                ("count", _) => {
                    if verbose {
                        println!("Sending 'GET RECORD COUNT' command to DMM.");
                    }
                    let count = dmm.get_record_count()?;

                    println!("Record count: {}", count);
                }
                ("list", _) => {
                    if verbose {
                        println!("Sending 'GET RECORD COUNT' command to DMM.");
                    }
                    let count = dmm.get_record_count()?;

                    for i in 1..(count + 1) {
                        if verbose {
                            println!("Sending 'GET RECORD INFO #{}' command to DMM.", i);
                        }
                        let info = dmm.get_record_info(i)?;
                        println!("RECORD #{}:", i);
                        println!("\tName: {}", info.name);
                        println!("\tUnit: {}", info.unit);
                        println!("\tInterval: {}", format_duration(info.interval));
                        println!("\tDuration: {}", format_duration(info.duration));
                        println!("\tSample count: {}", info.sample_count);
                        println!("\tMaximum value: {}", info.max);
                        println!("\tAverage value: {}", info.average);
                        println!("\tMinimum value: {}", info.min);
                    }

                    println!("\nTotal record count: {}", count);
                }
                ("read", Some(read_matches)) => {
                    let n = read_matches
                        .value_of("INDEX")
                        .ok_or("Undefined index")?
                        .parse::<u16>()?;
                    if verbose {
                        println!("Sending 'GET RECORD DATA #{}' command to DMM.", n);
                    }
                    let items = dmm.get_record_data(n)?;
                    for (i, item) in items.iter().enumerate() {
                        println!("#{:06} {} {}", i + 1, item.timestamp, item.value);
                    }
                    println!("Total sample count: {}", items.len());
                }
                ("start", Some(start_matches)) => {
                    let name = start_matches.value_of("NAME").ok_or("Undefined name")?;
                    let interval = start_matches
                        .value_of("INTERVAL")
                        .ok_or("Undefined interval")?
                        .parse::<u16>()?;
                    let duration = start_matches
                        .value_of("DURATION")
                        .ok_or("Undefined duration")?
                        .parse::<u32>()?;
                    if verbose {
                        println!("Sending 'RECORD START' command to DMM.");
                    }
                    dmm.start_record(name, interval, duration)?;
                }
                ("stop", _) => {
                    if verbose {
                        println!("Sending 'RECORD STOP' command to DMM.");
                    }
                    dmm.stop_record()?;
                }
                (subcmd, _) => {
                    return Err(ErrorKind::UnknownCliCommand(format!("{} {}", cmd, subcmd)).into());
                }
            }
        }
        ("ref", Some(submatches)) => {
            let val = submatches
                .value_of("VALUE")
                .ok_or("Undefined reference value")?
                .parse::<f32>()?;
            let mut dmm = open_dmm(&manager, &cli)?;
            dmm.monitor_off()?;
            if verbose {
                println!("Sending 'SET REFERENCE VALUE {}' command to DMM.", val);
            }
            dmm.set_reference_value(val)?;
        }
        (cmd @ "range", Some(sub_matches)) => {
            let mut dmm = open_dmm(&manager, &cli)?;
            dmm.monitor_off()?;
            match sub_matches.subcommand() {
                ("auto", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE AUTO' command to DMM.");
                    }
                    dmm.set_range(Range::Auto)?;
                }
                ("step1", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE 1' command to DMM.");
                    }
                    dmm.set_range(Range::Step1)?;
                }
                ("step2", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE 2' command to DMM.");
                    }
                    dmm.set_range(Range::Step2)?;
                }
                ("step3", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE 3' command to DMM.");
                    }
                    dmm.set_range(Range::Step3)?;
                }
                ("step4", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE 4' command to DMM.");
                    }
                    dmm.set_range(Range::Step4)?;
                }
                ("step5", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE 5' command to DMM.");
                    }
                    dmm.set_range(Range::Step5)?;
                }
                ("step6", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE 6' command to DMM.");
                    }
                    dmm.set_range(Range::Step6)?;
                }
                ("step7", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE 7' command to DMM.");
                    }
                    dmm.set_range(Range::Step7)?;
                }
                ("step8", _) => {
                    if verbose {
                        println!("Sending 'SET RANGE 8' command to DMM.");
                    }
                    dmm.set_range(Range::Step8)?;
                }
                (subcmd, _) => {
                    return Err(ErrorKind::UnknownCliCommand(format!("{} {}", cmd, subcmd)).into());
                }
            }
        }
        (cmd @ "mode", Some(submatches)) => {
            let mut dmm = open_dmm(&manager, &cli)?;
            dmm.monitor_off()?;
            match submatches.subcommand() {
                ("vac", _) => set_mode(&mut dmm, Mode::VAC_Normal, verbose)?,
                ("vac-rel", _) => set_mode(&mut dmm, Mode::VAC_Normal_Rel, verbose)?,
                ("vac-hz", _) => set_mode(&mut dmm, Mode::VAC_Hz, verbose)?,
                ("vac-peak", _) => set_mode(&mut dmm, Mode::VAC_Peak, verbose)?,
                ("vac-lowpass", _) => set_mode(&mut dmm, Mode::VAC_LowPass, verbose)?,
                ("vac-lowpass-rel", _) => set_mode(&mut dmm, Mode::VAC_LowPass_Rel, verbose)?,
                ("vac-dbv", _) => set_mode(&mut dmm, Mode::VAC_dBV, verbose)?,
                ("vac-dbv-rel", _) => set_mode(&mut dmm, Mode::VAC_dBV_Rel, verbose)?,
                ("vac-dbm", _) => set_mode(&mut dmm, Mode::VAC_dBm, verbose)?,
                ("vac-dbm-rel", _) => set_mode(&mut dmm, Mode::VAC_dBm_Rel, verbose)?,

                ("mvac", _) => set_mode(&mut dmm, Mode::mVAC_Normal, verbose)?,
                ("mvac-rel", _) => set_mode(&mut dmm, Mode::mVAC_Normal_Rel, verbose)?,
                ("mvac-hz", _) => set_mode(&mut dmm, Mode::mVAC_Hz, verbose)?,
                ("mvac-peak", _) => set_mode(&mut dmm, Mode::mVAC_Peak, verbose)?,
                ("mvac-acdc", _) => set_mode(&mut dmm, Mode::mVAC_AC_DC, verbose)?,
                ("mvac-acdc-rel", _) => set_mode(&mut dmm, Mode::mVAC_AC_DC_Rel, verbose)?,

                ("vdc", _) => set_mode(&mut dmm, Mode::VDC_Normal, verbose)?,
                ("vdc-rel", _) => set_mode(&mut dmm, Mode::VDC_Normal_Rel, verbose)?,
                ("vdc-acdc", _) => set_mode(&mut dmm, Mode::VDC_AC_DC, verbose)?,
                ("vdc-acdc-rel", _) => set_mode(&mut dmm, Mode::VDC_AC_DC_Rel, verbose)?,
                ("vdc-peak", _) => set_mode(&mut dmm, Mode::VDC_Peak, verbose)?,

                ("mvdc", _) => set_mode(&mut dmm, Mode::mVDC_Normal, verbose)?,
                ("mvdc-rel", _) => set_mode(&mut dmm, Mode::mVDC_Normal_Rel, verbose)?,
                ("mvdc-peak", _) => set_mode(&mut dmm, Mode::mVDC_Peak, verbose)?,

                ("temp-c-t1t2", _) => set_mode(&mut dmm, Mode::TempC_T1_T2, verbose)?,
                ("temp-c-t1t2-rel", _) => set_mode(&mut dmm, Mode::TempC_T1_T2_Rel, verbose)?,
                ("temp-c-t2t1", _) => set_mode(&mut dmm, Mode::TempC_T2_T1, verbose)?,
                ("temp-c-t2t1-rel", _) => set_mode(&mut dmm, Mode::TempC_T2_T1_Rel, verbose)?,
                ("temp-c-t1t2-diff", _) => set_mode(&mut dmm, Mode::TempC_T1_T2_Diff, verbose)?,
                ("temp-c-t2t1-diff", _) => set_mode(&mut dmm, Mode::TempC_T2_T1_Diff, verbose)?,

                ("temp-f-t1t2", _) => set_mode(&mut dmm, Mode::TempF_T1_T2, verbose)?,
                ("temp-f-t1t2-rel", _) => set_mode(&mut dmm, Mode::TempF_T1_T2_Rel, verbose)?,
                ("temp-f-t2t1", _) => set_mode(&mut dmm, Mode::TempF_T2_T1, verbose)?,
                ("temp-f-t2t1-rel", _) => set_mode(&mut dmm, Mode::TempF_T2_T1_Rel, verbose)?,
                ("temp-f-t1t2-diff", _) => set_mode(&mut dmm, Mode::TempF_T1_T2_Diff, verbose)?,
                ("temp-f-t2t1-diff", _) => set_mode(&mut dmm, Mode::TempF_T2_T1_Diff, verbose)?,

                ("res", _) => set_mode(&mut dmm, Mode::Resistance, verbose)?,
                ("res-rel", _) => set_mode(&mut dmm, Mode::Resistance_Rel, verbose)?,

                ("beeper-short", _) => set_mode(&mut dmm, Mode::Beeper_Short, verbose)?,
                ("beeper-open", _) => set_mode(&mut dmm, Mode::Beeper_Open, verbose)?,

                ("adm", _) => set_mode(&mut dmm, Mode::Admittance, verbose)?,
                ("adm-rel", _) => set_mode(&mut dmm, Mode::Admittance_Rel, verbose)?,

                ("diode", _) => set_mode(&mut dmm, Mode::Diode_Normal, verbose)?,
                ("diode-alarm", _) => set_mode(&mut dmm, Mode::Diode_Alarm, verbose)?,

                ("cap", _) => set_mode(&mut dmm, Mode::Capacitance, verbose)?,
                ("cap-rel", _) => set_mode(&mut dmm, Mode::Capacitance_Rel, verbose)?,

                ("freq", _) => set_mode(&mut dmm, Mode::Frequency, verbose)?,
                ("freq-rel", _) => set_mode(&mut dmm, Mode::Frequency_Rel, verbose)?,

                ("duty", _) => set_mode(&mut dmm, Mode::DutyCycle, verbose)?,
                ("duty-rel", _) => set_mode(&mut dmm, Mode::DutyCycle_Rel, verbose)?,

                ("pulse", _) => set_mode(&mut dmm, Mode::PulseWidth, verbose)?,
                ("pulse-rel", _) => set_mode(&mut dmm, Mode::PulseWidth_Rel, verbose)?,

                ("uadc", _) => set_mode(&mut dmm, Mode::uADC_Normal, verbose)?,
                ("uadc-rel", _) => set_mode(&mut dmm, Mode::uADC_Normal_Rel, verbose)?,
                ("uadc-acdc", _) => set_mode(&mut dmm, Mode::uADC_AC_DC, verbose)?,
                ("uadc-acdc-rel", _) => set_mode(&mut dmm, Mode::uADC_AC_DC_Rel, verbose)?,
                ("uadc-peak", _) => set_mode(&mut dmm, Mode::uADC_Peak, verbose)?,

                ("madc", _) => set_mode(&mut dmm, Mode::mADC_Normal, verbose)?,
                ("madc-rel", _) => set_mode(&mut dmm, Mode::mADC_Normal_Rel, verbose)?,
                ("madc-acdc", _) => set_mode(&mut dmm, Mode::mADC_AC_DC, verbose)?,
                ("madc-acdc-rel", _) => set_mode(&mut dmm, Mode::mADC_AC_DC_Rel, verbose)?,
                ("madc-peak", _) => set_mode(&mut dmm, Mode::mADC_Peak, verbose)?,

                ("adc", _) => set_mode(&mut dmm, Mode::ADC_Normal, verbose)?,
                ("adc-rel", _) => set_mode(&mut dmm, Mode::ADC_Normal_Rel, verbose)?,
                ("adc-acdc", _) => set_mode(&mut dmm, Mode::ADC_AC_DC, verbose)?,
                ("adc-acdc-rel", _) => set_mode(&mut dmm, Mode::ADC_AC_DC_Rel, verbose)?,
                ("adc-peak", _) => set_mode(&mut dmm, Mode::ADC_Peak, verbose)?,

                ("uaac", _) => set_mode(&mut dmm, Mode::uAAC_Normal, verbose)?,
                ("uaac-rel", _) => set_mode(&mut dmm, Mode::uAAC_Normal_Rel, verbose)?,
                ("uaac-hz", _) => set_mode(&mut dmm, Mode::uAAC_Hz, verbose)?,
                ("uaac-peak", _) => set_mode(&mut dmm, Mode::uAAC_Peak, verbose)?,

                ("maac", _) => set_mode(&mut dmm, Mode::mAAC_Normal, verbose)?,
                ("maac-rel", _) => set_mode(&mut dmm, Mode::mAAC_Normal_Rel, verbose)?,
                ("maac-hz", _) => set_mode(&mut dmm, Mode::mAAC_Hz, verbose)?,
                ("maac-peak", _) => set_mode(&mut dmm, Mode::mAAC_Peak, verbose)?,

                ("aac", _) => set_mode(&mut dmm, Mode::AAC_Normal, verbose)?,
                ("aac-rel", _) => set_mode(&mut dmm, Mode::AAC_Normal_Rel, verbose)?,
                ("aac-hz", _) => set_mode(&mut dmm, Mode::AAC_Hz, verbose)?,
                ("aac-peak", _) => set_mode(&mut dmm, Mode::AAC_Peak, verbose)?,

                (subcmd, _) => {
                    return Err(ErrorKind::UnknownCliCommand(format!("{} {}", cmd, subcmd)).into());
                }
            }
        }
        ("read-once", _) => {
            let mut dmm = open_dmm(&manager, &cli)?;

            if verbose {
                println!("Sending 'MONITOR ON' command to DMM.");
            }
            dmm.monitor_on()?;
            if verbose {
                println!("Reading a message from DMM.");
            }

            let measurement = dmm.get_measurement()?;
            display_measurement(&measurement)?;

            if verbose {
                println!("Sending 'MONITOR OFF' command to DMM.");
            }
            dmm.monitor_off()?;
        }
        ("read-cont", _) => {
            let mut dmm = open_dmm(&manager, &cli)?;

            if verbose {
                println!("Sending 'MONITOR ON' command to DMM.");
            }
            dmm.monitor_on()?;

            loop {
                if verbose {
                    println!("Reading a message from DMM.");
                }
                let measurement = dmm.get_measurement()?;
                display_measurement(&measurement)?;
            }
        }
        (cmd, _) => {
            return Err(ErrorKind::UnknownCliCommand(cmd.to_owned()).into());
        }
    }
    Ok(())
}

quick_main!(run);
