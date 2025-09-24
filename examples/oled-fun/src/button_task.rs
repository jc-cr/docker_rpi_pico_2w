// file: button_task
// desc: Handles button press updates

use embassy_time::{Duration, Timer};
use embassy_sync::pipe::{Writer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_rp::gpio::{Input};

use crate::setup_devices::Buttons;

async fn handle_button_1(
    mut button_1: &mut Input<'static>,
    pipe_writer:  Writer<'static, CriticalSectionRawMutex, 1>
){

        button_1.wait_for_high().await;  // Wait for button press
        let _ = pipe_writer.write(&[1]); // Write 1 since need non-zero
        button_1.wait_for_low().await;   // Wait for button release
        Timer::after(Duration::from_millis(50)).await;  // Debounce delay

}



#[embassy_executor::task]
pub async fn button_task(
    mut buttons: Buttons,
    pipe_writer:  Writer<'static, CriticalSectionRawMutex, 1>
){

    loop{
        handle_button_1(&mut buttons.button_1, pipe_writer).await;
        Timer::after(Duration::from_millis(5)).await;
    }

}