error_chain! {
    links {
        Dmm(::ut181a::Error, ::ut181a::ErrorKind);
    }
    foreign_links {
        HidError(::hid::Error);
        Clap(::clap::Error);
        FloatParse(::std::num::ParseFloatError);
        IntParse(::std::num::ParseIntError);
    }
    errors {
        DmmIsNotFound {
            description("DMM is not found")
            display("DMM is not found")
        }
        UnknownCliCommand(cmd: String) {
            description("Unknown CLI command error")
            display("unknown CLI command '{}'", cmd)
        }
    }
}
