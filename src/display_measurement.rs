use std::time::Duration;

use ut181a::{Measurement, Mode, Range};

use error::*;

pub(crate) fn format_duration(d: Duration) -> String {
    let s = d.as_secs() % 60;
    let m = (d.as_secs() - s) / 60;
    let h = (d.as_secs() - (m * 60) - s) / 3600;

    format!("{}:{:02}:{:02}", h, m, s)
}

fn display_mvdc_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("-60...60 mV"),
        Range::Step2 => print!("-600...600 mV"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_mvac_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...60 mV"),
        Range::Step2 => print!("0...600 mV"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_vdc_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("-6...6 V"),
        Range::Step2 => print!("-60...60 V"),
        Range::Step3 => print!("-600...600 V"),
        Range::Step4 => print!("-1000...1000 V"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_vac_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...6 V"),
        Range::Step2 => print!("0...60 V"),
        Range::Step3 => print!("0...600 V"),
        Range::Step4 => print!("0...1000 V"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_no_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("-"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_ohm_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...600 Ohm"),
        Range::Step2 => print!("0...6 kOhm"),
        Range::Step3 => print!("0...60 kOhm"),
        Range::Step4 => print!("0...600 kOhm"),
        Range::Step5 => print!("0...6 MOhm"),
        Range::Step6 => print!("0...60 MOhm"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_siemens_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...60 nS"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_diode_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...3 V"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_farad_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...6 nF"),
        Range::Step2 => print!("0...60 nF"),
        Range::Step3 => print!("0...600 nF"),
        Range::Step4 => print!("0...6 uF"),
        Range::Step5 => print!("0...60 uF"),
        Range::Step6 => print!("0...600 uF"),
        Range::Step7 => print!("0...6 mF"),
        Range::Step8 => print!("0...60 mF"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_hz_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...60 Hz"),
        Range::Step2 => print!("0...600 Hz"),
        Range::Step3 => print!("0...6 kHz"),
        Range::Step4 => print!("0...60 kHz"),
        Range::Step5 => print!("0...600 kHz"),
        Range::Step6 => print!("0...6 MHz"),
        Range::Step7 => print!("0...60 MHz"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_uadc_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("-600...600 uA"),
        Range::Step2 => print!("-6000...6000 uA"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_uaac_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...600 uA"),
        Range::Step2 => print!("0...6000 uA"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_madc_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("-60...60 mA"),
        Range::Step2 => print!("-600...600 mA"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_maac_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...60 mA"),
        Range::Step2 => print!("0...600 mA"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_adc_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("-20...20 A"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_aac_range(r: Range) -> Result<()> {
    match r {
        Range::Step1 => print!("0...20 A"),
        _ => {
            return Err("Unused range step".into());
        }
    }
    Ok(())
}

fn display_range(m: Mode, r: Range) -> Result<()> {
    match m {
        Mode::mVDC_Normal | Mode::mVDC_Normal_Rel | Mode::mVDC_Peak => display_mvdc_range(r),

        Mode::mVAC_Normal |
        Mode::mVAC_Normal_Rel |
        Mode::mVAC_Peak |
        Mode::mVAC_Hz |
        Mode::mVAC_AC_DC |
        Mode::mVAC_AC_DC_Rel => display_mvac_range(r),

        Mode::VDC_Normal |
        Mode::VDC_Normal_Rel |
        Mode::VDC_Peak |
        Mode::VDC_AC_DC |
        Mode::VDC_AC_DC_Rel => display_vdc_range(r),

        Mode::VAC_Normal |
        Mode::VAC_Normal_Rel |
        Mode::VAC_Peak |
        Mode::VAC_Hz |
        Mode::VAC_LowPass |
        Mode::VAC_LowPass_Rel |
        Mode::VAC_dBV |
        Mode::VAC_dBV_Rel |
        Mode::VAC_dBm |
        Mode::VAC_dBm_Rel => display_vac_range(r),

        Mode::TempC_T1_T2 |
        Mode::TempC_T1_T2_Rel |
        Mode::TempC_T2_T1 |
        Mode::TempC_T2_T1_Rel |
        Mode::TempC_T1_T2_Diff |
        Mode::TempC_T2_T1_Diff |
        Mode::TempF_T1_T2 |
        Mode::TempF_T1_T2_Rel |
        Mode::TempF_T2_T1 |
        Mode::TempF_T2_T1_Rel |
        Mode::TempF_T1_T2_Diff |
        Mode::TempF_T2_T1_Diff => display_no_range(r),
        Mode::Resistance | Mode::Resistance_Rel | Mode::Beeper_Open | Mode::Beeper_Short => {
            display_ohm_range(r)
        }
        Mode::Admittance | Mode::Admittance_Rel => display_siemens_range(r),
        Mode::Diode_Normal | Mode::Diode_Alarm => display_diode_range(r),
        Mode::Capacitance | Mode::Capacitance_Rel => display_farad_range(r),
        Mode::Frequency |
        Mode::Frequency_Rel |
        Mode::DutyCycle |
        Mode::DutyCycle_Rel |
        Mode::PulseWidth |
        Mode::PulseWidth_Rel => display_hz_range(r),

        Mode::uADC_Normal |
        Mode::uADC_Normal_Rel |
        Mode::uADC_Peak |
        Mode::uADC_AC_DC |
        Mode::uADC_AC_DC_Rel => display_uadc_range(r),

        Mode::uAAC_Normal | Mode::uAAC_Normal_Rel | Mode::uAAC_Peak | Mode::uAAC_Hz => {
            display_uaac_range(r)
        }

        Mode::mADC_Normal |
        Mode::mADC_Normal_Rel |
        Mode::mADC_Peak |
        Mode::mADC_AC_DC |
        Mode::mADC_AC_DC_Rel => display_madc_range(r),

        Mode::mAAC_Normal | Mode::mAAC_Normal_Rel | Mode::mAAC_Peak | Mode::mAAC_Hz => {
            display_maac_range(r)
        }

        Mode::ADC_Normal |
        Mode::ADC_Normal_Rel |
        Mode::ADC_Peak |
        Mode::ADC_AC_DC |
        Mode::ADC_AC_DC_Rel => display_adc_range(r),

        Mode::AAC_Normal | Mode::AAC_Normal_Rel | Mode::AAC_Peak | Mode::AAC_Hz => {
            display_aac_range(r)
        }
    }
}

pub(crate) fn display_measurement(m: &Measurement) -> Result<()> {
    match *m {
        Measurement::Normal {
            mode,
            is_holded,
            is_auto_range,
            range,
            main,
            aux1,
            aux2,
            fast,
        } => {
            let display_auto = if is_auto_range { "AUTO" } else { "" };
            let display_hold = if is_holded { "HOLD" } else { "" };
            println!("Mode: {} [{}] [{}]", mode, display_hold, display_auto);
            print!("Range: ");
            display_range(mode, range)?;
            println!();
            println!("{}", main);
            if let Some(aux1_val) = aux1 {
                println!("AUX1: {}", aux1_val);
            }
            if let Some(aux2_val) = aux2 {
                println!("AUX2: {}", aux2_val);
            }
            if let Some(fast_val) = fast {
                println!("FAST: {}", fast_val);
            }
        }
        Measurement::Relative {
            mode,
            is_holded,
            is_auto_range,
            range,
            relative,
            reference,
            measurement,
            fast,
        } => {
            let display_auto = if is_auto_range { "AUTO" } else { "" };
            let display_hold = if is_holded { "HOLD" } else { "" };
            println!("Mode: {} [{}] [{}]", mode, display_hold, display_auto);
            print!("Range: ");
            display_range(mode, range)?;
            println!();
            println!("REL: {}", relative);
            println!("REFERENCE: {}", reference);
            println!("MEASUREMENT: {}", measurement);
            if let Some(fast_val) = fast {
                println!("FAST: {}", fast_val);
            }
        }
        Measurement::MinMax {
            mode,
            is_holded,
            is_auto_range,
            range,
            main,
            max,
            max_time,
            average,
            average_time,
            min,
            min_time,
        } => {
            let display_auto = if is_auto_range { "AUTO" } else { "" };
            let display_hold = if is_holded { "HOLD" } else { "" };
            println!("Mode: {} [{}] [{}]", mode, display_hold, display_auto);
            print!("Range: ");
            display_range(mode, range)?;
            println!();
            println!("{}", main);
            println!("MAXIMUM: {}\t{}", max, format_duration(max_time));
            println!("AVERAGE: {}\t{}", average, format_duration(average_time));
            println!("MINIMUM: {}\t{}", min, format_duration(min_time));
        }
        Measurement::Peak {
            mode,
            is_holded,
            is_auto_range,
            range,
            min,
            max,
        } => {
            let display_auto = if is_auto_range { "AUTO" } else { "" };
            let display_hold = if is_holded { "HOLD" } else { "" };
            println!("Mode: {} [{}] [{}]", mode, display_hold, display_auto);
            print!("Range: ");
            display_range(mode, range)?;
            println!();
            println!("PEAK MAX: {}", max);
            println!("PEAK MIN: {}", min);
        }
    }
    Ok(())
}
