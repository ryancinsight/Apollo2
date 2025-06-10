Okay, this is a great exercise! The Python scripts and the PDF provide a good foundation for understanding the Lumidox II Controller's capabilities via its serial interface.

Here's a software-agnostic manual based on that information. It will focus on *what* the Lumidox II can do and *how* to communicate with it generally, rather than specific Python implementations.

---

**Lumidox II Controller User Manual (Serial Command Interface)**

**Version 1.0**

**Table of Contents**
1.  Introduction
2.  Safety Precautions
3.  System Overview
    *   Lumidox II Controller
    *   Connected Light Device (Smart Card Features)
4.  Initial Setup & Connection
5.  Operating Modes
    *   Local Mode
    *   Remote Mode (Serial Control)
6.  Serial Communication Protocol
    *   Connection Parameters
    *   Command Structure
    *   Response Structure
    *   Checksum Calculation
7.  Remote Mode Operation
    *   Entering and Exiting Remote Mode
    *   Querying Device Information
    *   Controlling Light Output
        *   Using Pre-defined Stages
        *   Direct Current Control
    *   Reading Stage Parameters
8.  Key Serial Commands (Overview)
    *   Device & Controller Information
    *   Remote Mode Control
    *   Light Device (Smart Card) Parameters
    *   Direct Output Control
9.  Troubleshooting
10. Glossary

---

**1. Introduction**

The Lumidox II Controller is designed to power and control compatible LED light-emitting devices. This manual focuses on operating the Lumidox II Controller via its serial (COM port) interface, allowing for automated control from a computer or other serial-capable host. This provides flexibility for integrating the Lumidox II into various experimental setups and automated processes.

**2. Safety Precautions**

*   **EYE AND SKIN PROTECTION:** The connected light devices can emit high-intensity light. ALWAYS wear appropriate Personal Protective Equipment (PPE) for your eyes and skin to protect against potential hazards from the specific wavelength and intensity of the light device being used.
*   **INFORM OTHERS:** Ensure anyone in the vicinity of an active device is aware of its operation and is also equipped with appropriate PPE.
*   **AUTHORIZED PERSONNEL:** Only trained personnel should operate or service the equipment.
*   **MODIFICATIONS:** Do not modify the controller or light devices. Unauthorized modifications can be hazardous and will void the warranty.
*   **ENVIRONMENT:** Operate the device in a suitable laboratory environment, away from flammable materials and excessive moisture.

**3. System Overview**

*   **Lumidox II Controller:** The main unit that provides power, control logic, and a user interface for local operation. It also features a serial port for remote control.
*   **Connected Light Device (Smart Card Features):** The LED light-emitting module that connects to the Lumidox II Controller. These devices typically store their own identification and operational parameters (often referred to as "Smart Card" data within the command set). This data includes:
    *   Model Number
    *   Serial Number
    *   Wavelength(s)
    *   Pre-defined operational "Stages" (e.g., specific current settings, power output information, units).

**4. Initial Setup & Connection**

1.  **Safety First:** Ensure all safety precautions (Section 2) are followed.
2.  **Connect Light Device:** Securely connect a compatible light device to the Lumidox II controller.
3.  **Connect USB Cable:** Connect a USB cable from your host computer to the USB port on the Lumidox II controller.
4.  **Connect Power:** Connect the Lumidox II controller to AC mains using the provided power adapter.
5.  **Power On:** Turn on the Lumidox II controller. Its display should show the main menu, indicating it is in local mode.
6.  **Install Drivers:** Ensure appropriate FTDI (or equivalent USB-to-Serial) drivers are installed on your host computer to allow serial communication.
7.  **Identify COM Port:** Determine the COM port number assigned to the Lumidox II controller on your host computer (e.g., COM3, COM4).

**5. Operating Modes**

*   **Local Mode:** The default mode upon power-up. The controller is operated using its front panel display and buttons.
*   **Remote Mode (Serial Control):** Allows the controller to be managed by commands sent over the serial interface. When in remote mode, front panel operation is typically disabled. To resume local mode operation after using remote mode, it is often necessary to power cycle the controller.

**6. Serial Communication Protocol**

