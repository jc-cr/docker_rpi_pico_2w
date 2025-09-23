// file: button_task
// desc: Handles button press updates

use crate::setup_devices::Buttons;
use embassy_sync::pipe::{Writer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

#[embassy_executor::task]
pub async fn button_task(
    mut buttons: Buttons,
    mut pipe_writer:  Writer<'static, CriticalSectionRawMutex, 1>
){

}