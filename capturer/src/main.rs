use pcap::{Capture, Device};
use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    let dev = Device::lookup()
        .expect("Erro no Device")
        .expect("Device Vazio");

    let mut cap = Capture::from_device(dev)
        .expect("Erro ao capturar o device")
        .open()
        .expect("Erro ao abrir o device");

    let mut count = 0;

    let mut packets = Vec::new();

    while let Ok(packet) = cap.next_packet() {
        count += 1;

        println!("+1 packet total: {:?}", count);

        packets.push(packet.len());
        if count == 10 {
            break;
        }
    }

    let client = Client::new();
    let body = json!(packets);
    let response = client
        .post("http://192.168.0.134:8000/packets")
        .json(&body)
        .send();
    println!("Enviado com sucesso: {:#?}", packets);
    println!(
        "{:?}",
        response
            .expect("Erro ao extrair o response")
            .text()
            .expect("Erro ao receber a messagem do servidor")
    );
}
