Room temperature monitoring
Shared by Moto

This is a copy of a chat between Claude and Moto. Content may include unverified or unsafe content that do not represent the views of Anthropic. Shared snapshot may contain attachments and data not displayed here.

Current, 24 hour max, 24 hour min in degrees centigrade
In my room 
28.9, 28.9, 26.4
Check current location
Done

No location access, but that's fine since I don't need it for this — you've already given me the readings.

Your room right now:

    Current: 28.9°C
    24hr max: 28.9°C (currently at the peak)
    24hr min: 26.4°C

That's a fairly narrow swing (2.5°C) but sitting at the high end all day — 28.9°C indoors is warm enough to be uncomfortable for sustained periods, especially with Hampton Roads humidity this time of year. If you don't have AC running in there, it's worth checking airflow or a fan setup.

Ok I allowed access now
Check current location
Done

Got it, that's set now for Newport News. Anything you want me to check with it — outdoor weather to compare against your room readings, or something else?

Yes, good idea check my room against outside? How?
Searched the web
Newport News, VA 10-Day Weather Forecast | Weather Underground
www.wunderground.com
Newport News, VA Weather Forecast | AccuWeather
www.accuweather.com
Newport News, VA Current Weather | AccuWeather
www.accuweather.com
Newport News, VA Weather Forecast, Conditions, and Maps – Yahoo Weather
weather.yahoo.com
10-Day Weather Forecast for Newport News, Virginia - The Weather Channel | weather.com
weather.com
Newport News VA
forecast.weather.gov
Newport News, VA Weather Forecast | Local Weather Updates for Virginia by FOX Weather
www.foxweather.com
Newport News, VA Current Weather Today and Forecast with Radar | LocalConditions.com
www.localconditions.com
Zone Area Forecast for Newport News
forecast.weather.gov
Newport News, Virginia, USA 14 day weather forecast
www.timeanddate.com
Done

Right now outside is 87°F (about 30.6°C), with thunderstorms and 60% humidity — feels closer to 90°F with the humidity factored in.

So compared to your room:
	Outside	Your room
Current	~30.6°C (87°F)	28.9°C
Today's high	likely mid-90s°F (~34-35°C)	28.9°C

Your room's actually running a few degrees cooler than outside right now, which is good — whatever setup you've got (fan, shade, insulation) is doing real work. But 28.9°C indoors is still on the warm side for comfort, especially with today's humid conditions and highs in the lower 90s pushing heat index values up. If you don't have AC in there, it's more a "not terrible" situation than a "comfortable" one.

Inkbird ith 13 b reading on my desk is 28.9 smoke detector close to the floor by the door is 26 (does not have accuracy precision to show decimals so take it with a grain of salt)

That's a meaningful stratification — nearly 3°C difference between floor-level and desk height in the same room. Warm air rises, so this is exactly the pattern you'd expect: the smoke detector near the floor by the door is reading cooler air, while your desk (presumably higher up, maybe away from the door draft) is sitting in the warmer layer.

A few things that could be driving it:

    Height alone — even without any airflow, a 3-4 foot vertical gap can easily produce 2-3°C of stratification in an unconditioned or lightly conditioned room
    Door proximity — if there's any air movement from outside the room (hallway, another room, exterior door) at floor level, that's pulling cooler air in near the detector
    Desk position — is it against a wall that gets sun exposure, or near a window? That would explain a localized hot spot

Practically: if you're trying to figure out what temperature to "trust" for whether the room needs more cooling, the desk reading is probably more representative of where you actually sit and feel it. The floor reading is closer to what a thermostat mounted low would see, which tends to undercall how warm a room actually feels to a person sitting or standing.

Desk is at one corner of the room with computers multiple laptops and monitor running automated tests and near the door it is easier to get outside air which is cooler

