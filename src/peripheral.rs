//Peripheral
#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{PIO0, USB};
use embassy_rp::usb::InterruptHandler;
use keymap::{COL, ROW};
use panic_probe as _;
use rmk::debounce::fast_debouncer::FastDebouncer;
use rmk::futures::future::join;
use rmk::matrix::direct_pin::DirectPinMatrix;
use rmk::run_all;
use rmk::split::SPLIT_MESSAGE_MAX_SIZE;
use rmk::split::peripheral::run_rmk_split_peripheral;
use rmk::split::rp::uart::{BufferedUart, UartInterruptHandler};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    PIO0_IRQ_0 => UartInterruptHandler<PIO0>;
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

    // Pin config
    // RP2350b core from Weact studio
    #[cfg(feature = "weact_core")]
    let direct_pins = [
        [
            Some(Input::new(p.PIN_26, Pull::Up)),
            Some(Input::new(p.PIN_27, Pull::Up)),
            Some(Input::new(p.PIN_28, Pull::Up)),
            Some(Input::new(p.PIN_25, Pull::Up)),
            None,
        ],
        [
            Some(Input::new(p.PIN_33, Pull::Up)),
            Some(Input::new(p.PIN_29, Pull::Up)),
            Some(Input::new(p.PIN_34, Pull::Up)),
            Some(Input::new(p.PIN_32, Pull::Up)),
            None,
        ],
        [
            Some(Input::new(p.PIN_14, Pull::Up)),
            Some(Input::new(p.PIN_16, Pull::Up)),
            Some(Input::new(p.PIN_15, Pull::Up)),
            Some(Input::new(p.PIN_17, Pull::Up)),
            None,
        ],
        [
            Some(Input::new(p.PIN_0, Pull::Up)),
            Some(Input::new(p.PIN_2, Pull::Up)),
            Some(Input::new(p.PIN_1, Pull::Up)),
            Some(Input::new(p.PIN_4, Pull::Up)),
            None,
        ],
        [
            Some(Input::new(p.PIN_7, Pull::Up)),
            Some(Input::new(p.PIN_8, Pull::Up)),
            Some(Input::new(p.PIN_10, Pull::Up)),
            Some(Input::new(p.PIN_5, Pull::Up)),
            None,
        ],
        [
            Some(Input::new(p.PIN_43, Pull::Up)),
            Some(Input::new(p.PIN_41, Pull::Up)),
            Some(Input::new(p.PIN_42, Pull::Up)),
            Some(Input::new(p.PIN_45, Pull::Up)),
            None,
        ],
        [
            Some(Input::new(p.PIN_20, Pull::Up)),
            Some(Input::new(p.PIN_35, Pull::Up)),
            Some(Input::new(p.PIN_36, Pull::Up)),
            Some(Input::new(p.PIN_37, Pull::Up)),
            None,
        ],
        [
            Some(Input::new(p.PIN_18, Pull::Up)),
            Some(Input::new(p.PIN_21, Pull::Up)),
            Some(Input::new(p.PIN_19, Pull::Up)),
            Some(Input::new(p.PIN_22, Pull::Up)),
            None,
        ],
    ];

    // Pin config
    // RP2350b mini from spotbear
    #[cfg(feature = "spotbear_mini")]
    let direct_pins = [
        [
            Some(Input::new(p.PIN_47, Pull::Up)),
            Some(Input::new(p.PIN_44, Pull::Up)),
            Some(Input::new(p.PIN_45, Pull::Up)),
            Some(Input::new(p.PIN_46, Pull::Up)),
            Some(Input::new(p.PIN_43, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_23, Pull::Up)),
            Some(Input::new(p.PIN_42, Pull::Up)),
            Some(Input::new(p.PIN_39, Pull::Up)),
            Some(Input::new(p.PIN_41, Pull::Up)),
            Some(Input::new(p.PIN_40, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_15, Pull::Up)),
            Some(Input::new(p.PIN_13, Pull::Up)),
            Some(Input::new(p.PIN_16, Pull::Up)),
            Some(Input::new(p.PIN_14, Pull::Up)),
            Some(Input::new(p.PIN_17, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_18, Pull::Up)),
            Some(Input::new(p.PIN_25, Pull::Up)),
            Some(Input::new(p.PIN_21, Pull::Up)),
            Some(Input::new(p.PIN_19, Pull::Up)),
            Some(Input::new(p.PIN_22, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_6, Pull::Up)),
            Some(Input::new(p.PIN_8, Pull::Up)),
            Some(Input::new(p.PIN_10, Pull::Up)),
            Some(Input::new(p.PIN_7, Pull::Up)),
            Some(Input::new(p.PIN_9, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_1, Pull::Up)),
            Some(Input::new(p.PIN_2, Pull::Up)),
            Some(Input::new(p.PIN_4, Pull::Up)),
            Some(Input::new(p.PIN_0, Pull::Up)),
            Some(Input::new(p.PIN_3, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_30, Pull::Up)),
            Some(Input::new(p.PIN_33, Pull::Up)),
            Some(Input::new(p.PIN_31, Pull::Up)),
            Some(Input::new(p.PIN_35, Pull::Up)),
            Some(Input::new(p.PIN_32, Pull::Up)),
        ],
        [
            Some(Input::new(p.PIN_11, Pull::Up)),
            Some(Input::new(p.PIN_29, Pull::Up)),
            Some(Input::new(p.PIN_12, Pull::Up)),
            Some(Input::new(p.PIN_5, Pull::Up)),
            Some(Input::new(p.PIN_28, Pull::Up)),
        ],
    ];

    // Pin config
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    // RP2350b core from Weact studio
    #[cfg(feature = "weact_core")]
    let uart = BufferedUart::new_full_duplex(p.PIO0, p.PIN_11, p.PIN_13, tx_buf, rx_buf, Irqs);

    // Pin config
    // RP2350b mini from spotbear
    #[cfg(feature = "spotbear_mini")]
    let uart = BufferedUart::new_full_duplex(p.PIO0, p.PIN_26, p.PIN_27, tx_buf, rx_buf, Irqs);

    // Initialize the matrix + keyboard
    let debouncer = FastDebouncer::new();
    let mut matrix = DirectPinMatrix::<_, _, ROW, COL, 40>::new(direct_pins, debouncer, true);

    // Start
    join(run_all!(matrix), run_rmk_split_peripheral(uart)).await;
}
