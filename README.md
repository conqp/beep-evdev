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
extern crate beep_evdev;

use beep_evdev::beep;
use std::{thread, time};

fn main() {
    beep(440).expect("could not beep");
    thread::sleep(time::Duration::from_millis(500));
    beep(880).expect("could not beep");
    thread::sleep(time::Duration::from_millis(500));
    beep(0).expect("could not beep");
}
```