That confirms it — you've basically got a self-inflicted heat source at the desk. Multiple laptops, a monitor, and whatever's running the automated tests are all dumping watts as heat right into that corner, on top of it being spatially higher (rises) and further from the outside-air path.

So the 2.9°C gap isn't really "room temperature" varying — it's two different microclimates:

    Desk corner: ambient + equipment heat load + rising warm air, poor mixing with fresh air
    Door area: getting some benefit from cooler outside air infiltration, floor-level, no local heat sources

A couple of practical implications:

    If comfort at the desk is the goal, a small fan aimed to break up that local heat pocket (even just to mix air rather than blow directly on you) would probably help more than anything applied to the whole room, since the problem is localized
    If those automated tests can be scheduled overnight or staggered, that's the actual lever on the heat load — everything else is just managing symptoms
    The Inkbird reading is telling you the truth about your workspace specifically, not the room in general — worth keeping in mind if you're using it to decide when to run AC or open windows, since the door sensor might say "fine" while your desk is still uncomfortable

I opened the door now so it will get better I hope

Makes sense — that gives the warm desk-corner air somewhere to go and lets the cooler outside-adjacent air actually mix in instead of just pooling near the floor by the door.

Worth a quick check-in on the Inkbird in 20-30 min to see how much it actually pulled the desk reading down. If it barely moves, that's a sign the equipment heat load is outpacing what passive airflow can clear, and you'd need something more active (a fan, or scheduling the tests differently) rather than just the door.

The inkbird supposedly supports Bluetooth. Is there any free and open source project that gets data off of it and on my fedora laptop with Acer Swift go 14 without destroying the aaa batteries on the inkbird? I want the batteries in the inkbird to last as long as possible
Searched the web
Read temperature and humidity data from Inkbird ibs-TH1. · GitHub
gist.github.com
GitHub - tobievii/inkbird: ble devices to mqtt
github.com
For IBS-TH1 can data be extracted to PC? - Questions & Answers - INKBIRD Community
community.inkbird.com
inkbird · GitHub Topics · GitHub
github.com
INKBIRD Wireless Hygrometer Thermometer Data Logger IBS-TH1
www.inkbird.com
[New Sensor]: Inkbird IBS-TH1 Plus · Issue #1420 · custom-components/ble_monitor
github.com
Add support for Inkbird IBS-TH1 Mini sensor · Issue #60 · sputnikdev/eclipse-smarthome-bluetooth-binding
github.com
GitHub - viyh/inkbird: Script for interacting over bluetooth with an Inkbird IBS-TH1 temperature probe. · GitHub
github.com
INKBIRD IBS-P01B - Swimming pool temperature sensor BLE · Issue #3180 · esphome/feature-requests
github.com
INKBIRD Bluetooth Temperature and Humidity Thermometer IBS-TH1 Plus
www.inkbird.com
Done

Good news — yes, there are FOSS options, and the good/bad news on battery is worth being precise about since your goal is battery longevity.

The key fact for your battery question: the IBS-TH1 broadcasts its readings as BLE advertisement packets — the same passive broadcast method used by things like Xiaomi Mijia sensors. This means it's already transmitting periodically whether or not anything is listening. A passive scanner on your laptop just listens to broadcasts in the air; it doesn't wake the device, doesn't establish a connection, and draws essentially nothing extra from the sensor's battery beyond what it's already spending to broadcast.

The battery drain risk comes from active GATT connections (like some Home Assistant integrations that connect+poll rather than just listen for adverts). That does increase Inkbird-side power draw because the device has to maintain a connection.

For your Fedora + Acer Swift Go 14 setup, the best FOSS options:

    ble_monitor (Home Assistant custom component) — passive-listening BLE monitor originally for Xiaomi/Mijia-style sensors, has some Inkbird support tracked in issues. Overkill if you don't already run Home Assistant.
    Plain passive scanning with bluetoothctl / bleak (Python) — simplest, no daemon needed. A short Python script using the bleak library can passively scan and just log manufacturer_data from advertisements, parsing temp/humidity yourself (the community gists above show the byte layout). This is the lowest-overhead option and fits your "review the code, don't blindly trust agentic tools" preference — it's maybe 40-60 lines, fully inspectable.
    viyh/inkbird — an existing script for this exact use case, though it was built targeting AWS CloudWatch; you'd strip that part out and just keep the BLE read logic.

