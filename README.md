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

- `examples/` - Embassy-based examples
- `./.cargo/` - Dir for configuration file
- `./build.rs` - Build code
- `./memory.x` - Memory layout file

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