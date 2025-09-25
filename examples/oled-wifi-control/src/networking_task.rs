// file: networking_task.rs
// desc: handle networking

use defmt::{info, warn};
use core::str::from_utf8;

use embassy_sync::pipe::{Writer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_net::tcp::TcpSocket;
use cyw43::JoinOptions;
use embassy_time::{Duration, Timer};

use crate::setup_devices::WifiStack;

// Source from env variables WIFI_ID, WIFI_PASS
const WIFI_NETWORK: &str = env!("WIFI_ID");
const WIFI_PASSWORD: &str = env!("WIFI_PASS");

#[embassy_executor::task]
pub async fn networking_task(
    mut wifi_stack: WifiStack,
    pipe_writer: Writer<'static, CriticalSectionRawMutex, 1>,
) {
    info!("Starting networking task...");
    
    // Connect to WiFi
    connect_wifi(&mut wifi_stack).await;
    
    // HTTP server loop - inline
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];
    let mut request_buffer = [0; 512];

    loop {
        // Try dereferencing the stack
        let mut socket = TcpSocket::new(*wifi_stack.stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        info!("Listening for HTTP connections on port 80...");
        
        if let Err(e) = socket.accept(80).await {
            warn!("Socket accept error: {:?}", e);
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }

        info!("New HTTP connection from {:?}", socket.remote_endpoint());

        // Handle request inline
        if let Ok(bytes_read) = socket.read(&mut request_buffer).await {
            if bytes_read > 0 {
                let request = from_utf8(&request_buffer[..bytes_read]).unwrap_or("Invalid UTF-8");
                info!("HTTP Request: {=str}", &request[..request.len().min(100)]);

                // Parse command
                let command = parse_command(request);
                info!("Parsed command: {:?}", command);

                // Send command through pipe
                if let Some(cmd) = command {
                    // Quick inline blink for visual feedback
                    for _i in 0..cmd {
                        wifi_stack.wifi_controller.gpio_set(0, false).await;
                        Timer::after(Duration::from_millis(100)).await;
                        wifi_stack.wifi_controller.gpio_set(0, true).await;
                        Timer::after(Duration::from_millis(100)).await;
                    }

                    match pipe_writer.try_write(&[cmd]) {
                        Ok(_) => info!("Command {} sent to display task", cmd),
                        Err(_) => warn!("Failed to send command (pipe full?)"),
                    }
                }

                // Simple inline response - convert all to &[u8] slices
                let response: &[u8] = match command {
                    Some(1) => b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nAnimation 1 triggered!",
                    Some(2) => b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nAnimation 2 triggered!",
                    Some(3) => b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nAnimation 3 triggered!",
                    Some(4) => b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nAnimation 4 triggered!",
                    _ => b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n<h1>Pico 2W Control</h1><p><a href='/1'>Anim 1</a> | <a href='/2'>Anim 2</a> | <a href='/3'>Anim 3</a> | <a href='/4'>Anim 4</a></p>",
                };

                // Use write instead of write_all
                if let Err(e) = socket.write(response).await {
                    warn!("Write error: {:?}", e);
                }
            }
        }

        socket.close();
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn connect_wifi(wifi_stack: &mut WifiStack) {
    info!("Attempting to connect to WiFi network: {}", WIFI_NETWORK);
    
    loop {
        match wifi_stack.wifi_controller
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => {
                info!("WiFi connection successful!");
                break;
            }
            Err(err) => {
                warn!("WiFi join failed with status={}, retrying...", err.status);
                Timer::after(Duration::from_secs(5)).await;
            }
        }
    }

    info!("Waiting for link up...");
    wifi_stack.stack.wait_link_up().await;
    
    info!("Waiting for DHCP...");
    wifi_stack.stack.wait_config_up().await;
    
    if let Some(config) = wifi_stack.stack.config_v4() {
        info!("Network configured!");
        info!("IP Address: {}", config.address.address());
        info!("Gateway: {:?}", config.gateway);
        info!("HTTP Server ready at: http://{}", config.address.address());
    }


    // Turn on LED if connected
    wifi_stack.wifi_controller.gpio_set(0, true).await;
}

fn parse_command(request: &str) -> Option<u8> {
    // Look for GET /command?value=X
    if let Some(start) = request.find("GET /command?value=") {
        let value_start = start + "GET /command?value=".len();
        if let Some(end) = request[value_start..].find(|c: char| c == ' ' || c == '&' || c == '\r' || c == '\n') {
            let value_str = &request[value_start..value_start + end];
            if let Ok(value) = value_str.parse::<u8>() {
                if (1..=4).contains(&value) {
                    return Some(value);
                }
            }
        }
    }
    
    // Simple GET /1, /2, /3, /4 URLs
    if request.starts_with("GET /1 ") || request.starts_with("GET /1\r") { return Some(1); }
    if request.starts_with("GET /2 ") || request.starts_with("GET /2\r") { return Some(2); }
    if request.starts_with("GET /3 ") || request.starts_with("GET /3\r") { return Some(3); }
    if request.starts_with("GET /4 ") || request.starts_with("GET /4\r") { return Some(4); }
    
    None
}