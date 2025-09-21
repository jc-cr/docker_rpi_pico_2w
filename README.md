# docker_rpi_pico_2w

Docker dev env for Raspberry Pi Pico 2W with Embassy framework

## Setup

### Docker
Setup docker container:
```bash
cd .docker
docker compose build
```

### Rust project structure
- `pico-blink/` - Embassy-based blink example for Pico 2W
- `pico-blink/cyw43-firmware/` - CYW43 WiFi chip firmware files
- Uses embassy-rp with RP2350 support and cyw43 for onboard LED

## Flash to board
- Plug in board in boot mode (hold BOOTSEL)
- Flash:
```bash
cd .docker
docker compose run --rm flash
```

## Resources
- [Embassy Book](https://embassy.dev/book/)
- [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
- [Pico 2W Datasheet](https://datasheets.raspberrypi.com/pico/pico-2-w-datasheet.pdf)