Given your GoTunnels/self-hosted stack, I'd guess you'd want option 2: a small passive-scan Python script feeding into whatever you're already using for observability (Uptrace?), rather than pulling in Home Assistant just for one sensor. Want me to sketch that script?

Turn on web search in Search and tools menu. Otherwise, links provided may not be accurate or up to date.

No, please write this in plain English first

Here's the plain-English version:

Your battery question, answered directly: listening to the Inkbird from your laptop won't meaningfully drain its AAA batteries, as long as you use the right method.

Here's why. The IBS-TH1 doesn't wait for something to ask it for data — it just continuously broadcasts its temperature/humidity readings into the air every so often, like a radio station. It does this whether or not anyone's listening, because that's how the official phone app gets readings without needing a stable connection.

There are two ways a computer can get that data:

    Just listen (passive scanning) — your laptop's Bluetooth radio quietly picks up the broadcasts the Inkbird is already sending out. The Inkbird has no idea your laptop is even there. Zero extra battery cost to the sensor.
    Connect to it (active connection) — your laptop actively pairs/connects to the Inkbird and asks it questions. This forces the Inkbird to do extra work to maintain that connection, which does cost it battery. Some smart-home integrations (like certain Home Assistant setups) work this way.

So the answer is: use a tool that does option 1, not option 2, and your batteries will last basically as long as they would if you never touched it from your laptop at all.

There are free, open-source projects that do this — some are small scripts other people wrote specifically for this Inkbird model, and there's also a way to write a short custom script yourself using a Python library called bleak, which just sits and listens for those broadcasts and decodes the numbers.

Want me to go through the actual options and how you'd set one up on your Fedora machine?
Files hidden in shared chats

It is ITH 13 B even smart is misspelled

Good catch on the exact model — that matters, because "ITH-13-B" is a different device than the "IBS-TH1" I searched for earlier. Let me look this one up specifically.
Searched the web
INKBIRD Wireless Hygrometer Thermometer Data Logger ITH-13-B
www.inkbird.com
GitHub - Bluetooth-Devices/inkbird-ble: Parser for INKBIRD BLE devices · GitHub
github.com
INKBIRD - Home Assistant
www.home-assistant.io
Releases · Bluetooth-Devices/inkbird-ble
github.com
INKBIRD ITH-13-B Bluetooth Thermometer Hygrometer: Setup, Features & App Control
manuals.plus
Wi-Fi and Bluetooth Sensors – INKBIRD
www.inkbird.com
Inkbird: Add support for ITH-11-B BLE thermometer hygrometer - Feature Requests - Home Assistant Community
community.home-assistant.io
ibbq-thermometer-protocol · GitHub
gist.github.com
inkbird-ble · PyPI
pypi.org
ITH-13-B Manual – INKBIRD
www.inkbird.com
Done

Good news — your exact model is directly and well supported.

There's a proper open-source Python library called inkbird-ble (on GitHub under Bluetooth-Devices/inkbird-ble, also on PyPI), and it's the actual parser used by Home Assistant's built-in INKBIRD integration. Your specific model, the ITH-13-B, is explicitly listed as supported.

On the battery question specifically: the INKBIRD integration will automatically discover devices once Bluetooth is enabled — most devices work this way, though the battery-level sensor specifically requires an "active scan" to update. That's an important nuance for you:

    Temperature and humidity readings come through passive listening (broadcasts), so reading those has no real battery cost.
    The battery percentage reading itself apparently needs an active scan/connection to refresh — so if you want your Fedora tool to also report "battery: 73%," that specific piece would cost a little extra power. If you only care about temp/humidity, you can skip that and stay fully passive.