*   **Connection Parameters:**
    *   **Baud Rate:** 19200
    *   **Parity:** None
    *   **Start Bits:** 1
    *   **Stop Bits:** 1
    *   **Data Bits:** 8 (implied by ASCII usage)
    *   **Flow Control:** None (implied)
    *   **Levels:** TTL

*   **Command Structure (Computer to Controller):**
    `(stx)CCDDDDSS(etx)`
    *   `(stx)`: Start of Text character: `*` (ASCII 42, Hex `0x2a`)
    *   `CC`: 2-character ASCII hexadecimal command code (e.g., "02", "15", "41", "6c"). Use lowercase ASCII for hex 'a' through 'f'.
    *   `DDDD`: 4-character ASCII hexadecimal data value to be sent with the command (e.g., "0000", "0001", "0bb8" for 3000mA). For read commands or commands not requiring data, this is often "0000". Values are typically two's complement for signed numbers if applicable.
    *   `SS`: 2-character ASCII hexadecimal checksum.
    *   `(etx)`: End of Text character: Carriage Return (ASCII 13, Hex `0x0d`)

*   **Response Structure (Controller to Computer):**
    *   **Successful Command:** `(stx)DDDDSS(ack)`
        *   `(stx)`: Start of Text character: `*`
        *   `DDDD`: 4-character ASCII hexadecimal data value returned by the controller.
        *   `SS`: 2-character ASCII hexadecimal checksum of the response.
        *   `(ack)`: Acknowledge character: `^` (ASCII 94, Hex `0x5e`)
    *   **Incorrect Checksum in Sent Command:** `(stx)XXXX60(ack)`
        *   `(stx)`: Start of Text character: `*`
        *   `XXXX`: Placeholder, often indicates error or status.
        *   `60`: Checksum for this specific error response.
        *   `(ack)`: Acknowledge character: `^`

*   **Checksum Calculation:**
    The checksum (`SS`) is an 8-bit (modulo 256) sum of the ASCII values of the characters in the command *excluding* `(stx)` and `(etx)`.
    Example for `*040000SS\r`:
    1.  Characters for checksum: '0', '4', '0', '0', '0', '0'
    2.  ASCII values: 48, 52, 48, 48, 48, 48
    3.  Sum: 48+52+48+48+48+48 = 292
    4.  Modulo 256: 292 mod 256 = 36 (decimal)
    5.  Convert to 2-digit hex: 36 decimal = `24` hex.
    6.  So, `SS` would be "24". The full command: `*04000024\r`

*   **Hexadecimal to Decimal Conversion (for DDDD values from controller):**
    The 4-character hex value `DDDD` returned by the controller often needs to be converted to a decimal value. This is a standard hex-to-decimal conversion. If the value represents a two's complement signed number, values greater than 32767 (0x7FFF) should be interpreted as negative (e.g., 0xFFFF is -1). Many commands also require applying a multiplier (`XMULT` from the PDF documentation) to get the final physical unit (e.g., dividing by 10, 100, or 1000).

**7. Remote Mode Operation**

Before sending most operational commands, the controller must be placed into remote mode.

*   **Entering and Exiting Remote Mode (Command `0x15` - "Remote GO"):**
    *   **Value `0000` (0 decimal):** REMOTE OFF. Returns controller to a state where local operation might be possible (power cycle usually recommended to fully restore local mode).
    *   **Value `0001` (1 decimal):** REMOTE ON, Output OFF. Puts controller in remote mode, ensures light output is OFF. Good for initial setup or pausing operation.
    *   **Value `0002` (2 decimal):** REMOTE ON, Output ARM. Puts controller in remote mode, prepares for firing (specific use case).
    *   **Value `0003` (3 decimal):** REMOTE ON, Output FIRE. Puts controller in remote mode, enables immediate light output upon receiving a current-setting command (e.g., `0x41`).

    *Example: To enter remote mode with output off:* `*150001SS\r` (calculate SS accordingly)

*   **Querying Device Information:**
    Once in remote mode (e.g., `0x15` set to `0001`), you can query information.
    *   **Firmware Revision (Controller):** Command `0x02`, Data `0000`.
    *   **Model Number (Light Device):** Commands `0x6c` through `0x73` (each reads one character). Data `0000`.
    *   **Serial Number (Light Device):** Commands `0x60` through `0x6b` (each reads one character). Data `0000`.
    *   **Wavelength (Light Device):** Commands `0x76`, `0x81`, `0x82`, `0x89`, `0x8a` (each reads one character). Data `0000`.

    *Example: To read firmware revision:* `*020000SS\r`

