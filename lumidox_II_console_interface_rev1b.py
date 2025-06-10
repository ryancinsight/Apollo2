# Import necessary files
import serial # get this w/ "pip install pyserial"
import serial.tools.list_ports
import time   # included with base installtion of python 3

# Be sure to get the FTDI drivers in order to "talk serial"
# to the controller:
# https://www.ftdichip.com/FTDrivers.htm

def getFirmwareVersion():
    serial_data = str(int(getComVal(b"02", 0)))
    return serial_data
        
def getModelNumber():
  model_number_list = []

  serial_data = str(chr(int(getComVal(b"6c", 0))))
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal(b"6d", 0))));
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal(b"6e", 0))));
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal(b"6f", 0))));
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal(b"70", 0))));
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal(b"71", 0))));
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal(b"72", 0))));
  model_number_list.append(serial_data)
                    
  serial_data = str(chr(int(getComVal(b"73", 0))));
  model_number_list.append(serial_data)

  model_number = ''.join(model_number_list)
  
  return model_number
    
def getSerialNumber():
  serial_number_list = []
  
  serial_data = str(chr(int(getComVal(b"60", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"61", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"62", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"63", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"64", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"65", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"66", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"67", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"68", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"69", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"6a", 0))))
  serial_number_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"6b", 0))))
  serial_number_list.append(serial_data)

  serial_number = ''.join(serial_number_list)
  
  return serial_number
    
def getWavelength():
  wavelength_list = []
  
  serial_data = str(chr(int(getComVal(b"76", 0))))
  wavelength_list.append(serial_data)
                            
  serial_data = str(chr(int(getComVal(b"81", 0))))
  wavelength_list.append(serial_data)
                            
  serial_data = str(chr(int(getComVal(b"82", 0))))
  wavelength_list.append(serial_data)
                            
  serial_data = str(chr(int(getComVal(b"89", 0))))
  wavelength_list.append(serial_data)

  serial_data = str(chr(int(getComVal(b"8a", 0))))
  wavelength_list.append(serial_data)

  wavelength = ''.join(wavelength_list)
  
  return wavelength

