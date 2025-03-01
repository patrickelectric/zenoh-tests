# Rust Ping Pong

All tests running in a Raspberry Pi 5

| Protocol     | Frequency | Time per msg |
|--------------|-----------|--------------|
| Tokio        | 1.4 MHz   | 357 ns       |
| Zenoh async  | 330 kHz   | 1.5 µs       |
| Socket file  | 128 kHz   | 3.9 µs       |
| Zenoh sync   | 114 kHz   | 4.4 µs       |
| UDP          | 101 kHz   | 5.4 µs       |
