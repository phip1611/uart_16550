//! UART discovery through legacy probing.
//!
//! The inventory merges every discovery path so one physical UART is tested
//! exactly once.

use crate::device::{Address, Inventory, Source};
use crate::raw_uart::RawUart;
use crate::uefi;

/// Combines every discovery source into a deduplicated test inventory.
pub fn discover() -> Inventory {
    let mut inventory = Inventory::default();
    discover_legacy(&mut inventory);
    inventory
}

/// Probes conventional COM addresses while always retaining COM1 as a baseline.
fn discover_legacy(inventory: &mut Inventory) {
    const PORTS: [u16; 4] = [0x3f8, 0x2f8, 0x3e8, 0x2e8];

    uefi::println!("\nLegacy UART probes:");
    for (index, port) in PORTS.into_iter().enumerate() {
        let address = Address::Port(port);
        let passed = RawUart::new(address).scratch_test();
        uefi::println!(
            "  {address}: scratch test {}",
            if passed { "PASS" } else { "FAIL" }
        );

        if index == 0 {
            inventory.add(address, None, Source::RequiredCom1);
        } else if passed {
            inventory.add(address, None, Source::LegacyProbe);
        }
    }
}
