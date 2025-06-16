#!/usr/bin/env python3
"""
Lumidox II Stage 1 Power Units Display Script
============================================

This script connects to the Lumidox II Controller on COM4 at 19200 baud
and displays the power units information for Stage 1, including total
radiant power in mW/cm² if configured.

Based on LumidoxII.md protocol specification.
"""

import serial
import time
import sys

# Serial communication parameters
COM_PORT = "COM4"
BAUD_RATE = 19200
TIMEOUT = 1.0

def checkSum(s):
    """Calculate checksum for Lumidox II protocol"""
    total = 0
    for char in s:
        if isinstance(char, str):
            total += ord(char)
        else:
            total += char
    return format(total % 256, '02x')

def hexc2dec(bufp):
    """Convert hexadecimal string to decimal"""
    try:
        return int(bufp, 16)
    except ValueError:
        return 0

def getComVal(ser, command_bytes, data_value):
    """Send command to device and get response"""
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

def decodeTotalUnits(index):
    """Decode total units index to human-readable string"""
    unit_map = {
        0: "W TOTAL RADIANT POWER",
        1: "mW TOTAL RADIANT POWER", 
        2: "W/cm² TOTAL IRRADIANCE",
        3: "mW/cm² TOTAL IRRADIANCE",
        4: "",
        5: "A TOTAL CURRENT",
        6: "mA TOTAL CURRENT"
    }
    return unit_map.get(index, "UNKNOWN UNITS")

def decodePerUnits(index):
    """Decode per-unit index to human-readable string"""
    unit_map = {
        0: "W PER WELL",
        1: "mW PER WELL",
        2: "W TOTAL RADIANT POWER",
        3: "mW TOTAL RADIANT POWER",
        4: "mW/cm² PER WELL",
        5: "mW/cm²",
        6: "J/s",
        7: "",
        8: "A PER WELL",
        9: "mA PER WELL"
    }
    return unit_map.get(index, "UNKNOWN UNITS")

def getStagePowerInfo(ser, stage_num):
    """Get power information for any stage (1-5) including units and current"""
    print(f"Reading Stage {stage_num} power information...")
    
    # Stage power commands based on LumidoxII.md
    # Stage 1: 0x7b-0x7e, Stage 2: 0x83-0x86, Stage 3: 0x8b-0x8e, Stage 4: 0x93-0x96, Stage 5: 0x9b-0x9e
    stage_commands = {
        1: {"total_power": "7b", "per_power": "7c", "total_units": "7d", "per_units": "7e", "fire_current": "78", "arm_current": "77"},
        2: {"total_power": "83", "per_power": "84", "total_units": "85", "per_units": "86", "fire_current": "80", "arm_current": "7f"},
        3: {"total_power": "8b", "per_power": "8c", "total_units": "8d", "per_units": "8e", "fire_current": "88", "arm_current": "87"},
        4: {"total_power": "93", "per_power": "94", "total_units": "95", "per_units": "96", "fire_current": "90", "arm_current": "8f"},
        5: {"total_power": "9b", "per_power": "9c", "total_units": "9d", "per_units": "9e", "fire_current": "98", "arm_current": "97"}
    }
    
    if stage_num not in stage_commands:
        raise ValueError(f"Invalid stage number: {stage_num}. Must be 1-5.")
    
    commands = stage_commands[stage_num]
    
    # Get total power value
    total_power_raw = getComVal(ser, commands["total_power"], 0)
    total_power = total_power_raw / 10.0  # Divide by 10 as per protocol
    
    # Get per LED power value
    per_power_raw = getComVal(ser, commands["per_power"], 0)
    per_power = per_power_raw / 10.0  # Divide by 10 as per protocol
    
    # Get total units index
    total_units_index = getComVal(ser, commands["total_units"], 0)
    total_units = decodeTotalUnits(total_units_index)
    
    # Get per LED units index
    per_units_index = getComVal(ser, commands["per_units"], 0)
    per_units = decodePerUnits(per_units_index)
    
    # Get FIRE current (mA)
    fire_current_ma = getComVal(ser, commands["fire_current"], 0)
    
    # Get ARM current (mA)
    arm_current_ma = getComVal(ser, commands["arm_current"], 0)
    
    return {
        'stage': stage_num,
        'total_power': total_power,
        'total_units': total_units,
        'total_units_index': total_units_index,
        'per_power': per_power,
        'per_units': per_units,
        'per_units_index': per_units_index,
        'fire_current_ma': fire_current_ma,
        'arm_current_ma': arm_current_ma
    }