Practically, for your Fedora Acer Swift Go 14, this means:

    pip install inkbird-ble gives you the parsing logic
    Pair it with bleak (which inkbird-ble already depends on) to do passive BLE scanning
    You read temp/humidity continuously with zero meaningful battery impact, and just skip battery-level polling if you want to be maximally conservative

Want me to sketch a short script using inkbird-ble + bleak that just passively logs temp/humidity to your terminal or a file?

Yes please fully passive and once we have the data we can do whatever we want on the laptop
Searched the web
Releases · Bluetooth-Devices/inkbird-ble
github.com
GitHub - hbldh/bleak: A cross platform Bluetooth Low Energy Client for Python using asyncio · GitHub
github.com
GitHub - Bluetooth-Devices/inkbird-ble: Parser for INKBIRD BLE devices · GitHub
github.com
GitHub - satrik/esp32_ble_inkbird: Inkbird BLE BBQ Thermometer Webserver with some nice features
github.com
GitHub - tobievii/inkbird: ble devices to mqtt
github.com
BleakScanner class — bleak 3.0.1 documentation
bleak.readthedocs.io
Bleak | mbedded.ninja
blog.mbedded.ninja
GitHub - runningtoy/InkBird_BLE2MQTT: ibbq Inkbird ble to mqtt bridge ESP32 based
github.com
bleak Documentation Release 3.0.1 Henrik Blidh Mar 25, 2026
bleak.readthedocs.io
bleak/bleak/backends/scanner.py at develop · hbldh/bleak
github.com
Done

Let me check the actual inkbird-ble source to get the correct API before writing anything, so the script is accurate rather than guessed.
Searched the web
Done

Now I have the real API. The library expects a service_info object shaped like habluetooth's BluetoothServiceInfoBleak, which normally comes from Home Assistant. Since you're not running HA, I need to build that object myself from raw bleak scan data. Let me write this carefully.
Ran a command
Done

No network access in this sandbox to actually install and test it — that's fine, I'll write the script from the documented API and be upfront about what's verified vs. what you'll need to confirm on your own machine (network is disabled here per your bash tool config anyway, separate from your actual laptop).
Created a file, read a file
Done

To run it on your Fedora machine:
bash

pip install bleak inkbird-ble --break-system-packages
python3 inkbird_passive_log.py

What it does:

    Listens for BLE advertisement broadcasts only — never opens a connection to the sensor
    Uses inkbird_ble's real parser (INKBIRDBluetoothDeviceData), which is the same code Home Assistant uses under the hood
    Prints temp/humidity/battery live to the terminal
    Appends every reading to ~/inkbird_readings.csv, so from there you can do whatever you want with the data (feed it into Uptrace, chart it, compare against the door sensor, etc.)

