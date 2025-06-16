# Import necessary files
import serial # get this w/ "pip install pyserial"
import serial.tools.list_ports
import time   # included with base installtion of python 3

# Be sure to get the FTDI drivers in order to "talk serial"
# to the controller:
# https://www.ftdichip.com/FTDrivers.htm

# Global variable for serial connection
ser = None

def getFirmwareVersion():
    serial_data = str(int(getComVal("02", 0)))
    return serial_data
        
def getModelNumber():
  model_number_list = []

  serial_data = str(chr(int(getComVal("6c", 0))))
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal("6d", 0))))
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal("6e", 0))))
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal("6f", 0))))
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal("70", 0))))
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal("71", 0))))
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal("72", 0))))
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal("73", 0))))
  model_number_list.append(serial_data)

  model_number = ''.join(model_number_list)
  
  return model_number
    
def getSerialNumber():
  serial_number_list = []
  
  serial_data = str(chr(int(getComVal("60", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("61", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("62", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("63", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("64", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("65", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("66", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("67", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("68", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("69", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("6a", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal("6b", 0))))
  serial_number_list.append(serial_data)

  serial_number = ''.join(serial_number_list)
  
  return serial_number
    
def getWavelength():
  wavelength_list = []
  
  serial_data = str(chr(int(getComVal("76", 0))))
  wavelength_list.append(serial_data)
                            
  serial_data = str(chr(int(getComVal("81", 0))))
  wavelength_list.append(serial_data)
                            
  serial_data = str(chr(int(getComVal("82", 0))))
  wavelength_list.append(serial_data)
                            
  serial_data = str(chr(int(getComVal("89", 0))))
  wavelength_list.append(serial_data)

  serial_data = str(chr(int(getComVal("8a", 0))))
  wavelength_list.append(serial_data)

  wavelength = ''.join(wavelength_list)
  
  return wavelength



def fireStage2():
  serial_data = str(chr(int(getComVal("15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal("80", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal("41", current_in_ma))))
def fireStage1():
  serial_data = str(chr(int(getComVal("15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal("78", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal("41", current_in_ma))))
def fireStage3():
  serial_data = str(chr(int(getComVal("15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal("88", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal("41", current_in_ma))))

def fireStage4():
  serial_data = str(chr(int(getComVal("15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal("90", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal("41", current_in_ma))))

def fireStage5():
  serial_data = str(chr(int(getComVal("15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal("98", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal("41", current_in_ma))))

def fireCurrentStage1():
  serial_data = int(getComVal("78", 0))
  return serial_data

def fireCurrentStage2():
  serial_data = int(getComVal("80", 0))
  return serial_data

def fireCurrentStage3():
  serial_data = int(getComVal("88", 0))
  return serial_data

def fireCurrentStage4():
  serial_data = int(getComVal("90", 0))
  return serial_data

def fireCurrentStage5():
  serial_data = int(getComVal("98", 0))
  return serial_data

def powerTotalStage1(default=0):
  try:
    serial_data = int(getComVal("7b", 0))  # Always send 0 for read operations
    power_total = serial_data / 10
    return power_total
  except:
    return default  # Return default value if communication fails

def powerTotalStage2(default=0):
  try:
    serial_data = int(getComVal("83", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerTotalStage3(default=0):
  try:
    serial_data = int(getComVal("8b", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerTotalStage4(default=0):
  try:
    serial_data = int(getComVal("93", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerTotalStage5(default=0):
  try:
    serial_data = int(getComVal("9b", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerPerStage1(default=0):
  try:
    serial_data = int(getComVal("7c", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerPerStage2(default=0):
  try:
    serial_data = int(getComVal("84", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerPerStage3(default=0):
  try:
    serial_data = int(getComVal("8c", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerPerStage4(default=0):
  try:
    serial_data = int(getComVal("94", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerPerStage5(default=0):
  try:
    serial_data = int(getComVal("9c", 0))
    power_total = serial_data / 10
    return power_total
  except:
    return default

def powerTotalUnits1(default=0):
  try:
    serial_data = int(getComVal("7d", 0))
    return decodeTotalUnits(serial_data)
  except:
    return decodeTotalUnits(default)

def powerTotalUnits2(default=0):
  try:
    serial_data = int(getComVal("85", 0))
    return decodeTotalUnits(serial_data)
  except:
    return decodeTotalUnits(default)

def powerTotalUnits3(default=0):
  try:
    serial_data = int(getComVal("8d", 0))
    return decodeTotalUnits(serial_data)
  except:
    return decodeTotalUnits(default)

def powerTotalUnits4(default=0):
  try:
    serial_data = int(getComVal("95", 0))
    return decodeTotalUnits(serial_data)
  except:
    return decodeTotalUnits(default)

def powerTotalUnits5(default=0):
  try:
    serial_data = int(getComVal("9d", 0))
    return decodeTotalUnits(serial_data)
  except:
    return decodeTotalUnits(default)

def decodeTotalUnits(index):
  total_units = "???"
  if index == 0:
      total_units = "W TOTAL RADIANT POWER"
  elif index == 1:
      total_units = "mW TOTAL RADIANT POWER"
  elif index == 2:
      total_units = "W/cm² TOTAL IRRADIANCE"
  elif index == 3:
      total_units = "mW/cm² TOTAL IRRADIANCE"
  elif index == 4:
      total_units = ""
  elif index == 5:
      total_units = "A TOTAL CURRENT"
  elif index == 6:
      total_units = "mA TOTAL CURRENT"
  else:
      total_units = "UNKNOWN UNITS"

  return total_units

def powerPerUnits1(default=0):
  try:
    serial_data = int(getComVal("7e", 0))
    return decodePerUnits(serial_data)
  except:
    return decodePerUnits(default)

def powerPerUnits2(default=0):
  try:
    serial_data = int(getComVal("86", 0))
    return decodePerUnits(serial_data)
  except:
    return decodePerUnits(default)

def powerPerUnits3(default=0):
  try:
    serial_data = int(getComVal("8e", 0))
    return decodePerUnits(serial_data)
  except:
    return decodePerUnits(default)

def powerPerUnits4(default=0):
  try:
    serial_data = int(getComVal("96", 0))
    return decodePerUnits(serial_data)
  except:
    return decodePerUnits(default)

def powerPerUnits5(default=0):
  try:
    serial_data = int(getComVal("9e", 0))
    return decodePerUnits(serial_data)
  except:
    return decodePerUnits(default)

def decodePerUnits(index):
  per_units = "???"
  if index == 0:
      per_units = "W PER WELL"
  elif index == 1:
      per_units = "mW PER WELL"
  elif index == 2:
      per_units = "W TOTAL RADIANT POWER"
  elif index == 3:
      per_units = "mW TOTAL RADIANT POWER"
  elif index == 4:
      per_units = "mW/cm² PER WELL"
  elif index == 5:
      per_units = "mW/cm²"
  elif index == 6:
      per_units = "J/s"
  elif index == 7:
      per_units = ""
  elif index == 8:
      per_units = "A PER WELL"
  elif index == 9:
      per_units = "mA PER WELL"
  else:
      per_units = "UNKNOWN UNITS"

  return per_units

def turnOffDevice():
  serial_data = str(chr(int(getComVal("15", 1))))
  time.sleep(1)

# Checksum function - Updated for better compatibility
def checkSum(s):
    """Calculate checksum for Lumidox II protocol"""
    total = 0
    for char in s:
        if isinstance(char, str):
            total += ord(char)
        else:
            total += char
    return format(total % 256, '02x')

# Hexadecimal to decimal conversion function - Updated
def hexc2dec(bufp):
    """Convert hexadecimal string to decimal"""
    try:
        return int(bufp, 16)
    except ValueError:
        return 0

# Send data/ return data to/from controller function - Updated
def getComVal(command_bytes, data_value=0):
    """
    Send command to device and get response - Updated version
    
    Args:
        command_bytes: 2-character hex command code as string or bytes
        data_value: Integer data value (0 for read operations)
    
    Returns:
        Integer value from device response
    """
    try:
        # Convert command to string if it's bytes
        if isinstance(command_bytes, bytes):
            command_str = command_bytes.decode('ascii')
        else:
            command_str = command_bytes
        
        # Format data value as 4-character hex
        data_str = format(data_value, '04x')
        
        # Build command string without STX and ETX
        cmd_without_markers = command_str + data_str
        
        # Calculate checksum
        checksum = checkSum(cmd_without_markers)
        
        # Build complete command with STX (*) and ETX (\r)
        full_command = '*' + cmd_without_markers + checksum + '\r'
        
        # Send command
        ser.write(full_command.encode('ascii'))
        
        # Read response
        response = ser.read(8).decode('ascii', errors='ignore')
        
        if len(response) >= 7 and response[0] == '*' and response[-1] == '^':
            # Extract data portion (4 hex characters)
            data_hex = response[1:5]
            return hexc2dec(data_hex)
        else:
            print(f"Invalid response: {repr(response)}")
            return 0
            
    except Exception as e:
        print(f"Communication error: {e}")
        return 0

# List COM ports (old function but works across all OS types)
def getAvailableSerialPortsOld():
    ports = ['COM%s' % (i + 1) for i in range(256)]
    result = []
    for port in ports:
        try:
            ser = serial.Serial(port)
            print(ser.name)
            ser.close()
            result.append(port)
        except (OSError, serial.SerialException):
            pass
    return result

# List COM ports
def getValidSerialPorts():
    ports = list(serial.tools.list_ports.comports())
    valid_ports = []
    for p in ports:
        if(str(p).find("USB Serial Port") == -1):
            pass
        else:
            valid_ports.append((str(p)[3:5].strip(), str(p)))
    return valid_ports
    
def welcomeMessage():
    print("Welcome to the Analytical Sales & Services, Inc. Lumidox II Controller PC App!")
    print("")
    print("Before we get started please make sure to do the following:")
    print("  * Have proper PPE for skin & eyes to protect from high powered LEDs.")
    print("  * Ensure those around also have the same level of PPE.")
    print("  * Connect a light device to the Lumidox II controller.")
    print("  * Connect a USB cable from the PC to the Lumidox II controller.")
    print("  * Connect the Lumidox II controller to AC mains with the power adapter.")
    print("  * Power on the Lumidox II controller to show the main menu on it's display.")
    print("")
    choice = input("Press ENTER after the above is complete.")
    print("")

    valid_ports = getValidSerialPorts()
    print("Available COM ports on this PC: ")
    for port in valid_ports:
        (port_number, port_desciption) = port
        print(port_desciption)
    print("")

    final_choice = "-1"
    while(final_choice == "-1"):
        choice = input("Please type in the COM port # the Lumidox II is connected to on the PC: ")
        for port in valid_ports:
            (port_number, port_desciption) = port
            if (choice != port_number):
               print("Not a valid port number.")
               print("")
            else:
                final_choice = choice
                break
    
    ser = serial.Serial('COM' + str(final_choice), 19200, timeout=1);
    ser.reset_input_buffer();
    ser.close();
    print("")
    
    # test for existence here of the com port

    return 'COM' + str(choice)

def get_stage_mw_cm2_display(stage_num):
    """Get mW/cm² value for menu display"""
    try:
        mw_cm2_data = calculate_mw_cm2_for_stage(stage_num)
        if mw_cm2_data and 'total_irradiance_mw_cm2' in mw_cm2_data:
            return f", {mw_cm2_data['total_irradiance_mw_cm2']:.3f} mW/cm² total irradiance"
        else:
            return ", 0.000 mW/cm² total irradiance"
    except Exception as e:
        # Return a simple fallback if there are any errors
        return ", -- mW/cm² total irradiance"

# Menu
def menu():
    choice ='0'
    while choice =='0':
        print("-- Select an action --")
        print("1) Turn on stage 1: " + str(fireCurrentStage1()) + "mA, " + str(powerTotalStage1(default=3)) + " " + powerTotalUnits1(default=3) + ", " + str(powerPerStage1()) + " " + powerPerUnits1() + get_stage_mw_cm2_display(1))
        print("2) Turn on stage 2: " + str(fireCurrentStage2()) + "mA, " + str(powerTotalStage2()) + " " + powerTotalUnits2() + ", " + str(powerPerStage2()) + " " + powerPerUnits2() + get_stage_mw_cm2_display(2))
        print("3) Turn on stage 3: " + str(fireCurrentStage3()) + "mA, " + str(powerTotalStage3()) + " " + powerTotalUnits3() + ", " + str(powerPerStage3()) + " " + powerPerUnits3() + get_stage_mw_cm2_display(3))
        print("4) Turn on stage 4: " + str(fireCurrentStage4()) + "mA, " + str(powerTotalStage4()) + " " + powerTotalUnits4() + ", " + str(powerPerStage4()) + " " + powerPerUnits4() + get_stage_mw_cm2_display(4))
        print("5) Turn on stage 5: " + str(fireCurrentStage5()) + "mA, " + str(powerTotalStage5()) + " " + powerTotalUnits5() + ", " + str(powerPerStage5()) + " " + powerPerUnits5() + get_stage_mw_cm2_display(5))
        print("6) Turn on stage with specific current (up to " + str(int(getComVal("98", 0))) + "mA).")
        print("7) Turn off device.")
        print("8) Show device unit diagnostics.")
        print("9) Show stage mW/cm² calculations.")
        print("10) Show detailed stage info with mW/cm².")
        print("11) Quit program.")
        print("")
        choice = input("Please enter choice number, then press ENTER: ")

        if choice == "1":
            print("")
            print("Firing stage 1.")
            print("")
            fireStage1()
            return True
        elif choice == "2":
            print("")
            print("Firing stage 2.")
            print("")
            fireStage2()
            return True
        elif choice == "3":
            print("")
            print("Firing stage 3.")
            print("")
            fireStage3()
            return True
        elif choice == "4":
            print("")
            print("Firing stage 4.")
            print("")
            fireStage4()
            return True
        elif choice == "5":
            print("")
            print("Firing stage 5.")
            print("")
            fireStage5()
            return True
        elif choice == "6":
            print("")
            specific_current = input("Please enter current in mA (no decimals), then press ENTER: ")
            if (specific_current.isnumeric() is not True):
                print("")
                print("Invalid input. Aborting action")
                print("")
                return True
            if( int(specific_current) > int(getComVal("98", 0)) ):
                print("")
                print("Cannot fire above " + str(int(getComVal("98", 0))) + "mA. Aborting action.")
                print("")
                return True
            else:
                print("")
                print("Firing with " + specific_current + "mA.")
                print("")
                serial_data = str(chr(int(getComVal("15", 3))))
                time.sleep(0.1)
                serial_data = str(chr(int(getComVal("41", specific_current))))
                return True
        elif choice == "7":
            print("")
            print("Turning off device.")
            turnOffDevice()
            print("")
            return True
        elif choice == "8":
            print("")
            diagnoseUnits()
            print("")
            return True
        elif choice == "9":
            print("")
            print("=== mW/cm² CALCULATIONS FOR ALL STAGES ===")
            for stage in range(1, 6):
                mw_cm2_data = calculate_mw_cm2_for_stage(stage)
                if mw_cm2_data:
                    print(f"Stage {stage}: {mw_cm2_data['total_irradiance_mw_cm2']:.3f} mW/cm²")
                else:
                    print(f"Stage {stage}: No power data available")
            print("")
            return True
        elif choice == "10":
            print("")
            stage_choice = input("Which stage (1-5) would you like detailed info for? ")
            if stage_choice.isdigit() and 1 <= int(stage_choice) <= 5:
                display_stage_with_mw_cm2(int(stage_choice))
            else:
                print("Invalid stage number.")
            print("")
            return True
        elif choice == "11":
            print("")
            print("Turning off device.")
            turnOffDevice()
            serial_data = str(chr(int(getComVal("15", 0))))
            time.sleep(1)
            print("To use resume using the controller in local mode, please cycle the power with on/off switch.")
            time.sleep(1)
            print("Quitting program...")
            time.sleep(1)
            print("")
            return False
        else:
            print("")
            print("Not a valid choice. Please try again.")
            print("")
            return True

def diagnoseUnits():
  """Diagnostic function to show what unit indices are stored in the device"""
  print("=== DEVICE UNIT DIAGNOSTICS ===")
  
  for stage in range(1, 6):
    print(f"\nStage {stage}:")
    
    # Get the register addresses for this stage
    if stage == 1:
      total_units_cmd, per_units_cmd = "7d", "7e"
    elif stage == 2:
      total_units_cmd, per_units_cmd = "85", "86"
    elif stage == 3:
      total_units_cmd, per_units_cmd = "8d", "8e"
    elif stage == 4:
      total_units_cmd, per_units_cmd = "95", "96"
    elif stage == 5:
      total_units_cmd, per_units_cmd = "9d", "9e"
    
    try:
      # Read the actual unit indices stored in the device
      total_index = int(getComVal(total_units_cmd, 0))
      per_index = int(getComVal(per_units_cmd, 0))
      
      print(f"  Total Units Index: {total_index} -> {decodeTotalUnits(total_index)}")
      print(f"  Per Units Index: {per_index} -> {decodePerUnits(per_index)}")
      
      # Add mW/cm² calculation for this stage
      mw_cm2_data = calculate_mw_cm2_for_stage(stage)
      if mw_cm2_data:
        print(f"  ⭐ Calculated mW/cm²: {mw_cm2_data['total_irradiance_mw_cm2']:.3f}")
      
    except Exception as e:
      print(f"  Error reading stage {stage}: {e}")

def get_plate_geometry():
    """Get plate geometry from the schematic"""
    # Based on the schematic dimensions
    plate_length_mm = 127.75
    plate_width_mm = 105.5
    
    # Convert to cm
    plate_length_cm = plate_length_mm / 10
    plate_width_cm = plate_width_mm / 10
    
    # Calculate total area
    total_area_cm2 = plate_length_cm * plate_width_cm
    
    # From schematic: appears to be 96-well plate (8x12 grid)
    well_count = 96
    
    # Estimate well spacing and area (typical 96-well plate)
    well_spacing_mm = 9.0  # Typical 96-well spacing
    well_diameter_mm = 6.5  # Typical well diameter
    
    well_area_cm2 = 3.14159 * (well_diameter_mm/20)**2  # Convert to cm² and calculate circle area
    
    return {
        'plate_length_cm': plate_length_cm,
        'plate_width_cm': plate_width_cm,
        'total_area_cm2': total_area_cm2,
        'well_count': well_count,
        'well_area_cm2': well_area_cm2,
        'well_spacing_mm': well_spacing_mm,
        'well_diameter_mm': well_diameter_mm
    }

def calculate_mw_cm2_for_stage(stage_num):
    """Calculate mW/cm² for a specific stage using device readings and geometry"""
    try:
        # Get stage power information
        if stage_num == 1:
            total_power = powerTotalStage1()
            per_power = powerPerStage1()
            total_units = powerTotalUnits1()
            per_units = powerPerUnits1()
        elif stage_num == 2:
            total_power = powerTotalStage2()
            per_power = powerPerStage2()
            total_units = powerTotalUnits2()
            per_units = powerPerUnits2()
        elif stage_num == 3:
            total_power = powerTotalStage3()
            per_power = powerPerStage3()
            total_units = powerTotalUnits3()
            per_units = powerPerUnits3()
        elif stage_num == 4:
            total_power = powerTotalStage4()
            per_power = powerPerStage4()
            total_units = powerTotalUnits4()
            per_units = powerPerUnits4()
        elif stage_num == 5:
            total_power = powerTotalStage5()
            per_power = powerPerStage5()
            total_units = powerTotalUnits5()
            per_units = powerPerUnits5()
        else:
            return None
        
        # Get plate geometry
        geometry = get_plate_geometry()
        
        # Convert total power to mW if needed
        if "W TOTAL" in total_units and "mW" not in total_units:
            total_power_mw = total_power * 1000  # Convert W to mW
        elif "mW TOTAL" in total_units:
            total_power_mw = total_power
        else:
            total_power_mw = total_power  # Assume mW if unclear
        
        # Calculate irradiance in mW/cm²
        if total_power_mw > 0:
            total_irradiance_mw_cm2 = total_power_mw / geometry['total_area_cm2']
            return {
                'total_irradiance_mw_cm2': total_irradiance_mw_cm2,
                'total_power_mw': total_power_mw,
                'total_area_cm2': geometry['total_area_cm2'],
                'well_count': geometry['well_count']
            }
        
        return None
        
    except Exception as e:
        print(f"Error calculating mW/cm² for Stage {stage_num}: {e}")
        return None

def display_stage_with_mw_cm2(stage_num):
    """Display stage information including calculated mW/cm² values"""
    print(f"\n=== STAGE {stage_num} DETAILED INFORMATION ===")
    
    try:
        # Get basic stage info
        if stage_num == 1:
            fire_current = fireCurrentStage1()
            total_power = powerTotalStage1()
            per_power = powerPerStage1()
            total_units = powerTotalUnits1()
            per_units = powerPerUnits1()
        elif stage_num == 2:
            fire_current = fireCurrentStage2()
            total_power = powerTotalStage2()
            per_power = powerPerStage2()
            total_units = powerTotalUnits2()
            per_units = powerPerUnits2()
        elif stage_num == 3:
            fire_current = fireCurrentStage3()
            total_power = powerTotalStage3()
            per_power = powerPerStage3()
            total_units = powerTotalUnits3()
            per_units = powerPerUnits3()
        elif stage_num == 4:
            fire_current = fireCurrentStage4()
            total_power = powerTotalStage4()
            per_power = powerPerStage4()
            total_units = powerTotalUnits4()
            per_units = powerPerUnits4()
        elif stage_num == 5:
            fire_current = fireCurrentStage5()
            total_power = powerTotalStage5()
            per_power = powerPerStage5()
            total_units = powerTotalUnits5()
            per_units = powerPerUnits5()
        else:
            print("Invalid stage number")
            return
        
        print(f"Device Settings:")
        print(f"  Fire Current: {fire_current} mA")
        print(f"  Total Power: {total_power} {total_units}")
        print(f"  Per Well Power: {per_power} {per_units}")
        
        # Calculate and display mW/cm²
        mw_cm2_data = calculate_mw_cm2_for_stage(stage_num)
        if mw_cm2_data:
            geometry = get_plate_geometry()
            print(f"\nCalculated Values:")
            print(f"  Total Power: {mw_cm2_data['total_power_mw']:.1f} mW")
            print(f"  Plate Area: {mw_cm2_data['total_area_cm2']:.2f} cm²")
            print(f"  ⭐ Total Irradiance: {mw_cm2_data['total_irradiance_mw_cm2']:.3f} mW/cm²")
            print(f"  Well Count: {mw_cm2_data['well_count']}")
              # Per-well calculations
            if per_power > 0:
                per_power_mw = per_power * 1000 if "W PER" in per_units and "mW" not in per_units else per_power
                per_well_irradiance = per_power_mw / geometry['well_area_cm2']
                print(f"  Per Well Irradiance: {per_well_irradiance:.3f} mW/cm²")
        else:
            print("\nNo power data available for mW/cm² calculations")
            
    except Exception as e:
        print(f"Error displaying Stage {stage_num} information: {e}")

############# Main Routine #############
if __name__ == "__main__":
    com_port = welcomeMessage()
    ser = serial.Serial(com_port, 19200, timeout=1);
    ser.reset_input_buffer();
    print(com_port + " has been connected!")

    serial_data = str(chr(int(getComVal("15", 1))))
    time.sleep(0.1)
    print('--------------------------------------')
    print("Controller Firmware Version: 1." + getFirmwareVersion())
    print("Devce Model Number: " + getModelNumber())
    print("Device Serial Number: " + getSerialNumber())
    print("Device Wavelength: " + getWavelength())
    print("")

    loop_flag = True
    while(loop_flag):
        loop_flag = menu()

    ser.close();