//Central
#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;
mod vial;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::flash::{Async, Flash};
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0, USB};
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, dma};
use keymap::{COL, ROW};
use panic_probe as _;
use rmk::config::{BehaviorConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig};
use rmk::debounce::fast_debouncer::FastDebouncer;
use rmk::futures::future::join;
use rmk::host::HostService;
use rmk::keyboard::Keyboard;
use rmk::matrix::direct_pin::DirectPinMatrix;
use rmk::processor::builtin::wpm::WpmProcessor;
use rmk::split::SPLIT_MESSAGE_MAX_SIZE;
use rmk::split::central::run_peripheral_manager;
use rmk::split::rp::uart::{BufferedUart, UartInterruptHandler};
use rmk::usb::UsbTransport;
use rmk::{KeymapData, initialize_keymap_and_storage, run_all};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    PIO0_IRQ_0 => UartInterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Keyboard K1012"),
    embassy_rp::binary_info::rp_program_description!(c"Keybord for 5 way switch using RMK"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Create the usb driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    // Pin config
    let direct_pins = [
        [
            Some(Input::new(p.PIN_10, Pull::Up)),
            Some(Input::new(p.PIN_8, Pull::Up)),
            Some(Input::new(p.PIN_9, Pull::Up)),
            Some(Input::new(p.PIN_6, Pull::Up)),
            Some(Input::new(p.PIN_7, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_18, Pull::Up)),
            Some(Input::new(p.PIN_21, Pull::Up)),
            Some(Input::new(p.PIN_19, Pull::Up)),
            Some(Input::new(p.PIN_11, Pull::Up)),
            Some(Input::new(p.PIN_12, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_33, Pull::Up)),
            Some(Input::new(p.PIN_37, Pull::Up)),
            Some(Input::new(p.PIN_35, Pull::Up)),
            Some(Input::new(p.PIN_34, Pull::Up)),
            Some(Input::new(p.PIN_36, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_32, Pull::Up)),
            Some(Input::new(p.PIN_29, Pull::Up)),
            Some(Input::new(p.PIN_31, Pull::Up)),
            Some(Input::new(p.PIN_28, Pull::Up)),
            Some(Input::new(p.PIN_30, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_45, Pull::Up)),
            Some(Input::new(p.PIN_41, Pull::Up)),
            Some(Input::new(p.PIN_47, Pull::Up)),
            Some(Input::new(p.PIN_43, Pull::Up)),
            Some(Input::new(p.PIN_46, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_0, Pull::Up)),
            Some(Input::new(p.PIN_1, Pull::Up)),
            Some(Input::new(p.PIN_3, Pull::Up)),
            Some(Input::new(p.PIN_5, Pull::Up)),
            Some(Input::new(p.PIN_2, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_13, Pull::Up)),
            Some(Input::new(p.PIN_16, Pull::Up)),
            Some(Input::new(p.PIN_14, Pull::Up)),
            Some(Input::new(p.PIN_17, Pull::Up)),
            Some(Input::new(p.PIN_15, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_38, Pull::Up)),
            Some(Input::new(p.PIN_25, Pull::Up)),
            Some(Input::new(p.PIN_22, Pull::Up)),
            Some(Input::new(p.PIN_23, Pull::Up)),
            Some(Input::new(p.PIN_24, Pull::Up)),
        ],
    ];

    // Use internal flash to emulate eeprom
    let flash = Flash::<_, Async, FLASH_SIZE>::new(p.FLASH, p.DMA_CH0, Irqs);

    let keyboard_device_config = DeviceConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "Verdea",
        product_name: "Keyboard Mouse combo Mehra K1012",
        ..DeviceConfig::default()
    };

    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[(0, 0), (1, 1)]);

    let rmk_config = RmkConfig {
        device_config: keyboard_device_config,
        vial_config,
        ..Default::default()
    };

    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart = BufferedUart::new_full_duplex(p.PIO0, p.PIN_27, p.PIN_26, tx_buf, rx_buf, Irqs);

    // Initialize the storage and keymap
    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
    let mut behavior_config = BehaviorConfig::default();
    let storage_config = StorageConfig::default();
    let per_key_config = PositionalConfig::default();
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &storage_config,
        &mut behavior_config,
        &per_key_config,
    )
    .await;

    // Initialize the matrix + keyboard
    let debouncer = FastDebouncer::new();
    //let mut matrix = DirectPinMatrix::<_, _, 9, 4, 36>::new(direct_pins, debouncer, true);
    let mut matrix = DirectPinMatrix::<_, _, ROW, COL, 40>::new(direct_pins, debouncer, true);
    let mut keyboard = Keyboard::new(&keymap);
    let host_ctx = rmk::host::KeyboardContext::new(&keymap);
    let mut host_service = HostService::new(&host_ctx, &rmk_config);

    let mut usb_transport = UsbTransport::new(driver, rmk_config.device_config);
    let mut wpm_processor = WpmProcessor::new();

    // Start
    join(
        run_all!(matrix, storage, usb_transport, wpm_processor, keyboard, host_service),
        run_peripheral_manager::<ROW, COL, 8, 0, _>(0, uart),
    )
    .await;
}