def fireStage1():
  serial_data = str(chr(int(getComVal(b"15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal(b"78", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal(b"41", current_in_ma))))

def fireStage2():
  serial_data = str(chr(int(getComVal(b"15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal(b"80", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal(b"41", current_in_ma))))

def fireStage3():
  serial_data = str(chr(int(getComVal(b"15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal(b"88", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal(b"41", current_in_ma))))

def fireStage4():
  serial_data = str(chr(int(getComVal(b"15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal(b"90", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal(b"41", current_in_ma))))

def fireStage5():
  serial_data = str(chr(int(getComVal(b"15", 3))))
  time.sleep(0.1)
  serial_data = int(getComVal(b"98", 0))
  current_in_ma = serial_data
  serial_data = str(chr(int(getComVal(b"41", current_in_ma))))

def powerTotalStage1():
  serial_data = int(getComVal(b"7b", 0))
  power_total = serial_data / 10
  return power_total

def powerTotalStage2():
  serial_data = int(getComVal(b"83", 0))
  power_total = serial_data / 10
  return power_total

def powerTotalStage3():
  serial_data = int(getComVal(b"88", 0))
  power_total = serial_data / 10
  return power_total

def powerTotalStage4():
  serial_data = int(getComVal(b"90", 0))
  power_total = serial_data / 10
  return power_total

def powerTotalStage5():
  serial_data = int(getComVal(b"9b", 0))
  power_total = serial_data / 10
  return power_total

def powerPerStage1():
  serial_data = int(getComVal(b"7c", 0))
  power_total = serial_data / 10
  return power_total

def powerPerStage2():
  serial_data = int(getComVal(b"84", 0))
  power_total = serial_data / 10
  return power_total

def powerPerStage3():
  serial_data = int(getComVal(b"8c", 0))
  power_total = serial_data / 10
  return power_total

def powerPerStage4():
  serial_data = int(getComVal(b"94", 0))
  power_total = serial_data / 10
  return power_total

def powerPerStage5():
  serial_data = int(getComVal(b"9c", 0))
  power_total = serial_data / 10
  return power_total

def powerTotalUnits1():
  serial_data = int(getComVal(b"7d", 0))
  return decodeTotalUnits(serial_data)

def powerTotalUnits2():
  serial_data = int(getComVal(b"85", 0))
  return decodeTotalUnits(serial_data)

def powerTotalUnits3():
  serial_data = int(getComVal(b"8d", 0))
  return decodeTotalUnits(serial_data)

def powerTotalUnits4():
  serial_data = int(getComVal(b"95", 0))
  return decodeTotalUnits(serial_data)

def powerTotalUnits5():
  serial_data = int(getComVal(b"9d", 0))
  return decodeTotalUnits(serial_data)

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

def powerPerUnits1():
  serial_data = int(getComVal(b"7e", 0))
  return decodePerUnits(serial_data)

def powerPerUnits2():
  serial_data = int(getComVal(b"86", 0))
  return decodePerUnits(serial_data)

def powerPerUnits3():
  serial_data = int(getComVal(b"8e", 0))
  return decodePerUnits(serial_data)

def powerPerUnits4():
  serial_data = int(getComVal(b"96", 0))
  return decodePerUnits(serial_data)

def powerPerUnits5():
  serial_data = int(getComVal(b"9e", 0))
  return decodePerUnits(serial_data)

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
  serial_data = str(chr(int(getComVal(b"15", 1))))
  time.sleep(1)

# Checksum function
def checkSum(s):
  value = 0
  for w in s[1:]:
    value+=int(w)
  value = value % 256
  ss=str(hex(value)[2:]).rjust(2,'0')
  return bytearray(ss,'utf8')

# Hexadecimal to decimal conversion function
def hexc2dec(bufp):
        newval=0
        divvy=4096
        for pn in range (1,5):
                vally=bufp[pn]
                if(vally < 97):
                        subby=48
                else:
                        subby=87
                newval+=((bufp[pn]-subby)*divvy)
                divvy/=16
                if(newval > 32767):
                        newval=newval-65536
        return newval

# Send data/ return data to/from controller function
def getComVal(s, i):
  i=int(i)
  command=b'*';
  command+=s;
  if i==0: 
    command+=b"0000"
  else:
    command+=bytearray(str(hex(i)[2:]).rjust(4,'0'),'utf8');
  command+=checkSum(command);
  command+=b'\r';
  ser.write(command);
  response= ser.read_until(b'^');
  return hexc2dec(response);

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

# Menu
def menu():
    choice ='0'
    while choice =='0':
        print("-- Select an action --")
        print("1) Turn on stage 1: " + str(powerTotalStage1()) + " " + powerTotalUnits1() + ", " + str(powerPerStage1()) + " " + powerPerUnits1())
        print("2) Turn on stage 2: " + str(powerTotalStage2()) + " " + powerTotalUnits2() + ", " + str(powerPerStage2()) + " " + powerPerUnits2())
        print("3) Turn on stage 3: " + str(powerTotalStage3()) + " " + powerTotalUnits3() + ", " + str(powerPerStage3())+ " " + powerPerUnits3())
        print("4) Turn on stage 4: " + str(powerTotalStage4()) + " " + powerTotalUnits4() + ", " + str(powerPerStage4()) + " " + powerPerUnits4())
        print("5) Turn on stage 5: " + str(powerTotalStage5()) + " " + powerTotalUnits5() + ", " + str(powerPerStage4()) + " " + powerPerUnits5())
        print("6) Turn on stage with specific current (up to " + str(int(getComVal(b"98", 0))) + "mA).")
        print("7) Turn off device.")
        print("8) Quit program.")
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
            if( int(specific_current) > int(getComVal(b"98", 0)) ):
                print("")
                print("Cannot fire above " + str(int(getComVal(b"98", 0))) + "mA. Aborting action.")
                print("")
                return True
            else:
                print("")
                print("Firing with " + specific_current + "mA.")
                print("")
                serial_data = str(chr(int(getComVal(b"15", 3))))
                time.sleep(0.1)
                serial_data = str(chr(int(getComVal(b"41", specific_current))))
                return True
        elif choice == "7":
            print("")
            print("Turning off device.")
            turnOffDevice()
            print("")
            return True
        elif choice == "8":
            print("")
            print("Turning off device.")
            turnOffDevice()
            serial_data = str(chr(int(getComVal(b"15", 0))))
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

############# Main Routine #############
com_port = welcomeMessage()
ser = serial.Serial(com_port, 19200, timeout=1);
ser.reset_input_buffer();
print(com_port + " has been connected!")

serial_data = str(chr(int(getComVal(b"15", 1))))
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












