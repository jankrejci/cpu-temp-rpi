# Temperature reader for Raspberry Pi
Temperature reader periodically reads the RPi CPU temperature and publish it
as the Prometheus readable metric on http interface.

## Prerequisities
* It is expected that there is Raspberry Pi OS Lite 64 bit is running on the target machine
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
Add key to ssh agent
```
ssh-add ~/.ssh/rpi_rsa
```
Now you can log to the RPi without password.

## Cross compilation
As the Raspberry Pi runs on the different environment than your host machine,
it is needed to cross compile the binary. One way to do it is to use docker
image with the correct environmanet installed and build the binary inside
the container. First you need to install docker and then, inside the project
folder tun the following command. 
```
docker build . -t rpi-cross-compile-image
```
It builds the docker image from the `Dockerfile` supplied in the project root.
Then there is script `cross.sh`, which runs the docker image and executes commands
supplied to it. So you can build the binary inside container.
```
./cross.sh cargo build
```
And then run the binary on the remote target.
```
./cross.sh cargo run pi@PI_IP_ADDRESS
```
Tere is another script `run.sh` used as a custom cargo runner. It copies and run the
binary on the raspberry over SSH.

## Test
To test the temperature measuring you can run stress test on the target machine.
First you need to install `stress` package.
```
sudo apt install stress
```
And the run the command. It spawns 4 processes for 600 seconds. It calculate
square root of some garbage, so it loads CPU to 100%.
```
stress -c 4 -t 600
```
You can see the temperature rising on the web page.
```
http://PI_IP_ADDRESS:8081/metrics
```
You can also run prometheus and grafana on your host machine and create a beautiful
temperature chart :) 