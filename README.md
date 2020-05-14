# Hello STM32F3

Playing around with STM32F3.

## HOWTO

```
# Terminal 1
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg

# Terminal 2
cd src
cargo run

# On device, click user button
# You should see "Hello world" being printed in terminal 1
```
