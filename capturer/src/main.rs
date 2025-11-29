use pcap::{Capture, Device};

fn main() {
    let dev = Device::lookup()
        .expect("Erro no Device")
        .expect("Device Vazio");
    let mut cap = Capture::from_device(dev)
        .expect("Erro ao capturar o device")
        .open()
        .expect("Erro ao abrir o device");

    let mut count = 0;
    while let Ok(_) = cap.next_packet() {
        count += 1;
        println!("+1 packet total: {:?}", count);
        if count == 10 {
            break;
        }
    }
}