Things worth knowing before you run it:

    I don't have network access in this sandbox, so I could not actually install bleak/inkbird-ble and test this against a live device — the code follows the documented API exactly, but you're the first real test of it.
    The BluetoothServiceInfoBleak object comes from the habluetooth package (a dependency pulled in indirectly by inkbird-ble/Home Assistant's Bluetooth stack). If pip install inkbird-ble doesn't pull it in automatically, run pip install habluetooth --break-system-packages too.
    battery will likely come back as None since — per the earlier research — battery level needs an active scan/connection for some models. That's expected and fine given your priority.
    If BlueZ on Fedora doesn't honor scanning_mode="passive" (support varies by adapter/BlueZ version), it may silently fall back to active-mode BLE scanning at the OS level. That's still not the same as connecting to the sensor — the sensor still doesn't know or care your laptop exists — so it doesn't affect the Inkbird's battery either way.

Try it and paste back any errors — first run will probably need one or two dependency tweaks.

```python
#!/usr/bin/env python3
"""
Passive BLE listener for the INKBIRD ITH-13-B thermo-hygrometer.

This does NOT connect to the device. It only listens to the advertisement
packets the ITH-13-B is already broadcasting on its own (the same packets
the phone app uses). No connection = no extra battery drain on the sensor.

Requires:
    pip install bleak inkbird-ble --break-system-packages

Usage:
    python3 inkbird_passive_log.py
"""

import asyncio
import csv
import time
from datetime import datetime, timezone
from pathlib import Path

from bleak import BleakScanner
from bleak.backends.device import BLEDevice
from bleak.backends.scanner import AdvertisementData
from habluetooth import BluetoothServiceInfoBleak
from inkbird_ble import INKBIRDBluetoothDeviceData

LOG_FILE = Path.home() / "inkbird_readings.csv"

# If you know your sensor's MAC address, put it here to only log that one
# device. Leave as None to log any INKBIRD device bleak detects.
TARGET_ADDRESS = None  # e.g. "AA:BB:CC:DD:EE:FF"


def ensure_log_header():
    if not LOG_FILE.exists():
        with LOG_FILE.open("w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["timestamp_utc", "address", "temperature_c", "humidity_pct", "battery_pct"])


def append_row(address: str, temperature, humidity, battery):
    with LOG_FILE.open("a", newline="") as f:
        writer = csv.writer(f)
        writer.writerow([
            datetime.now(timezone.utc).isoformat(),
            address,
            temperature,
            humidity,
            battery,
        ])


def build_service_info(device: BLEDevice, adv: AdvertisementData) -> BluetoothServiceInfoBleak:
    """Wrap raw bleak advertisement data into the object inkbird_ble expects."""
    return BluetoothServiceInfoBleak(
        name=adv.local_name or device.name or device.address,
        address=device.address,
        rssi=adv.rssi,
        manufacturer_data=adv.manufacturer_data,
        service_data=adv.service_data,
        service_uuids=adv.service_uuids,
        source="local",
        device=device,
        advertisement=adv,
        connectable=False,   # we never intend to connect
        time=time.monotonic(),
        tx_power=adv.tx_power,
    )


def main():
    ensure_log_header()
    parser = INKBIRDBluetoothDeviceData()
    seen_addresses = set()

    def detection_callback(device: BLEDevice, adv: AdvertisementData):
        if TARGET_ADDRESS and device.address.upper() != TARGET_ADDRESS.upper():
            return

        try:
            service_info = build_service_info(device, adv)
        except Exception:
            return

        if not parser.supported(service_info):
            return

        update = parser.update(service_info)

        values = {}
        for device_key, value in update.entity_values.items():
            values[device_key.key] = value.native_value

        temperature = values.get("temperature")
        humidity = values.get("humidity")
        battery = values.get("battery")

        label = "new" if device.address not in seen_addresses else "seen"
        seen_addresses.add(device.address)

        print(
            f"[{label}] {device.address}  "
            f"temp={temperature}\u00b0C  humidity={humidity}%  battery={battery}%"
        )

        if temperature is not None:
            append_row(device.address, temperature, humidity, battery)

    async def run():
        # scanning_mode="passive" tells bleak/BlueZ to use passive BLE scanning
        # where supported. On some BlueZ versions/adapters this flag is ignored
        # and it falls back to active scanning at the OS level, but this
        # script itself never opens a GATT connection to the sensor either way,
        # which is the part that actually costs the sensor's battery.
        scanner = BleakScanner(detection_callback=detection_callback, scanning_mode="passive")
        print(f"Listening for INKBIRD advertisements. Logging to {LOG_FILE}")
        print("Press Ctrl+C to stop.\n")
        async with scanner:
            while True:
                await asyncio.sleep(1)

    try:
        asyncio.run(run())
    except KeyboardInterrupt:
        print("\nStopped.")


if __name__ == "__main__":
    main()
```