*   **Controlling Light Output:**
    1.  Ensure controller is in Remote Mode with output enabled for firing (e.g., send `0x15` with data `0003`).
    2.  Then, set the desired current.

    *   **Using Pre-defined Stages:**
        The connected light device stores parameters for up to 5 "stages." To use a stage:
        a. Read the "FIRE Current" for the desired stage (e.g., Stage 1: command `0x78`; Stage 2: `0x80`; Stage 3: `0x88`; Stage 4: `0x90`; Stage 5: `0x98`). Data `0000`. The returned `DDDD` is the current in mA (XMULT=1).
        b. Send the "FIRE Current" command (`0x41`) with the `DDDD` value obtained in step (a) as its data.

        *Example: To fire Stage 1:*
        1.  `*150003SS\r` (Enter Remote ON, Output FIRE mode)
        2.  `*780000SS\r` (Read Stage 1 FIRE current - let's say it returns `0bb8` for 3000mA)
        3.  `*410bb8SS\r` (Set FIRE current to 3000mA)

    *   **Direct Current Control:**
        Set current directly using the "FIRE Current" command (`0x41`). The `DDDD` data is the current in mA (XMULT=1). Check the maximum current supported by the connected light device (e.g., by reading Stage 5 FIRE current with command `0x98`). Do not exceed this.

        *Example: To fire with 500mA (hex `01f4`):*
        1.  `*150003SS\r` (Enter Remote ON, Output FIRE mode)
        2.  `*4101f4SS\r` (Set FIRE current to 500mA)

    *   **Turning Off Light Output (while remaining in Remote Mode):**
        Method 1: Send command `0x15` with data `0001` (REMOTE ON, Output OFF).
        Method 2: Send command `0x41` (FIRE Current) with data `0000` (0 mA).
        Method 1 is generally preferred as a "safe stop."

*   **Reading Stage Parameters (Light Device "Smart Card"):**
    The light device stores various parameters for each of its up to 5 stages. These are read-only.
    *   **Stage 1 Parameters:**
        *   FIRE Current: `0x78` (mA)
        *   Power Total: `0x7b` (Value/10)
        *   Power Per LED/Well: `0x7c` (Value/10)
        *   Total Units: `0x7d` (Index for unit string)
        *   Per LED/Well Units: `0x7e` (Index for unit string)
    *   **Stage 2 Parameters:** Commands `0x80`, `0x83`, `0x84`, `0x85`, `0x86` respectively.
    *   **Stage 3 Parameters:** Commands `0x88`, `0x8b`, `0x8c`, `0x8d`, `0x8e` respectively.
    *   **Stage 4 Parameters:** Commands `0x90`, `0x93`, `0x94`, `0x95`, `0x96` respectively.
    *   **Stage 5 Parameters:** Commands `0x98`, `0x9b`, `0x9c`, `0x9d`, `0x9e` respectively.

    *Unit Decoding (for `0x7d`, `0x85`, etc. - SC TOTAL UNITS):*
    *   0: "W TOTAL RADIANT POWER"
    *   1: "mW TOTAL RADIANT POWER"
    *   2: "W/cm² TOTAL IRRADIANCE"
    *   3: "mW/cm² TOTAL IRRADIANCE"
    *   4: "" (BLANK)
    *   5: "A TOTAL CURRENT"
    *   6: "mA TOTAL CURRENT"
    *   Other: "UNKNOWN UNITS"

    *Unit Decoding (for `0x7e`, `0x86`, etc. - SC PER LED UNITS):*
    *   0: "W PER WELL"
    *   1: "mW PER WELL"
    *   2: "W TOTAL RADIANT POWER"
    *   3: "mW TOTAL RADIANT POWER"
    *   4: "mW/cm² PER WELL"
    *   5: "mW/cm²"
    *   6: "J/s"
    *   7: "" (BLANK)
    *   8: "A PER WELL"
    *   9: "mA PER WELL"
    *   Other: "UNKNOWN UNITS"

**8. Key Serial Commands (Overview)**

This is a summary. Refer to the PDF documentation for a complete list and `XMULT` values. All commands take `0000` as data `DDDD` when reading.

*   **Device & Controller Information (Read Only):**
    *   `0x02`: Revision (Controller Firmware)
    *   `0x60`-`0x6b`: Smart Card Serial Chars [0]-[b] (Light Device Serial Number)
    *   `0x6c`-`0x73`: Smart Card Model Chars [0]-[7] (Light Device Model Number)
    *   `0x76`, `0x81`, `0x82`, `0x89`, `0x8a`: Smart Card Lambda Chars [0]-[4] (Light Device Wavelength)

*   **Remote Mode Control (Write/Read):**
    *   `0x15` (Write), `0x13` (Read): Remote GO (Set/Get remote operational state)
        *   Data for `0x15`: `0000` (Off), `0001` (On, Output Off), `0002` (On, Arm), `0003` (On, Fire)

*   **Light Device (Smart Card) Stage Parameters (Read Only):**
    *(Example for Stage 1, similar command addresses for Stages 2-5)*
    *   `0x77`: Smart Card ARM Current [1] (mA, XMULT=1)
    *   `0x78`: Smart Card FIRE Current [1] (mA, XMULT=1)
    *   `0x79`: Smart Card VOLT Limit [1] (V, XMULT=100, e.g. value/100)
    *   `0x7a`: Smart Card VOLT Start [1] (V, XMULT=100)
    *   `0x7b`: Smart Card POWER TOTAL [1] (Power, XMULT=10, e.g. value/10)
    *   `0x7c`: Smart Card POWER PER LED [1] (Power, XMULT=10)
    *   `0x7d`: Smart Card SC TOTAL UNITS [1] (Index for unit string)
    *   `0x7e`: Smart Card SC PER LED UNITS [1] (Index for unit string)

*   **Direct Output Control (Write/Read):**
    *   `0x41` (Write), `0x21` (Read): FIRE Current (Set/Get current in mA, XMULT=1000 for some interpretations in PDF, but scripts use XMULT=1 for mA) - **Assume XMULT=1 for mA for direct setting.**
    *   `0x40` (Write), `0x20` (Read): ARM Current (mA, XMULT=1000 in PDF, script implies XMULT=1)

**Important Note on XMULT for 0x40/0x41:** The PDF command table lists XMULT 1000 for ARM/FIRE Current (implying value is Amps x 1000 or mA). The Python scripts send integer mA values directly (e.g., `3000` for 3000mA). It's safer to assume direct mA values when *setting* current with `0x41`. Always verify with device behavior.

**9. Troubleshooting**

*   **No Response from Controller:**
    *   Check physical USB connection.
    *   Verify correct COM port is selected in your software.
    *   Ensure COM port parameters (baud, parity, etc.) are correct.
    *   Confirm FTDI drivers are installed and working.
    *   Try power cycling the Lumidox II controller.
*   **Controller Responds with `*XXXX60^`:**
    *   The checksum calculated by your software for the sent command was incorrect. Double-check your checksum calculation logic.
*   **Light Does Not Turn On:**
    *   Ensure the controller is in "Remote ON, Output FIRE" mode (command `0x15`, data `0003`).
    *   Verify a non-zero current has been set using command `0x41`.
    *   Check if the light device is properly connected.
    *   Ensure the current setting is within the capabilities of the light device.

**10. Glossary**

*   **Controller:** The Lumidox II main unit.
*   **Light Device:** The LED module connected to the controller.
*   **Smart Card:** Refers to the memory/parameters stored within the connected Light Device.
*   **Stage:** A pre-defined set of operating parameters (primarily current) stored on the Light Device.
*   **COM Port:** Serial communication port on the host computer.
*   **stx, etx, ack:** Start of Text, End of Text, Acknowledge characters in the serial protocol.
*   **Checksum:** A value used to verify data integrity during transmission.
*   **XMULT:** A multiplier or divisor specified in command documentation to convert raw data values to physical units.

---
**Disclaimer:** This manual is based on analysis of provided Python scripts and PDF documentation. Always refer to the official manufacturer's documentation for the most accurate and up-to-date information. The user assumes all responsibility for the safe and correct operation of the equipment.