def getAllStagesPowerInfo(ser):
    """Get power information for all stages (1-5)"""
    all_stages = []
    for stage in range(1, 6):
        try:
            stage_info = getStagePowerInfo(ser, stage)
            all_stages.append(stage_info)
            time.sleep(0.1)  # Small delay between commands
        except Exception as e:
            print(f"Error reading Stage {stage}: {e}")
            # Add placeholder data for failed stage
            all_stages.append({
                'stage': stage,
                'total_power': 0.0,
                'total_units': "ERROR",
                'total_units_index': -1,
                'per_power': 0.0,
                'per_units': "ERROR",
                'per_units_index': -1,
                'fire_current_ma': 0,
                'arm_current_ma': 0
            })
    return all_stages

def calculate_derived_units(power_info, plate_geometry):
    """Calculate additional unit types using device readings and plate geometry"""
    calculations = {}
    
    # Extract power values and convert units as needed
    total_power = power_info['total_power']
    per_power = power_info['per_power']
    total_units = power_info['total_units']
    per_units = power_info['per_units']
    
    # Convert total power to mW if needed
    if "W TOTAL" in total_units and "mW" not in total_units:
        total_power_mw = total_power * 1000  # Convert W to mW
    elif "mW TOTAL" in total_units:
        total_power_mw = total_power
    else:
        total_power_mw = total_power  # Assume mW if unclear
    
    # Convert per power to mW if needed  
    if "W PER" in per_units and "mW" not in per_units:
        per_power_mw = per_power * 1000  # Convert W to mW
    elif "mW PER" in per_units:
        per_power_mw = per_power
    else:
        per_power_mw = per_power  # Assume mW if unclear
    
    # Calculate total area in cm²
    total_area_cm2 = plate_geometry['total_area_cm2']
    well_area_cm2 = plate_geometry['well_area_cm2']
    well_count = plate_geometry['well_count']
    
    # Calculate irradiance values if we have power in mW
    if total_power_mw > 0:
        calculations['total_irradiance_mw_cm2'] = total_power_mw / total_area_cm2
        calculations['total_irradiance_w_cm2'] = (total_power_mw / 1000) / total_area_cm2
    
    # Calculate per-well irradiance
    if per_power_mw > 0 and well_area_cm2 > 0:
        calculations['per_well_irradiance_mw_cm2'] = per_power_mw / well_area_cm2
        calculations['per_well_irradiance_w_cm2'] = (per_power_mw / 1000) / well_area_cm2
    
    # Calculate total power from per-well if available
    if per_power_mw > 0:
        calculations['calculated_total_power_mw'] = per_power_mw * well_count
        calculations['calculated_total_power_w'] = (per_power_mw * well_count) / 1000
    
    # Calculate average irradiance across all wells
    if per_power_mw > 0:
        total_well_area_cm2 = well_area_cm2 * well_count
        calculations['avg_well_irradiance_mw_cm2'] = (per_power_mw * well_count) / total_well_area_cm2
    
    # Calculate power density (power per unit area)
    if total_power_mw > 0:
        calculations['power_density_mw_cm2'] = total_power_mw / total_area_cm2
        calculations['power_density_w_m2'] = (total_power_mw / 1000) / (total_area_cm2 / 10000)  # Convert to W/m²
    
    return calculations

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

