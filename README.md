# pixelring
 Rust interface for the Pixel Ring ReSpeaker USB 4 Mic Array LED Control Protocol

## Requirements

* rust
* rusb https://github.com/a1ien/rusb

## Respeaker

### Udev config under linux

Access to respeaker for everybody

/etc/udev/rules.d/60-respeaker.rules
```
SUBSYSTEM=="usb", ATTR{idProduct}=="0018", ATTR{idVendor}=="2886", MODE:="0666"
```

```shell
sudo udevadm control --reload
```

Unplug and plug the device.

## More information

* https://github.com/respeaker/pixel_ring/wiki/ReSpeaker-USB-4-Mic-Array-LED-Control-Protocol
* https://github.com/respeaker/pixel_ring/blob/master/pixel_ring/usb_pixel_ring_v2.py
* https://github.com/pyusb/pyusb/
* https://crates.io/crates/rusb
* https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/

## Author

Copyright (c) 2023 David Rasch, license under the GPL-V3
