fn main() {
// to show all input devices 

// ls /dev/input 
// or 
// ls /dev/input/by-path

// then try one after the other 
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.2.1:1.0-event-joystick
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.2.1:1.0-joystick
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.2.1:1.1-event
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.2.1:1.2-event-kbd
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.2.3:1.0-event-mouse
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.2.3:1.0-mouse
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.2.3:1.1-event-kbd
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.3.3:1.0-event
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:1.3.4:1.3-event
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:3:1.0-event
// sudo cat /dev/input/by-path/pci-0000:04:00.3-usb-0:3:1.2-event
// sudo cat /dev/input/by-path/platform-i8042-serio-0-event-kbd
// sudo cat /dev/input/by-path/platform-i8042-serio-1-event-mouse
// sudo cat /dev/input/by-path/platform-i8042-serio-1-mouse
// sudo cat /dev/input/by-path/platform-thinkpad_acpi-event

// from stackoverflow 
// According to the Linux input documentation, section 5, the /dev/input/eventX devices return data as following:

struct O_input_event {
    struct timeval time;
    unsigned short type;
    unsigned short code;
    unsigned int value; };
}
