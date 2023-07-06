# Temperature reader for Raspberry Pi
Temperature reader periodically reads the RPi CPU temperature and publish it
as the Prometheus readable metric on http interface.

## Prerequisities
* It is expected that there is Raspberry Pi OS Lite running on the target machine
* It is possible to burn it onto SD card via [rpi-imager](https://github.com/raspberrypi/rpi-imager)	
* There must be SSH access enabled on the target

## Find the target IP
If the target RPi is connected to the same network as your host, you can find the RPi's
IP with the following command.
```
nmap -p 22 --open -oG - $(hostname -I | cut -f1 -d ' ')/24
```
* `-p 22`						Only scans specified port 22 (SSH)
* `--open`					Only shows open ports
* `-oG`							Output scan in grepable format
* `hostname -I`			Shows all addreses for the host
* `cut -f1 -d ' '`	Cuts the first IP address

## Add key based authentication
Generate new key pair for RPi target.
```
ssh-keygen -t rsa -f ~/.ssh/rpi_rsa
```
Add your public key to the allowed keys on host.
```
ssh-copy-id -i ~/.ssh/rpi_rsa.pub pi@PI_IP_ADDRESS
```
Now you can log to the RPi without password.

## Cross compilation

```
rustup target add arm-unknown-linux-gnueabihf
```

```
apt install -y gcc
```

```
https://github.com/raspberrypi/tools.git
```
