# embedded-dht-rs

![steps](/docs/steps.png)

## Step 1

After powering on the DHT11 (once powered, allow 1 second to pass during which the sensor stabilizes; during this time, no commands should be sent), it measures the temperature and humidity of the surrounding environment and stores the data. Meanwhile, the DATA line of the DHT11 is kept high by a pull-up resistor. The DATA pin of the DHT11 is in input mode, ready to detect any external signals.

## Step 2

The microprocessor's I/O pin is set to output mode and pulled low, holding this state for at least 18 milliseconds. Then, the microprocessor's I/O is switched to input mode. Due to the pull-up resistor, the microprocessor’s I/O line and the DHT11 DATA line will remain high, waiting for the DHT11 to respond with a signal, as illustrated below:

![step2](/docs/step2.png)


## Step 3

The DHT11’s DATA pin detects an external signal and goes low, indicating that it is waiting for the external signal to complete. Once the signal ends, the DHT11’s DATA pin switches to output mode, producing a low signal for 80 microseconds as a response. This is followed by an 80-microsecond high signal, notifying the microprocessor that the sensor is ready to transmit data. At this point, the microprocessor's I/O pin, still in input mode, detects the low signal from the DHT11 (indicating the response) and then waits for the 80-microsecond high signal to start receiving data. The sequence of signal transmission is illustrated below:

![step3](/docs/step3.png)

## Step 4

The DHT11 outputs 40 bits of data through the DATA pin, and the microprocessor receives these 40 data bits. The format for a data bit "0" consists of a low level lasting 50 microseconds, followed by a high level lasting 26-28 microseconds, depending on changes in the I/O level. For a data bit "1," the format includes a low level of 50 microseconds followed by a high level lasting up to 70 microseconds. The signal formats for data bits "0" and "1" are shown below.

![step4](/docs/step4.png)

# End signal

After outputting a low signal for 50 microseconds, the DHT11 completes sending the 40 bits of data and switches the DATA pin back to input mode, which, along with the pull-up resistor, returns to a high state. Meanwhile, the DHT11 internally re-measures the environmental temperature and humidity, records the new data, and waits for the next external signal.


## Example Schematic 

![step3](/docs/example_esp32_dht11.png)
