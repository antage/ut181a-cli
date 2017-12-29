use clap;
use clap::{App, Arg, SubCommand};

fn mode_subcommands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
        SubCommand::with_name("vac").about("VAC"),
        SubCommand::with_name("vac-rel").about("VAC/Rel"),
        SubCommand::with_name("vac-hz").about("VAC/Hz"),
        SubCommand::with_name("vac-peak").about("VAC/Peak"),
        SubCommand::with_name("vac-lowpass").about("VAC/LowPass"),
        SubCommand::with_name("vac-lowpass-rel").about("VAC/LowPass/Rel"),
        SubCommand::with_name("vac-dbv").about("VAC/dBV"),
        SubCommand::with_name("vac-dbv-rel").about("VAC/dBV/Rel"),
        SubCommand::with_name("vac-dbm").about("VAC/dBm"),
        SubCommand::with_name("vac-dbm-rel").about("VAC/dBm/Rel"),
        SubCommand::with_name("mvac").about("mVAC"),
        SubCommand::with_name("mvac-rel").about("mVAC/Rel"),
        SubCommand::with_name("mvac-hz").about("mVAC/Hz"),
        SubCommand::with_name("mvac-peak").about("mVAC/Peak"),
        SubCommand::with_name("mvac-acdc").about("mVAC/AC+DC"),
        SubCommand::with_name("mvac-acdc-rel").about("mVAC/AC+DC/Rel"),
        SubCommand::with_name("vdc").about("VDC"),
        SubCommand::with_name("vdc-rel").about("VDC/Rel"),
        SubCommand::with_name("vdc-acdc").about("VDC/AC+DC"),
        SubCommand::with_name("vdc-acdc-rel").about("VDC/AC+DC/Rel"),
        SubCommand::with_name("vdc-peak").about("VDC/Peak"),
        SubCommand::with_name("mvdc").about("mVDC"),
        SubCommand::with_name("mvdc-rel").about("mVDC/Rel"),
        SubCommand::with_name("mvdc-peak").about("mVDC/Peak"),
        SubCommand::with_name("temp-c-t1t2").about("Temp C/T1,T2"),
        SubCommand::with_name("temp-c-t1t2-rel").about("Temp C/T1,T2/Rel"),
        SubCommand::with_name("temp-c-t2t1").about("Temp C/T2,T1"),
        SubCommand::with_name("temp-c-t2t1-rel").about("Temp C/T2,T1/Rel"),
        SubCommand::with_name("temp-c-t1t2-diff").about("Temp C/T1-T2"),
        SubCommand::with_name("temp-c-t2t1-diff").about("Temp C/T2-T1"),
        SubCommand::with_name("temp-f-t1t2").about("Temp F/T1,T2"),
        SubCommand::with_name("temp-f-t1t2-rel").about("Temp F/T1,T2/Rel"),
        SubCommand::with_name("temp-f-t2t1").about("Temp F/T2,T1"),
        SubCommand::with_name("temp-f-t2t1-rel").about("Temp F/T2,T1/Rel"),
        SubCommand::with_name("temp-f-t1t2-diff").about("Temp F/T1-T2"),
        SubCommand::with_name("temp-f-t2t1-diff").about("Temp F/T2-T1"),
        SubCommand::with_name("res").about("Resistance"),
        SubCommand::with_name("res-rel").about("Resistance/Rel"),
        SubCommand::with_name("beeper-short").about("Beeper/Short"),
        SubCommand::with_name("beeper-open").about("Beeper/Open"),
        SubCommand::with_name("adm").about("Admittance"),
        SubCommand::with_name("adm-rel").about("Admittance/Rel"),
        SubCommand::with_name("diode").about("Diode"),
        SubCommand::with_name("diode-alarm").about("Diode/Alarm"),
        SubCommand::with_name("cap").about("Capacitance"),
        SubCommand::with_name("cap-rel").about("Capacitance/Rel"),
        SubCommand::with_name("freq").about("Frequency"),
        SubCommand::with_name("freq-rel").about("Frequency/Rel"),
        SubCommand::with_name("duty").about("Duty cycle"),
        SubCommand::with_name("duty-rel").about("Duty cycle/Rel"),
        SubCommand::with_name("pulse").about("Pulse width"),
        SubCommand::with_name("pulse-rel").about("Pulse/Rel"),
        SubCommand::with_name("uadc").about("uADC"),
        SubCommand::with_name("uadc-rel").about("uADC/Rel"),
        SubCommand::with_name("uadc-acdc").about("uADC/AC+DC"),
        SubCommand::with_name("uadc-acdc-rel").about("uADC/AC+DC/Rel"),
        SubCommand::with_name("uadc-peak").about("uADC/Peak"),
        SubCommand::with_name("madc").about("mADC"),
        SubCommand::with_name("madc-rel").about("mADC/Rel"),
        SubCommand::with_name("madc-acdc").about("mADC/AC+DC"),
        SubCommand::with_name("madc-acdc-rel").about("mADC/AC+DC/Rel"),
        SubCommand::with_name("madc-peak").about("mADC/Peak"),
        SubCommand::with_name("adc").about("ADC"),
        SubCommand::with_name("adc-rel").about("ADC/Rel"),
        SubCommand::with_name("adc-acdc").about("ADC/AC+DC"),
        SubCommand::with_name("adc-acdc-rel").about("ADC/AC+DC/Rel"),
        SubCommand::with_name("adc-peak").about("ADC/Peak"),
        SubCommand::with_name("uaac").about("uAAC"),
        SubCommand::with_name("uaac-rel").about("uAAC/Rel"),
        SubCommand::with_name("uaac-hz").about("uAAC/Hz"),
        SubCommand::with_name("uaac-peak").about("uAAC/Peak"),
        SubCommand::with_name("maac").about("mAAC"),
        SubCommand::with_name("maac-rel").about("mAAC/Rel"),
        SubCommand::with_name("maac-hz").about("mAAC/Hz"),
        SubCommand::with_name("maac-peak").about("mAAC/Peak"),
        SubCommand::with_name("aac").about("AAC"),
        SubCommand::with_name("aac-rel").about("AAC/Rel"),
        SubCommand::with_name("aac-hz").about("AAC/Hz"),
        SubCommand::with_name("aac-peak").about("AAC/Peak"),
    ]
}

