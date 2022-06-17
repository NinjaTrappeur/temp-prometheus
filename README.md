# Prometheus Temperature Exporter

This small software reads some temperature/humidity data from a serial
USB device before exposing the data using the Prometheus file format.

Note: you'll have to use Nginx or another webserver to expose the prometheus file to the web.

## Usage

```sh
./temp-prometheus USB-DEV PROMETHEUS-FILE
```