def displayCalculatedUnits(stage_info, calculations, geometry):
    """Display calculated unit types"""
    stage = stage_info['stage']
    
    print(f"\n" + "="*80)
    print(f"                 CALCULATED UNITS FOR STAGE {stage}")
    print("="*80)
    
    print(f"\nPLATE GEOMETRY (from schematic):")
    print(f"  Plate size: {geometry['plate_length_cm']:.2f} x {geometry['plate_width_cm']:.2f} cm")
    print(f"  Total area: {geometry['total_area_cm2']:.2f} cm²")
    print(f"  Well count: {geometry['well_count']}")
    print(f"  Well area: {geometry['well_area_cm2']:.3f} cm² per well")
    print(f"\nDEVICE READINGS:")
    print(f"  Total Power: {stage_info['total_power']:.1f} {stage_info['total_units']}")
    print(f"  Per Well: {stage_info['per_power']:.1f} {stage_info['per_units']}")
    print(f"  FIRE Current: {stage_info['fire_current_ma']} mA")
    print(f"  ARM Current: {stage_info['arm_current_ma']} mA")
    
    print(f"\nCALCULATED IRRADIANCE VALUES:")
    
    if 'total_irradiance_mw_cm2' in calculations:
        print(f"  Total Irradiance: {calculations['total_irradiance_mw_cm2']:.3f} mW/cm² ⭐")
        print(f"  Total Irradiance: {calculations['total_irradiance_w_cm2']:.6f} W/cm²")
    
    if 'per_well_irradiance_mw_cm2' in calculations:
        print(f"  Per Well Irradiance: {calculations['per_well_irradiance_mw_cm2']:.3f} mW/cm²")
        print(f"  Per Well Irradiance: {calculations['per_well_irradiance_w_cm2']:.6f} W/cm²")
    
    if 'avg_well_irradiance_mw_cm2' in calculations:
        print(f"  Average Well Irradiance: {calculations['avg_well_irradiance_mw_cm2']:.3f} mW/cm²")
    
    print(f"\nCALCULATED POWER VALUES:")
    
    if 'calculated_total_power_mw' in calculations:
        print(f"  Calculated Total (from per-well): {calculations['calculated_total_power_mw']:.1f} mW")
        print(f"  Calculated Total (from per-well): {calculations['calculated_total_power_w']:.3f} W")
    
    if 'power_density_mw_cm2' in calculations:
        print(f"  Power Density: {calculations['power_density_mw_cm2']:.3f} mW/cm²")
        print(f"  Power Density: {calculations['power_density_w_m2']:.1f} W/m²")
    
    print(f"\n⭐ KEY RESULT: If this stage were configured with mW/cm² units,")
    print(f"   it would read approximately {calculations.get('total_irradiance_mw_cm2', 0):.3f} mW/cm²")
    
    print("="*80)

def displayAvailableUnitTypes():
    """Display all available unit types that the device supports"""
    print("\n" + "="*80)
    print("                      AVAILABLE UNIT TYPES")
    print("="*80)
    
    print("\nTOTAL POWER UNIT OPTIONS (for Total Power measurements):")
    print("  Index 0: W TOTAL RADIANT POWER")
    print("  Index 1: mW TOTAL RADIANT POWER")
    print("  Index 2: W/cm² TOTAL IRRADIANCE")
    print("  Index 3: mW/cm² TOTAL IRRADIANCE  ⭐ (This is mW/cm² for total radiant power)")
    print("  Index 4: (BLANK)")
    print("  Index 5: A TOTAL CURRENT")
    print("  Index 6: mA TOTAL CURRENT")
    
    print("\nPER LED/WELL UNIT OPTIONS (for Per-Unit measurements):")
    print("  Index 0: W PER WELL")
    print("  Index 1: mW PER WELL")
    print("  Index 2: W TOTAL RADIANT POWER")
    print("  Index 3: mW TOTAL RADIANT POWER")
    print("  Index 4: mW/cm² PER WELL")
    print("  Index 5: mW/cm²  ⭐ (This is also mW/cm² for per-unit power)")
    print("  Index 6: J/s")
    print("  Index 7: (BLANK)")
    print("  Index 8: A PER WELL")
    print("  Index 9: mA PER WELL")
    
    print("\n" + "="*80)

