error_chain! {
    links {
        Dmm(::ut181a::error::Error, ::ut181a::error::ErrorKind);
    }
    foreign_links {
        Clap(::clap::Error);
        FloatParse(::std::num::ParseFloatError);
        IntParse(::std::num::ParseIntError);
    }
    errors {
        UnknownCliCommand(cmd: String) {
            description("Unknown CLI command error")
            display("Unknown CLI command '{}'", cmd)
        }
    }
}
