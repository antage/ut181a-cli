# ut181a-cli

Uni-T UT181A digital multimeter (DMM) remote contorl command-line interface.
It supports USB connection only.

## Installing

```
$ sudo apt-get install libudev-dev libhidapi-dev
$ cargo install ut181a-cli
```

## Usage

```
$ # Connect DMM to PC.
$ # Turn on DMM, press 'F4' ('SETUP') and set 'COMMUNICATION' to 'ON' state.
$ # Set mVDC mode for example.
$ ut181a-cli list-devices
Found DMM at path '0001:0004:00'.
$ ut181a-cli read once
Mode: mVDC [] [AUTO]
Range: -600...600 mV
1.74 mVDC
FAST: -104.61425 mVDC
```

Run `ut181a-cli help` to see other commands.

## License

This software licensed under the following:

* MIT License ([LICENSE](LICENSE) or https://opensource.org/license/MIT).
