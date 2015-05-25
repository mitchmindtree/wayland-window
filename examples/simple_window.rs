extern crate byteorder;
extern crate wayland_client as wayland;
extern crate wayland_window;

use byteorder::{WriteBytesExt, NativeEndian};

use std::fs::OpenOptions;
use std::io::Write;

use wayland::core::default_display;
use wayland::core::ShmFormat;

use wayland_window::DecoratedSurface;

fn main() {

    assert!(wayland::is_wayland_lib_available(), "Wayland library could not be found.");

    let display = default_display().expect("Unable to connect to Wayland server.");

    let registry = display.get_registry();
    display.sync_roundtrip();

    let compositor = registry.get_compositor().expect("Unable to get the compositor.");

    // first, create the shell surface
    let surface = compositor.create_surface();
    let shell = registry.get_shell().expect("Unable to get the shell.");

    // then obtain a buffer to store contents
    let shm = registry.get_shm().expect("Unable to get the shm.");

    // Not a good way to create a shared buffer, but this will do for this example.
    let mut tmp = OpenOptions::new().read(true).write(true).create(true).truncate(true)
                            .open("shm.tmp").ok().expect("Unable to create a tempfile.");
    // write the contents to it, lets put everything in dark red
    for _ in 0..10_000 {
        let _ = tmp.write_u32::<NativeEndian>(0xFF880000);
    }
    let _ = tmp.flush();
    // create a shm_pool from this tempfile
    let pool = shm.pool_from_fd(&tmp, 40_000);
    // match a buffer on the part we wrote on
    let buffer = pool.create_buffer(0, 100, 100, 400, ShmFormat::WL_SHM_FORMAT_ARGB8888)
                     .expect("Could not create buffer.");

    let window = DecoratedSurface::new(surface, 100, 100, &registry);

    display.sync_roundtrip();

    loop {}
}