// file: networking_task.rs
// desc: handle networking

use defmt::{info, error};
use embassy_sync::pipe::{Writer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::Timer;


#[embassy_executor::task]

pub async fn networking_task(
    mut wifi_controller: cyw43::Control<'static>,
    mut pipe_writer: Writer<'static, CriticalSectionRawMutex, 1>,
){

    // Connect to wifi

    loop{
        // listen for requests
    }
}