1. Put the board into bootloader mode (boot button + reset button usually)
2. `cargo run` to flash (to the serial port at `/dev/tty.usbmodem01`)
3. Board resets (may need to fiddle with `.cargo/config.toml` runner arguments)
4. `defmt-print` pointing at `/dev/tty.usbmodemhighjeans1` to read defmt logs

Recording:

<a href="https://asciinema.org/a/y8s8AP3ucvYiWAVrzCt9nYxUO" target="_blank">
    <img src="https://asciinema.org/a/y8s8AP3ucvYiWAVrzCt9nYxUO.svg" />
</a>
