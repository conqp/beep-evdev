# beep-evdev

Beep the PC speaker using `evdev`.

# Prerequisites

The current user must be allowed to r/w to the speaker device:

```
$ cat /etc/udev/rules.d/70-pcspkr-beep.rules
ACTION=="add", SUBSYSTEM=="input", ATTRS{name}=="PC Speaker", ENV{DEVNAME}!="", GROUP="beep", MODE="0660"
```

# Examples:

```rust
use beep_evdev::{Beep, Note, DEFAULT_FILE};
use evdev::Device;

fn main() {
    Device::open(DEFAULT_FILE)
        .unwrap()
        .play([Note::default()])
        .unwrap()
}
```