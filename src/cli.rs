use clap;
use clap::{App, Arg, SubCommand};

pub(crate) fn clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("UT181A-CLI")
        .version("1.0")
        .author("Anton Ageev <antage@gmail.com>")
        .about("Remote control for DMM UT181A.")
        .global_setting(clap::AppSettings::ColorAuto)
        .global_setting(clap::AppSettings::ColoredHelp)
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg_from_usage("-d --device=[DEVICE] 'Open device at DEVICE path'")
        .arg_from_usage("-v --verbose 'Verbose mode'")
        .subcommand(SubCommand::with_name("list-devices").about("Enumerate all connected DMMs"))
        .subcommand(SubCommand::with_name("hold").about("Hold current measurement"))
        .subcommand(
            SubCommand::with_name("min-max-mode")
                .about("Min/Max mode commands")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("on").about("Turn on or restart Min/Max mode"))
                .subcommand(SubCommand::with_name("off").about("Turn off Min/Max mode")),
        )
        .subcommand(SubCommand::with_name("save").about("Save current measurement in DMM memory"))
        .subcommand(SubCommand::with_name("save-count").about("Get count of saved measurements"))
        .subcommand(
            SubCommand::with_name("read-save")
                .about("Get saved measurement")
                .arg(
                    Arg::with_name("INDEX")
                        .help("Entry index (it starts at 1)")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("delete-all-save").about("Delete all saved measurements"))
        .subcommand(
            SubCommand::with_name("delete-save")
                .about("Delete save entry with specified index")
                .arg(
                    Arg::with_name("INDEX")
                        .help("Entry index")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("record-count").about("Get count of records"))
        .subcommand(SubCommand::with_name("record-list").about("Get list of records"))
        .subcommand(
            SubCommand::with_name("record")
                .about("Get data of record")
                .arg(
                    Arg::with_name("INDEX")
                        .help("Record index")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("ref")
                .about("Set reference value in relative mode")
                .arg(
                    Arg::with_name("VALUE")
                        .help("Reference value")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("range")
                .about("Set measuring range")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("auto").about("Auto range"))
                .subcommand(
                    SubCommand::with_name("step1")
                        .about("60 mV/6 V/600 uA/60 mA/600 Ohm/60 Hz/6 nF range"),
                )
                .subcommand(
                    SubCommand::with_name("step2")
                        .about("600 mV/60 V/6000 uA/600 mA/6 KOhm/600 Hz/60 nF range"),
                )
                .subcommand(SubCommand::with_name("step3").about("600V/60 KOhm/6 KHz/600 nF range"))
                .subcommand(
                    SubCommand::with_name("step4").about("1000 V/600 KOhm/60 KHz/6 uF range"),
                )
                .subcommand(SubCommand::with_name("step5").about("6 MOhm/600 KHz/60 uF range"))
                .subcommand(SubCommand::with_name("step6").about("60 MOhm/6 MHz/600 uF range"))
                .subcommand(SubCommand::with_name("step7").about("60 MHz/6 mF range"))
                .subcommand(SubCommand::with_name("step8").about("60 mF range")),
        )
        .subcommand(
            SubCommand::with_name("mode")
                .about("Set measuring mode")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("vac").about("VAC"))
                .subcommand(SubCommand::with_name("vac-rel").about("VAC/Rel"))
                .subcommand(SubCommand::with_name("vac-hz").about("VAC/Hz"))
                .subcommand(SubCommand::with_name("vac-peak").about("VAC/Peak"))
                .subcommand(SubCommand::with_name("vac-lowpass").about("VAC/LowPass"))
                .subcommand(SubCommand::with_name("vac-lowpass-rel").about("VAC/LowPass/Rel"))
                .subcommand(SubCommand::with_name("vac-dbv").about("VAC/dBV"))
                .subcommand(SubCommand::with_name("vac-dbv-rel").about("VAC/dBV/Rel"))
                .subcommand(SubCommand::with_name("vac-dbm").about("VAC/dBm"))
                .subcommand(SubCommand::with_name("vac-dbm-rel").about("VAC/dBm/Rel"))
                .subcommand(SubCommand::with_name("mvac").about("mVAC"))
                .subcommand(SubCommand::with_name("mvac-rel").about("mVAC/Rel"))
                .subcommand(SubCommand::with_name("mvac-hz").about("mVAC/Hz"))
                .subcommand(SubCommand::with_name("mvac-peak").about("mVAC/Peak"))
                .subcommand(SubCommand::with_name("mvac-acdc").about("mVAC/AC+DC"))
                .subcommand(SubCommand::with_name("mvac-acdc-rel").about("mVAC/AC+DC/Rel"))
                .subcommand(SubCommand::with_name("vdc").about("VDC"))
                .subcommand(SubCommand::with_name("vdc-rel").about("VDC/Rel"))
                .subcommand(SubCommand::with_name("vdc-acdc").about("VDC/AC+DC"))
                .subcommand(SubCommand::with_name("vdc-acdc-rel").about("VDC/AC+DC/Rel"))
                .subcommand(SubCommand::with_name("vdc-peak").about("VDC/Peak"))
                .subcommand(SubCommand::with_name("mvdc").about("mVDC"))
                .subcommand(SubCommand::with_name("mvdc-rel").about("mVDC/Rel"))
                .subcommand(SubCommand::with_name("mvdc-peak").about("mVDC/Peak"))
                .subcommand(SubCommand::with_name("temp-c-t1t2").about("Temp C/T1,T2"))
                .subcommand(SubCommand::with_name("temp-c-t1t2-rel").about("Temp C/T1,T2/Rel"))
                .subcommand(SubCommand::with_name("temp-c-t2t1").about("Temp C/T2,T1"))
                .subcommand(SubCommand::with_name("temp-c-t2t1-rel").about("Temp C/T2,T1/Rel"))
                .subcommand(SubCommand::with_name("temp-c-t1t2-diff").about("Temp C/T1-T2"))
                .subcommand(SubCommand::with_name("temp-c-t2t1-diff").about("Temp C/T2-T1"))
                .subcommand(SubCommand::with_name("temp-f-t1t2").about("Temp F/T1,T2"))
                .subcommand(SubCommand::with_name("temp-f-t1t2-rel").about("Temp F/T1,T2/Rel"))
                .subcommand(SubCommand::with_name("temp-f-t2t1").about("Temp F/T2,T1"))
                .subcommand(SubCommand::with_name("temp-f-t2t1-rel").about("Temp F/T2,T1/Rel"))
                .subcommand(SubCommand::with_name("temp-f-t1t2-diff").about("Temp F/T1-T2"))
                .subcommand(SubCommand::with_name("temp-f-t2t1-diff").about("Temp F/T2-T1"))
                .subcommand(SubCommand::with_name("res").about("Resistance"))
                .subcommand(SubCommand::with_name("res-rel").about("Resistance/Rel"))
                .subcommand(SubCommand::with_name("beeper-short").about("Beeper/Short"))
                .subcommand(SubCommand::with_name("beeper-open").about("Beeper/Open"))
                .subcommand(SubCommand::with_name("adm").about("Admittance"))
                .subcommand(SubCommand::with_name("adm-rel").about("Admittance/Rel"))
                .subcommand(SubCommand::with_name("diode").about("Diode"))
                .subcommand(SubCommand::with_name("diode-alarm").about("Diode/Alarm"))
                .subcommand(SubCommand::with_name("cap").about("Capacitance"))
                .subcommand(SubCommand::with_name("cap-rel").about("Capacitance/Rel"))
                .subcommand(SubCommand::with_name("freq").about("Frequency"))
                .subcommand(SubCommand::with_name("freq-rel").about("Frequency/Rel"))
                .subcommand(SubCommand::with_name("duty").about("Duty cycle"))
                .subcommand(SubCommand::with_name("duty-rel").about("Duty cycle/Rel"))
                .subcommand(SubCommand::with_name("pulse").about("Pulse width"))
                .subcommand(SubCommand::with_name("pulse-rel").about("Pulse/Rel"))
                .subcommand(SubCommand::with_name("uadc").about("uADC"))
                .subcommand(SubCommand::with_name("uadc-rel").about("uADC/Rel"))
                .subcommand(SubCommand::with_name("uadc-acdc").about("uADC/AC+DC"))
                .subcommand(SubCommand::with_name("uadc-acdc-rel").about("uADC/AC+DC/Rel"))
                .subcommand(SubCommand::with_name("uadc-peak").about("uADC/Peak"))
                .subcommand(SubCommand::with_name("madc").about("mADC"))
                .subcommand(SubCommand::with_name("madc-rel").about("mADC/Rel"))
                .subcommand(SubCommand::with_name("madc-acdc").about("mADC/AC+DC"))
                .subcommand(SubCommand::with_name("madc-acdc-rel").about("mADC/AC+DC/Rel"))
                .subcommand(SubCommand::with_name("madc-peak").about("mADC/Peak"))
                .subcommand(SubCommand::with_name("adc").about("ADC"))
                .subcommand(SubCommand::with_name("adc-rel").about("ADC/Rel"))
                .subcommand(SubCommand::with_name("adc-acdc").about("ADC/AC+DC"))
                .subcommand(SubCommand::with_name("adc-acdc-rel").about("ADC/AC+DC/Rel"))
                .subcommand(SubCommand::with_name("adc-peak").about("ADC/Peak"))
                .subcommand(SubCommand::with_name("uaac").about("uAAC"))
                .subcommand(SubCommand::with_name("uaac-rel").about("uAAC/Rel"))
                .subcommand(SubCommand::with_name("uaac-hz").about("uAAC/Hz"))
                .subcommand(SubCommand::with_name("uaac-peak").about("uAAC/Peak"))
                .subcommand(SubCommand::with_name("maac").about("mAAC"))
                .subcommand(SubCommand::with_name("maac-rel").about("mAAC/Rel"))
                .subcommand(SubCommand::with_name("maac-hz").about("mAAC/Hz"))
                .subcommand(SubCommand::with_name("maac-peak").about("mAAC/Peak"))
                .subcommand(SubCommand::with_name("aac").about("AAC"))
                .subcommand(SubCommand::with_name("aac-rel").about("AAC/Rel"))
                .subcommand(SubCommand::with_name("aac-hz").about("AAC/Hz"))
                .subcommand(SubCommand::with_name("aac-peak").about("AAC/Peak")),
        )
        .subcommand(SubCommand::with_name("read-once").about("Read current measurement once"))
        .subcommand(
            SubCommand::with_name("read-cont").about("Read current measurement continuously"),
        )
}
