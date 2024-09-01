use std::thread::sleep;

use hmac::Mac;
use opencv::{
    core::Mat,
    videoio::{VideoCapture, VideoCaptureTrait, CAP_ANY},
    wechat_qrcode::{WeChatQRCode, WeChatQRCodeTrait},
};
use qr_common::{create_hmac, get_secret_from_env};
use tungstenite::{connect, Message};

fn main() {
    // Connect to the ZWave JS server over WebSocket
    let (mut socket, response) =
        connect("wss://zwave-js-ui.home.derham.me").expect("Can't connect");
    println!("Connected to the ZWave JS server");
    println!("Response HTTP code: {}", response.status());

    // Connect to the camera stream
    let mut video = VideoCapture::new(1, CAP_ANY).unwrap();
    let mut frame = Mat::default();
    println!("Connected to the camera stream");

    // Create a QR code detector
    let mut detector = WeChatQRCode::new_def().unwrap();

    loop {
        // Read the frame
        video.read(&mut frame).unwrap();

        // Detect the QR code
        let qr_code_info = detector.detect_and_decode_def(&frame).unwrap();
        if qr_code_info.is_empty() {
            continue;
        }

        // Parse the QR code data
        let qr_code_data = qr_code_info.get(0).unwrap();
        let mut parts = qr_code_data.split(':');
        let (hmac, user, expiry) = (
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        );

        // Verify the HMAC
        let payload = format!("{}:{}", user, expiry);
        let secret = get_secret_from_env();
        let mut mac = create_hmac(&secret);
        mac.update(payload.as_bytes());
        match mac.verify_slice(&base85::decode(hmac).unwrap()) {
            Ok(_) => println!("Verified"),
            Err(_) => {
                println!("Verification failed");
                continue;
            }
        }

        // Verify the expiry
        let expiry = expiry.parse::<u64>().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if now > expiry {
            println!("Expired ({})", expiry);
            continue;
        }

        // Open the door
        socket
            .send(Message::Text(
                r##"{
                    "messageId": 1,
                   	"command": "node.set_value",
                   	"nodeId": 4,
                   	"valueId": {
                     	"commandClass": 98,
                  		"property": "targetMode"
                   	},
                   	"value": 0
                }"##
                .into(),
            ))
            .unwrap();

        // Wait for the door to open
        sleep(std::time::Duration::from_secs(10));
    }
}