def displayAllStagesPowerInfo(all_stages_info):
    """Display power information for all stages in a formatted way"""
    print("\n" + "="*80)
    print("                    CURRENT STAGE CONFIGURATIONS")
    print("="*80)
    
    mw_cm2_stages = []
    unit_summary = {}
    
    for stage_info in all_stages_info:
        stage = stage_info['stage']
        print(f"\nSTAGE {stage}:")
        print(f"  Total Power: {stage_info['total_power']:.1f} {stage_info['total_units']}")
        print(f"  Per LED/Well: {stage_info['per_power']:.1f} {stage_info['per_units']}")
        print(f"  FIRE Current: {stage_info['fire_current_ma']} mA")
        print(f"  ARM Current: {stage_info['arm_current_ma']} mA")
        print(f"  Unit Indices: Total={stage_info['total_units_index']}, Per={stage_info['per_units_index']}")
        
        # Track unit usage
        total_unit = f"Index {stage_info['total_units_index']}: {stage_info['total_units']}"
        per_unit = f"Index {stage_info['per_units_index']}: {stage_info['per_units']}"
        
        if total_unit not in unit_summary:
            unit_summary[total_unit] = []
        unit_summary[total_unit].append(f"Stage {stage} (Total)")
        
        if per_unit not in unit_summary:
            unit_summary[per_unit] = []
        unit_summary[per_unit].append(f"Stage {stage} (Per)")
        
        # Check for mW/cm² units
        if stage_info['total_units_index'] == 3:
            mw_cm2_stages.append((stage, 'total', stage_info['total_power']))
        if stage_info['per_units_index'] == 5:
            mw_cm2_stages.append((stage, 'per', stage_info['per_power']))
    
    print("\n" + "="*80)
    print("                     UNIT USAGE SUMMARY")
    print("="*80)
    for unit_type, stages in unit_summary.items():
        print(f"{unit_type}")
        for stage in stages:
            print(f"    Used by: {stage}")
        print()
    
    # Summary of mW/cm² stages
    if mw_cm2_stages:
        print("🌟"*40)
        print("   STAGES WITH mW/cm² TOTAL RADIANT POWER:")
        print("🌟"*40)
        for stage, power_type, value in mw_cm2_stages:
            if power_type == 'total':
                print(f"   Stage {stage}: {value:.1f} mW/cm² (Total Power)")
            else:
                print(f"   Stage {stage}: {value:.1f} mW/cm² (Per-Unit Power)")
        print("🌟"*40)
    else:
        print("⚠️  No stages currently configured with mW/cm² total radiant power")
    
    print("\n" + "="*80)

def main():
    """Main function"""
    print("Lumidox II All Stages Power Units Display")
    print("=========================================")
    print(f"Connecting to {COM_PORT} at {BAUD_RATE} baud...")
    
    try:
        # Open serial connection
        ser = serial.Serial(
            port=COM_PORT,
            baudrate=BAUD_RATE,
            parity=serial.PARITY_NONE,
            stopbits=serial.STOPBITS_ONE,
            bytesize=serial.EIGHTBITS,
            timeout=TIMEOUT
        )
        
        print(f"Connected successfully to {ser.name}")
        
        # Give the connection a moment to stabilize
        time.sleep(0.5)
        
        # Enter remote mode (command 0x15 with value 0001 - Remote ON, Output OFF)
        print("\nEntering remote mode...")
        response = getComVal(ser, "15", 1)
        if response is not None:
            print("Remote mode activated successfully")
        else:
            print("Warning: Could not confirm remote mode activation")
        
        time.sleep(0.1)        # Get and display all stages power information
        print("\nScanning all stages for power information...")
        all_stages_info = getAllStagesPowerInfo(ser)
        
        # Get plate geometry from schematic
        plate_geometry = get_plate_geometry()
        
        # Display available unit types first
        displayAvailableUnitTypes()
        
        # Then display current configurations
        displayAllStagesPowerInfo(all_stages_info)
        
        # Calculate and display derived units for each stage
        print("\n" + "🧮"*40)
        print("           CALCULATING ADDITIONAL UNIT TYPES")
        print("🧮"*40)
        
        for stage_info in all_stages_info:
            if stage_info['total_power'] > 0 or stage_info['per_power'] > 0:
                calculations = calculate_derived_units(stage_info, plate_geometry)
                displayCalculatedUnits(stage_info, calculations, plate_geometry)
            else:
                print(f"\nStage {stage_info['stage']}: No power data available for calculations")
        
        # Exit remote mode (command 0x15 with value 0000 - Remote OFF)
        print("\nExiting remote mode...")
        getComVal(ser, "15", 0)
        print("Remote mode deactivated")
        
    except serial.SerialException as e:
        print(f"Serial communication error: {e}")
        print(f"Please check that:")
        print(f"  - The device is connected to {COM_PORT}")
        print(f"  - The device is powered on")
        print(f"  - No other applications are using {COM_PORT}")
        sys.exit(1)
        
    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
        sys.exit(0)
        
    except Exception as e:
        print(f"Unexpected error: {e}")
        sys.exit(1)
        
    finally:
        if 'ser' in locals() and ser.is_open:
            ser.close()
            print(f"Serial connection to {COM_PORT} closed")

if __name__ == "__main__":
    main()