fn range_subcommands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
        SubCommand::with_name("auto").about("Auto range"),
        SubCommand::with_name("step1").about("60 mV/6 V/600 uA/60 mA/600 Ohm/60 Hz/6 nF range"),
        SubCommand::with_name("step2")
            .about("600 mV/60 V/6000 uA/600 mA/6 KOhm/600 Hz/60 nF range"),
        SubCommand::with_name("step3").about("600V/60 KOhm/6 KHz/600 nF range"),
        SubCommand::with_name("step4").about("1000 V/600 KOhm/60 KHz/6 uF range"),
        SubCommand::with_name("step5").about("6 MOhm/600 KHz/60 uF range"),
        SubCommand::with_name("step6").about("60 MOhm/6 MHz/600 uF range"),
        SubCommand::with_name("step7").about("60 MHz/6 mF range"),
        SubCommand::with_name("step8").about("60 mF range"),
    ]
}

fn save_subcommands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
        SubCommand::with_name("store").about("Save current measurement in DMM memory"),
        SubCommand::with_name("count").about("Get count of saved measurements"),
        SubCommand::with_name("read")
            .about("Read saved measurement")
            .arg(
                Arg::with_name("INDEX")
                    .help("Entry index (it starts at 1)")
                    .required(true)
                    .index(1),
            ),
        SubCommand::with_name("delete-all").about("Delete all saved measurements"),
        SubCommand::with_name("delete")
            .about("Delete save entry with specified index")
            .arg(
                Arg::with_name("INDEX")
                    .help("Entry index")
                    .required(true)
                    .index(1),
            ),
    ]
}

fn record_subcommands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
        SubCommand::with_name("count").about("Get count of records"),
        SubCommand::with_name("list").about("Get list of records"),
        SubCommand::with_name("read")
            .about("Get data of record")
            .arg(
                Arg::with_name("INDEX")
                    .help("Record index")
                    .required(true)
                    .index(1),
            ),
        SubCommand::with_name("start")
            .about("Start new recording")
            .arg(
                Arg::with_name("NAME")
                    .help("Record name")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("INTERVAL")
                    .help("Sampling interval in seconds")
                    .required(true)
                    .index(2),
            )
            .arg(
                Arg::with_name("DURATION")
                    .help("Recording duration in minutes")
                    .required(true)
                    .index(3),
            ),
        SubCommand::with_name("stop").about("Stop current recording"),
    ]
}

pub(crate) fn clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("UT181A-CLI")
        .version("0.2.0")
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
                .about("Set measuring range  commands")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("auto").about("Auto range"))
                .subcommands(range_subcommands()),
        )
        .subcommand(
            SubCommand::with_name("mode")
                .about("Set measuring mode commands")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .subcommands(mode_subcommands()),
        )
        .subcommand(
            SubCommand::with_name("read")
                .about("Read measurement commands")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("once").about("Read current measurement once"))
                .subcommand(
                    SubCommand::with_name("cont").about("Read current measurement continuously"),
                ),
        )
        .subcommand(
            SubCommand::with_name("save")
                .about("Saved measurements commands")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .subcommands(save_subcommands()),
        )
        .subcommand(
            SubCommand::with_name("record")
                .about("Record management commands")
                .subcommands(record_subcommands()),
        )
}
