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


async fn handle_button_2(
    mut button_2: &mut Input<'static>,
    pipe_writer:  Writer<'static, CriticalSectionRawMutex, 1>
){

        button_2.wait_for_high().await;  // Wait for button press
        let _ = pipe_writer.write(&[2]); // Write 2 since need non-zero
        button_2.wait_for_low().await;   // Wait for button release
        Timer::after(Duration::from_millis(50)).await;  // Debounce delay

}


async fn handle_button_3(
    mut button_3: &mut Input<'static>,
    pipe_writer:  Writer<'static, CriticalSectionRawMutex, 1>
){

        button_3.wait_for_high().await;  // Wait for button press
        let _ = pipe_writer.write(&[3]); // Write 3 since need non-zero
        button_3.wait_for_low().await;   // Wait for button release
        Timer::after(Duration::from_millis(50)).await;  // Debounce delay

}


async fn handle_button_4(
    mut button_4: &mut Input<'static>,
    pipe_writer:  Writer<'static, CriticalSectionRawMutex, 1>
){

        button_4.wait_for_high().await;  // Wait for button press
        let _ = pipe_writer.write(&[4]); // Write 4 since need non-zero
        button_4.wait_for_low().await;   // Wait for button release
        Timer::after(Duration::from_millis(50)).await;  // Debounce delay

}





#[embassy_executor::task]
pub async fn button_task(
    mut buttons: Buttons,
    pipe_writer:  Writer<'static, CriticalSectionRawMutex, 1>
){

    loop{
        handle_button_1(&mut buttons.button_1, pipe_writer).await;

        handle_button_2(&mut buttons.button_2, pipe_writer).await;

        handle_button_3(&mut buttons.button_3, pipe_writer).await;

        handle_button_4(&mut buttons.button_4, pipe_writer).await;
        
        Timer::after(Duration::from_millis(5)).await;
    }

}