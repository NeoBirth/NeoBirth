# ![NeoBirth][logo]

[![Build Status][build-image]][build-link]
[![Apache 2.0 licensed][license-image]][license-link]
[![Gitter Chat][gitter-image]][gitter-link]

Acid house music program for the [Adafruit NeoTrellis M4], powered by the
[PureZen] music synthesis engine (a [Pure Data] engine targeting embedded
Rust devices).

Inspired by [Propellerhead ReBirth]. 

<img width="50%" src="https://cdn-learn.adafruit.com/guides/cropped_images/000/002/254/medium640/3963_top_lit_ORIG_2018_10.jpg?1541023640">

## Status

The current code produces a working executable which can be loaded onto a
NeoTrellis M4 device, however no functionality is yet in place (unless you
like blinking LEDs).

## Requirements

- NeoTrellis M4 Express device
- Rustup and Rust 1.31+: https://rustup.rs/ 
- `rustup target add thumbv7em-none-eabihf`
- [GNU ARM Embedded Toolchain]
- [BOSSA] (flash programming utility for Atmel's SAM microcntrollers)

## Compiling

```
$ CARGO_INCREMENTAL=0 cargo build --release
$ cd target/thumbv7em-none-eabihf/release/
$ arm-none-eabi-objcopy -O binary neobirth neobirth.bin
```

## Flashing the NeoTrellis M4

Press the `RESET` button on the back of the NeoTrellis M4 twice in rapid
succession. The LED on the back will turn *green*. 

Then run:

```
$ bossac -e -w -v -b -R -o 0x4000 --port=/dev/<your platform dev> neobirth.bin
```

Note that the `--port` flag for `bossac` varies by OS:

- Linux: `/dev/ttyACM0`
- macOS: `/dev/tty.usbmodemNNNNNN` (try `ls /dev/tty.usbmodem*`)

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

Copyright © 2018 NeoBirth Developers

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[logo]: https://raw.githubusercontent.com/NeoBirth/NeoBirth/master/logo.png
[build-image]: https://secure.travis-ci.org/NeoBirth/NeoBirth.svg?branch=master
[build-link]: https://travis-ci.org/NeoBirth/NeoBirth
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/NeoBirth/NeoBirth/blob/master/LICENSE
[gitter-image]: https://badges.gitter.im/NeoBirth/NeoBirth.svg
[gitter-link]: https://gitter.im/NeoBirth/NeoBirth
[Adafruit NeoTrellis M4]: https://learn.adafruit.com/adafruit-neotrellis-m4
[PureZen]: https://github.com/NeoBirth/PureZen
[Pure Data]: https://puredata.info/
[Propellerhead ReBirth]: https://en.wikipedia.org/wiki/ReBirth_RB-338
[GNU ARM Embedded Toolchain]: https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads
[BOSSA]: http://www.shumatech.com/web/products/bossa
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/NeoBirth/NeoBirth/blob/master/CODE_OF_CONDUCT.md
