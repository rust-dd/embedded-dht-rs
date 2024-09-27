# DHT20

![steps](/docs/dht20_steps.png)

- SDA = Serial Data Line
- SCL = Serial Clock Line

##  Start the sensor

The initial step is to supply power to the sensor using the chosen VDD voltage, which can range from 2.2V to 5.5V. Once powered on, the sensor requires less than 100ms to stabilize (with SCL held high during this period) before entering the idle state, after which it is ready to accept commands from the host (MCU).

## Step 1

After powering on, wait at least 100ms. Before reading the temperature and humidity values, retrieve a status byte by sending 0x71. If the result of the status byte and 0x18 is not equal to 0x18, initialize the 0x1B, 0x1C, and 0x1E registers.

## Step 2
Wait for 10ms before sending the 0xAC command to trigger the measurement. The command consists of two bytes: the first byte is 0x33 and the second byte is 0x00.

![step4](/docs/dht20_step2.png)

## Step 3
Wait for 80ms for the measurement to complete. If Bit [7] of the status word is 0, the measurement is done, and you can proceed to read six bytes continuously; if not, continue waiting.

## Step 4
After receiving the six bytes, the following byte is the CRC check data, which can be read if needed. If the receiver requires CRC validation, an ACK is sent after the sixth byte is received; otherwise, send a NACK to terminate. The initial CRC value is 0xFF, and the CRC8 check uses the polynomial: CRC [7:0] = 1 + X^4 + X^5 + X^8.

![step4](/docs/dht20_step4.png)

## Step 5
Compute the temperature and humidity values.