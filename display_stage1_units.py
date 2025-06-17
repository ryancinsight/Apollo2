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
    """
    Enhanced calculation wrapper that uses the comprehensive unit calculation
    Maintains backward compatibility while providing enhanced features
    """
    # Use the enhanced calculation function
    enhanced_calculations = calculate_all_possible_units(power_info, plate_geometry)
    
    # Extract backward-compatible results
    unit_matrix = enhanced_calculations['unit_matrix']
    
    # Legacy format for backward compatibility
    calculations = {}
    
    # Extract main values
    total_power_mw = unit_matrix['total_power_mw']['value']
    per_power_mw = unit_matrix['per_power_mw']['value']
    
    # Data sources (enhanced)
    if unit_matrix['total_power_mw']['source'] != 'NONE':
        calculations['total_power_source'] = f"{unit_matrix['total_power_mw']['source']}: {total_power_mw:.1f} mW"
    if unit_matrix['per_power_mw']['source'] != 'NONE':
        calculations['per_power_source'] = f"{unit_matrix['per_power_mw']['source']}: {per_power_mw:.1f} mW"
    
    # Irradiance calculations
    if unit_matrix['total_irradiance_mw_cm2']['value'] > 0:
        calculations['total_irradiance_mw_cm2'] = unit_matrix['total_irradiance_mw_cm2']['value']
        calculations['total_irradiance_w_cm2'] = unit_matrix['total_irradiance_w_cm2']['value']
        calculations['irradiance_source'] = f"{unit_matrix['total_irradiance_mw_cm2']['source']}: {calculations['total_irradiance_mw_cm2']:.3f} mW/cm²"
    
    if unit_matrix['per_well_irradiance_mw_cm2']['value'] > 0:
        calculations['per_well_irradiance_mw_cm2'] = unit_matrix['per_well_irradiance_mw_cm2']['value']
        calculations['per_well_irradiance_w_cm2'] = unit_matrix['per_well_irradiance_w_cm2']['value']
        calculations['per_well_irradiance_source'] = f"{unit_matrix['per_well_irradiance_mw_cm2']['source']}: {calculations['per_well_irradiance_mw_cm2']:.3f} mW/cm²"
    
    # Power calculations
    if unit_matrix['total_power_mw']['value'] > 0:
        calculations['calculated_total_power_mw'] = unit_matrix['total_power_mw']['value']
        calculations['calculated_total_power_w'] = unit_matrix['total_power_w']['value']
    
    # Current estimations
    if unit_matrix['total_current_ma']['value'] > 0 and 'ESTIMATED' in unit_matrix['total_current_ma']['source']:
        calculations['estimated_fire_current_ma'] = unit_matrix['total_current_ma']['value']
        calculations['fire_current_source'] = unit_matrix['total_current_ma']['source']
    
    # Average calculations
    if per_power_mw > 0:
        well_count = plate_geometry['well_count']
        well_area_cm2 = plate_geometry['well_area_cm2']
        total_well_area_cm2 = well_area_cm2 * well_count
        calculations['avg_well_irradiance_mw_cm2'] = (per_power_mw * well_count) / total_well_area_cm2
    
    # Power density
    if total_power_mw > 0:
        total_area_cm2 = plate_geometry['total_area_cm2']
        calculations['power_density_mw_cm2'] = total_power_mw / total_area_cm2
        calculations['power_density_w_m2'] = (total_power_mw / 1000) / (total_area_cm2 / 10000)
      # Unit conversion matrix (enhanced)
    if total_power_mw > 0 or per_power_mw > 0:
        calculations['unit_conversions'] = {}
        
        if total_power_mw > 0:
            calculations['unit_conversions'].update({
                'total_power_w': unit_matrix['total_power_w']['value'],
                'total_power_mw': unit_matrix['total_power_mw']['value'],
                'total_irradiance_mw_cm2': unit_matrix['total_irradiance_mw_cm2']['value'],
                'total_irradiance_w_cm2': unit_matrix['total_irradiance_w_cm2']['value'],
            })
        
        if per_power_mw > 0:
            calculations['unit_conversions'].update({
                'per_well_power_w': unit_matrix['per_power_w']['value'],
                'per_well_power_mw': unit_matrix['per_power_mw']['value'],
                'per_well_irradiance_mw_cm2': unit_matrix['per_well_irradiance_mw_cm2']['value'],
                'per_well_irradiance_w_cm2': unit_matrix['per_well_irradiance_w_cm2']['value'],
            })
    
    # Enhanced data quality assessment
    calculations['data_quality'] = {
        'has_direct_power_reading': unit_matrix['total_power_mw']['confidence'] in ['VERY_HIGH', 'HIGH'] and 'DEVICE_DIRECT' in unit_matrix['total_power_mw']['source'],
        'has_direct_irradiance_reading': unit_matrix['total_irradiance_mw_cm2']['confidence'] in ['VERY_HIGH', 'HIGH'] and 'DEVICE_DIRECT' in unit_matrix['total_irradiance_mw_cm2']['source'],
        'has_current_reading': unit_matrix['total_current_ma']['confidence'] in ['VERY_HIGH', 'HIGH'],
        'has_per_well_data': unit_matrix['per_power_mw']['value'] > 0,
        'power_calculated_from_current': 'ESTIMATED_FROM_CURRENT' in unit_matrix['total_power_mw']['source'],
        'irradiance_calculated': unit_matrix['total_irradiance_mw_cm2']['value'] > 0,
        'overall_confidence': enhanced_calculations['overall_confidence']
    }
    
    # Store enhanced calculations for advanced analysis
    calculations['enhanced'] = enhanced_calculations
    
    return calculations

def estimate_led_efficiency(current_ma, led_type="generic"):
    """
    Enhanced LED efficiency estimation based on current level and LED type
    Returns efficiency in mW/mA based on typical LED characteristics
    """
    # Different LED types have different efficiency curves
    efficiency_curves = {
        "generic": {
            "base_efficiency": 0.5,
            "efficiency_ranges": [
                (0, 50, 0.8),      # Very low current: high efficiency
                (50, 200, 0.6),    # Low current: good efficiency  
                (200, 500, 0.5),   # Medium current: moderate efficiency
                (500, 1000, 0.4),  # High current: lower efficiency
                (1000, float('inf'), 0.3)  # Very high current: poor efficiency
            ]
        },
        "high_power": {
            "base_efficiency": 0.7,
            "efficiency_ranges": [
                (0, 100, 1.0),
                (100, 300, 0.8),
                (300, 700, 0.7),
                (700, 1500, 0.6),
                (1500, float('inf'), 0.5)
            ]
        },
        "uv_led": {
            "base_efficiency": 0.3,  # UV LEDs typically less efficient
            "efficiency_ranges": [
                (0, 50, 0.4),
                (50, 150, 0.3),
                (150, 400, 0.25),
                (400, 800, 0.2),
                (800, float('inf'), 0.15)
            ]
        }
    }
    
    curve = efficiency_curves.get(led_type, efficiency_curves["generic"])
    
    # Find appropriate efficiency range
    for min_current, max_current, efficiency in curve["efficiency_ranges"]:
        if min_current <= current_ma < max_current:
            return efficiency
    
    return curve["base_efficiency"]

def detect_all_unit_types(power_info):
    """
    Comprehensive detection of all possible unit types from device readings
    Returns a dictionary of detected units with confidence levels
    """
    detected_units = {
        'total_power': {'available': [], 'missing': []},
        'per_power': {'available': [], 'missing': []},
        'current': {'available': [], 'missing': []},
        'irradiance': {'available': [], 'missing': []},
        'confidence': 'UNKNOWN'
    }
    
    total_power = power_info.get('total_power', 0)
    per_power = power_info.get('per_power', 0)
    total_units = power_info.get('total_units', '')
    per_units = power_info.get('per_units', '')
    fire_current = power_info.get('fire_current_ma', 0)
    arm_current = power_info.get('arm_current_ma', 0)
    
    # Detect available total power units
    if 'W TOTAL' in total_units and 'mW' not in total_units:
        detected_units['total_power']['available'].append(('W', total_power, 'DIRECT'))
        detected_units['total_power']['available'].append(('mW', total_power * 1000, 'CONVERTED'))
    elif 'mW TOTAL' in total_units:
        detected_units['total_power']['available'].append(('mW', total_power, 'DIRECT'))
        detected_units['total_power']['available'].append(('W', total_power / 1000, 'CONVERTED'))
    elif 'mW/cm² TOTAL' in total_units or 'W/cm² TOTAL' in total_units:
        detected_units['irradiance']['available'].append(('total_irradiance', total_power, 'DIRECT'))
    elif 'A TOTAL' in total_units:
        detected_units['current']['available'].append(('A', total_power, 'DIRECT'))
        detected_units['current']['available'].append(('mA', total_power * 1000, 'CONVERTED'))
    elif 'mA TOTAL' in total_units:
        detected_units['current']['available'].append(('mA', total_power, 'DIRECT'))
        detected_units['current']['available'].append(('A', total_power / 1000, 'CONVERTED'))
    
    # Detect available per-power units
    if 'W PER' in per_units and 'mW' not in per_units:
        detected_units['per_power']['available'].append(('W', per_power, 'DIRECT'))
        detected_units['per_power']['available'].append(('mW', per_power * 1000, 'CONVERTED'))
    elif 'mW PER' in per_units:
        detected_units['per_power']['available'].append(('mW', per_power, 'DIRECT'))
        detected_units['per_power']['available'].append(('W', per_power / 1000, 'CONVERTED'))
    elif 'mW/cm² PER' in per_units:
        detected_units['irradiance']['available'].append(('per_well_irradiance', per_power, 'DIRECT'))
    elif 'mW/cm²' == per_units.strip():
        detected_units['irradiance']['available'].append(('total_irradiance', per_power, 'DIRECT'))
    elif 'A PER' in per_units:
        detected_units['current']['available'].append(('A_per', per_power, 'DIRECT'))
    elif 'mA PER' in per_units:
        detected_units['current']['available'].append(('mA_per', per_power, 'DIRECT'))
    
    # Detect current readings
    if fire_current > 0:
        detected_units['current']['available'].append(('mA_fire', fire_current, 'DIRECT'))
        detected_units['current']['available'].append(('A_fire', fire_current / 1000, 'CONVERTED'))
    if arm_current > 0:
        detected_units['current']['available'].append(('mA_arm', arm_current, 'DIRECT'))
        detected_units['current']['available'].append(('A_arm', arm_current / 1000, 'CONVERTED'))
    
    # Determine confidence level
    has_direct_power = any(source == 'DIRECT' for _, _, source in detected_units['total_power']['available'])
    has_direct_irradiance = any(source == 'DIRECT' for _, _, source in detected_units['irradiance']['available'])
    has_current = len(detected_units['current']['available']) > 0
    
    if has_direct_power or has_direct_irradiance:
        detected_units['confidence'] = 'HIGH'
    elif has_current:
        detected_units['confidence'] = 'MEDIUM'
    else:
        detected_units['confidence'] = 'LOW'
    
    # Identify missing unit types
    all_power_types = ['W', 'mW', 'W/cm²', 'mW/cm²', 'A', 'mA']
    available_types = set(unit_type for unit_type, _, _ in 
                         detected_units['total_power']['available'] + 
                         detected_units['per_power']['available'] + 
                         detected_units['current']['available'])
    
    for unit_type in all_power_types:
        if unit_type not in available_types:
            detected_units['total_power']['missing'].append(unit_type)
    
    return detected_units

def calculate_all_possible_units(power_info, plate_geometry, led_type="generic"):
    """
    Calculate ALL possible unit representations with enhanced confidence scoring
    """
    calculations = {}
    
    # Get detected units
    detected = detect_all_unit_types(power_info)
    calculations['detected_units'] = detected
    
    # Extract values
    total_power = power_info['total_power']
    per_power = power_info['per_power']
    total_units = power_info['total_units']
    per_units = power_info['per_units']
    fire_current_ma = power_info['fire_current_ma']
    arm_current_ma = power_info['arm_current_ma']
    
    # Plate geometry
    total_area_cm2 = plate_geometry['total_area_cm2']
    well_area_cm2 = plate_geometry['well_area_cm2']
    well_count = plate_geometry['well_count']
    
    # === COMPREHENSIVE UNIT CALCULATION ===
    
    # Initialize calculation matrix
    unit_matrix = {
        'total_power_w': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'total_power_mw': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'per_power_w': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'per_power_mw': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'total_irradiance_w_cm2': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'total_irradiance_mw_cm2': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'per_well_irradiance_w_cm2': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'per_well_irradiance_mw_cm2': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'total_current_a': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'total_current_ma': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'per_current_a': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
        'per_current_ma': {'value': 0, 'source': 'NONE', 'confidence': 'NONE'},
    }
    
    # === DIRECT MEASUREMENTS (Highest Confidence) ===
    
    if "W TOTAL" in total_units and "mW" not in total_units:
        unit_matrix['total_power_w'] = {'value': total_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['total_power_mw'] = {'value': total_power * 1000, 'source': 'CONVERTED_W', 'confidence': 'HIGH'}
    elif "mW TOTAL" in total_units:
        unit_matrix['total_power_mw'] = {'value': total_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['total_power_w'] = {'value': total_power / 1000, 'source': 'CONVERTED_MW', 'confidence': 'HIGH'}
    elif "W/cm² TOTAL" in total_units:
        unit_matrix['total_irradiance_w_cm2'] = {'value': total_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['total_irradiance_mw_cm2'] = {'value': total_power * 1000, 'source': 'CONVERTED_W_CM2', 'confidence': 'HIGH'}
        unit_matrix['total_power_w'] = {'value': total_power * total_area_cm2, 'source': 'CALCULATED_FROM_IRRADIANCE', 'confidence': 'HIGH'}
        unit_matrix['total_power_mw'] = {'value': total_power * total_area_cm2 * 1000, 'source': 'CALCULATED_FROM_IRRADIANCE', 'confidence': 'HIGH'}
    elif "mW/cm² TOTAL" in total_units:
        unit_matrix['total_irradiance_mw_cm2'] = {'value': total_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['total_irradiance_w_cm2'] = {'value': total_power / 1000, 'source': 'CONVERTED_MW_CM2', 'confidence': 'HIGH'}
        unit_matrix['total_power_mw'] = {'value': total_power * total_area_cm2, 'source': 'CALCULATED_FROM_IRRADIANCE', 'confidence': 'HIGH'}
        unit_matrix['total_power_w'] = {'value': (total_power * total_area_cm2) / 1000, 'source': 'CALCULATED_FROM_IRRADIANCE', 'confidence': 'HIGH'}
    elif "A TOTAL" in total_units:
        unit_matrix['total_current_a'] = {'value': total_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['total_current_ma'] = {'value': total_power * 1000, 'source': 'CONVERTED_A', 'confidence': 'HIGH'}
    elif "mA TOTAL" in total_units:
        unit_matrix['total_current_ma'] = {'value': total_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['total_current_a'] = {'value': total_power / 1000, 'source': 'CONVERTED_MA', 'confidence': 'HIGH'}
    
    # === PER-WELL DIRECT MEASUREMENTS ===
    
    if "W PER" in per_units and "mW" not in per_units:
        unit_matrix['per_power_w'] = {'value': per_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['per_power_mw'] = {'value': per_power * 1000, 'source': 'CONVERTED_W', 'confidence': 'HIGH'}
    elif "mW PER" in per_units:
        unit_matrix['per_power_mw'] = {'value': per_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['per_power_w'] = {'value': per_power / 1000, 'source': 'CONVERTED_MW', 'confidence': 'HIGH'}
    elif "mW/cm² PER" in per_units:
        unit_matrix['per_well_irradiance_mw_cm2'] = {'value': per_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['per_well_irradiance_w_cm2'] = {'value': per_power / 1000, 'source': 'CONVERTED_MW_CM2', 'confidence': 'HIGH'}
        unit_matrix['per_power_mw'] = {'value': per_power * well_area_cm2, 'source': 'CALCULATED_FROM_IRRADIANCE', 'confidence': 'HIGH'}
        unit_matrix['per_power_w'] = {'value': (per_power * well_area_cm2) / 1000, 'source': 'CALCULATED_FROM_IRRADIANCE', 'confidence': 'HIGH'}
    elif "mW/cm²" == per_units.strip():
        # This is total irradiance displayed in per-unit field
        unit_matrix['total_irradiance_mw_cm2'] = {'value': per_power, 'source': 'DEVICE_DIRECT', 'confidence': 'VERY_HIGH'}
        unit_matrix['total_irradiance_w_cm2'] = {'value': per_power / 1000, 'source': 'CONVERTED_MW_CM2', 'confidence': 'HIGH'}
    
    # === CURRENT MEASUREMENTS ===
    
    if fire_current_ma > 0:
        unit_matrix['total_current_ma'] = {'value': fire_current_ma, 'source': 'DEVICE_CURRENT', 'confidence': 'HIGH'}
        unit_matrix['total_current_a'] = {'value': fire_current_ma / 1000, 'source': 'CONVERTED_MA', 'confidence': 'HIGH'}
        unit_matrix['per_current_ma'] = {'value': fire_current_ma / well_count, 'source': 'CALCULATED_FROM_TOTAL', 'confidence': 'MEDIUM'}
        unit_matrix['per_current_a'] = {'value': (fire_current_ma / well_count) / 1000, 'source': 'CALCULATED_FROM_TOTAL', 'confidence': 'MEDIUM'}
    
    # === CROSS-CALCULATIONS AND ESTIMATIONS ===
    
    # Calculate totals from per-well data
    if unit_matrix['per_power_mw']['value'] > 0 and unit_matrix['total_power_mw']['value'] == 0:
        unit_matrix['total_power_mw'] = {'value': unit_matrix['per_power_mw']['value'] * well_count, 'source': 'CALCULATED_FROM_PER_WELL', 'confidence': 'HIGH'}
        unit_matrix['total_power_w'] = {'value': unit_matrix['total_power_mw']['value'] / 1000, 'source': 'CALCULATED_FROM_PER_WELL', 'confidence': 'HIGH'}
    
    # Calculate per-well from totals
    if unit_matrix['total_power_mw']['value'] > 0 and unit_matrix['per_power_mw']['value'] == 0:
        unit_matrix['per_power_mw'] = {'value': unit_matrix['total_power_mw']['value'] / well_count, 'source': 'CALCULATED_FROM_TOTAL', 'confidence': 'HIGH'}
        unit_matrix['per_power_w'] = {'value': unit_matrix['per_power_mw']['value'] / 1000, 'source': 'CALCULATED_FROM_TOTAL', 'confidence': 'HIGH'}
    
    # Calculate irradiance from power
    if unit_matrix['total_power_mw']['value'] > 0 and unit_matrix['total_irradiance_mw_cm2']['value'] == 0:
        unit_matrix['total_irradiance_mw_cm2'] = {'value': unit_matrix['total_power_mw']['value'] / total_area_cm2, 'source': 'CALCULATED_FROM_POWER', 'confidence': 'HIGH'}
        unit_matrix['total_irradiance_w_cm2'] = {'value': unit_matrix['total_irradiance_mw_cm2']['value'] / 1000, 'source': 'CALCULATED_FROM_POWER', 'confidence': 'HIGH'}
    
    if unit_matrix['per_power_mw']['value'] > 0 and unit_matrix['per_well_irradiance_mw_cm2']['value'] == 0:
        unit_matrix['per_well_irradiance_mw_cm2'] = {'value': unit_matrix['per_power_mw']['value'] / well_area_cm2, 'source': 'CALCULATED_FROM_POWER', 'confidence': 'HIGH'}
        unit_matrix['per_well_irradiance_w_cm2'] = {'value': unit_matrix['per_well_irradiance_mw_cm2']['value'] / 1000, 'source': 'CALCULATED_FROM_POWER', 'confidence': 'HIGH'}
    
    # === POWER ESTIMATION FROM CURRENT ===
    
    if (unit_matrix['total_power_mw']['value'] == 0 and 
        unit_matrix['total_current_ma']['value'] > 0):
        
        efficiency = estimate_led_efficiency(unit_matrix['total_current_ma']['value'], led_type)
        estimated_power_mw = unit_matrix['total_current_ma']['value'] * efficiency
        
        unit_matrix['total_power_mw'] = {'value': estimated_power_mw, 'source': f'ESTIMATED_FROM_CURRENT(eff={efficiency:.2f})', 'confidence': 'MEDIUM'}
        unit_matrix['total_power_w'] = {'value': estimated_power_mw / 1000, 'source': f'ESTIMATED_FROM_CURRENT(eff={efficiency:.2f})', 'confidence': 'MEDIUM'}
        unit_matrix['per_power_mw'] = {'value': estimated_power_mw / well_count, 'source': 'CALCULATED_FROM_ESTIMATED_TOTAL', 'confidence': 'MEDIUM'}
        unit_matrix['per_power_w'] = {'value': (estimated_power_mw / well_count) / 1000, 'source': 'CALCULATED_FROM_ESTIMATED_TOTAL', 'confidence': 'MEDIUM'}
        
        # Calculate irradiance from estimated power
        unit_matrix['total_irradiance_mw_cm2'] = {'value': estimated_power_mw / total_area_cm2, 'source': 'CALCULATED_FROM_ESTIMATED_POWER', 'confidence': 'MEDIUM'}
        unit_matrix['total_irradiance_w_cm2'] = {'value': (estimated_power_mw / total_area_cm2) / 1000, 'source': 'CALCULATED_FROM_ESTIMATED_POWER', 'confidence': 'MEDIUM'}
        unit_matrix['per_well_irradiance_mw_cm2'] = {'value': (estimated_power_mw / well_count) / well_area_cm2, 'source': 'CALCULATED_FROM_ESTIMATED_POWER', 'confidence': 'MEDIUM'}
        unit_matrix['per_well_irradiance_w_cm2'] = {'value': ((estimated_power_mw / well_count) / well_area_cm2) / 1000, 'source': 'CALCULATED_FROM_ESTIMATED_POWER', 'confidence': 'MEDIUM'}
    
    # === CURRENT ESTIMATION FROM POWER ===
    
    if (unit_matrix['total_current_ma']['value'] == 0 and 
        unit_matrix['total_power_mw']['value'] > 0):
        
        efficiency = estimate_led_efficiency(500, led_type)  # Use middle estimate for reverse calculation
        estimated_current_ma = unit_matrix['total_power_mw']['value'] / efficiency
        
        unit_matrix['total_current_ma'] = {'value': estimated_current_ma, 'source': f'ESTIMATED_FROM_POWER(eff={efficiency:.2f})', 'confidence': 'MEDIUM'}
        unit_matrix['total_current_a'] = {'value': estimated_current_ma / 1000, 'source': f'ESTIMATED_FROM_POWER(eff={efficiency:.2f})', 'confidence': 'MEDIUM'}
        unit_matrix['per_current_ma'] = {'value': estimated_current_ma / well_count, 'source': 'CALCULATED_FROM_ESTIMATED_TOTAL', 'confidence': 'MEDIUM'}
        unit_matrix['per_current_a'] = {'value': (estimated_current_ma / well_count) / 1000, 'source': 'CALCULATED_FROM_ESTIMATED_TOTAL', 'confidence': 'MEDIUM'}
    
    # Store final results
    calculations['unit_matrix'] = unit_matrix
    calculations['led_type'] = led_type
    calculations['plate_geometry'] = plate_geometry
    
    # Calculate overall confidence
    confidence_levels = [entry['confidence'] for entry in unit_matrix.values() if entry['confidence'] != 'NONE']
    if 'VERY_HIGH' in confidence_levels:
        calculations['overall_confidence'] = 'VERY_HIGH'
    elif 'HIGH' in confidence_levels:
        calculations['overall_confidence'] = 'HIGH'
    elif 'MEDIUM' in confidence_levels:
        calculations['overall_confidence'] = 'MEDIUM'
    else:
        calculations['overall_confidence'] = 'LOW'
    
    return calculations

def detect_led_type(power_info_list, plate_geometry):
    """
    Intelligent LED type detection based on power characteristics
    Returns estimated LED type for better efficiency calculations
    """
    # Analyze power characteristics across all stages
    total_power_readings = []
    current_readings = []
    efficiency_estimates = []
    
    for power_info in power_info_list:
        if power_info.get('fire_current_ma', 0) > 0 and power_info.get('total_power', 0) > 0:
            current_ma = power_info['fire_current_ma']
            
            # Convert power to mW if needed
            total_power_mw = 0
            if 'mW TOTAL' in power_info.get('total_units', ''):
                total_power_mw = power_info['total_power']
            elif 'W TOTAL' in power_info.get('total_units', ''):
                total_power_mw = power_info['total_power'] * 1000
            elif 'mW/cm² TOTAL' in power_info.get('total_units', ''):
                total_power_mw = power_info['total_power'] * plate_geometry['total_area_cm2']
            
            if total_power_mw > 0:
                efficiency = total_power_mw / current_ma
                efficiency_estimates.append(efficiency)
                current_readings.append(current_ma)
                total_power_readings.append(total_power_mw)
    
    if not efficiency_estimates:
        return "generic", 0.5, "No power/current data available"
    
    avg_efficiency = sum(efficiency_estimates) / len(efficiency_estimates)
    avg_current = sum(current_readings) / len(current_readings)
    total_device_power = sum(total_power_readings)
    
    # LED type classification based on efficiency and power characteristics
    confidence_factors = []
    
    if avg_efficiency > 0.8:
        led_type = "high_power"
        confidence_factors.append("High efficiency indicates high-power LEDs")
    elif avg_efficiency < 0.3:
        led_type = "uv_led"
        confidence_factors.append("Low efficiency suggests UV LEDs")
    else:
        led_type = "generic"
        confidence_factors.append("Moderate efficiency, generic LED characteristics")
    
    # Additional classification based on current levels
    if avg_current > 500:
        if led_type == "generic":
            led_type = "high_power"
        confidence_factors.append("High current operation")
    elif avg_current < 50:
        confidence_factors.append("Low current operation")
    
    # Power density analysis
    total_area_cm2 = plate_geometry['total_area_cm2']
    power_density = total_device_power / total_area_cm2
    
    if power_density > 50:
        confidence_factors.append("High power density configuration")
        if led_type == "generic":
            led_type = "high_power"
    elif power_density < 5:
        confidence_factors.append("Low power density configuration")
    
    # Confidence assessment
    if len(efficiency_estimates) >= 3:
        confidence = "HIGH"
    elif len(efficiency_estimates) >= 2:
        confidence = "MEDIUM"  
    else:
        confidence = "LOW"
    
    analysis_summary = {
        'detected_type': led_type,
        'avg_efficiency': avg_efficiency,
        'confidence': confidence,
        'factors': confidence_factors,
        'sample_count': len(efficiency_estimates),
        'avg_current': avg_current,
        'power_density': power_density
    }
    
    return led_type, avg_efficiency, analysis_summary

def displayLedTypeAnalysis(all_stages_info, plate_geometry):
    """Display LED type analysis and recommendations"""
    print(f"\n" + "🔬"*40)
    print("                LED TYPE ANALYSIS")
    print("🔬"*40)
    
    led_type, avg_efficiency, analysis = detect_led_type(all_stages_info, plate_geometry)
    
    print(f"\n🔍 LED TYPE DETECTION:")
    print(f"   Detected Type: {led_type.upper()}")
    print(f"   Average Efficiency: {avg_efficiency:.3f} mW/mA")
    print(f"   Confidence: {analysis['confidence']}")
    print(f"   Sample Count: {analysis['sample_count']} stages")
    
    print(f"\n📊 POWER CHARACTERISTICS:")
    print(f"   Average Current: {analysis['avg_current']:.1f} mA")
    print(f"   Power Density: {analysis['power_density']:.2f} mW/cm²")
    
    print(f"\n💡 ANALYSIS FACTORS:")
    for factor in analysis['factors']:
        print(f"   • {factor}")
    
    print(f"\n🎯 LED TYPE RECOMMENDATIONS:")
    if led_type == "high_power":
        print(f"   ⚡ High-power LED configuration detected")
        print(f"   💡 Optimized for high-intensity applications")
        print(f"   🌡️  Monitor thermal management during extended operation")
        print(f"   📈 Efficiency: {avg_efficiency:.3f} mW/mA (typical: 0.7-1.0)")
    elif led_type == "uv_led":
        print(f"   🟣 UV LED configuration detected") 
        print(f"   ⚠️  Lower efficiency typical for UV spectrum")
        print(f"   🔬 Ideal for UV-specific applications")
        print(f"   📈 Efficiency: {avg_efficiency:.3f} mW/mA (typical: 0.2-0.4)")
    else:
        print(f"   💡 Generic LED configuration")
        print(f"   ⚖️  Balanced power and efficiency")
        print(f"   🔧 Suitable for general-purpose applications")
        print(f"   📈 Efficiency: {avg_efficiency:.3f} mW/mA (typical: 0.4-0.7)")
    
    if analysis['confidence'] == "LOW":
        print(f"\n⚠️  LOW CONFIDENCE WARNING:")
        print(f"   📊 Limited data available ({analysis['sample_count']} samples)")
        print(f"   🔧 More power/current measurements needed for accurate classification")
        print(f"   💡 Using conservative efficiency estimates")
    
    print("🔬"*40)
    
    return led_type, analysis

def get_plate_geometry():
    """Get plate geometry from the schematic"""
    # Based on the actual schematic dimensions (LUMIDOX II PROPRIETARY schematic)
    plate_length_mm = 127.75  # From schematic
    plate_width_mm = 105.5    # From schematic
    
    # Convert to cm
    plate_length_cm = plate_length_mm / 10
    plate_width_cm = plate_width_mm / 10
    
    # Calculate total area
    total_area_cm2 = plate_length_cm * plate_width_cm
    
    # From schematic: 96-well plate (8x12 grid) - confirmed from schematic layout
    well_count = 96
    
    # Actual values from schematic (not estimates)
    well_spacing_mm = 9.0  # Standard 96-well spacing, confirmed by schematic grid
    well_diameter_mm = 5.0  # From schematic: "∅5.0 (96 PLACES)"    
    well_area_cm2 = 3.14159 * (well_diameter_mm/20)**2  # Convert to cm² and calculate circle area
    
    # NOTE: Using actual schematic diameter (5.0mm) instead of typical estimate (6.5mm)
    # This results in ~1.69x higher per-well irradiance calculations (more accurate)
    
    return {
        'plate_length_cm': plate_length_cm,
        'plate_width_cm': plate_width_cm,
        'total_area_cm2': total_area_cm2,
        'well_count': well_count,
        'well_area_cm2': well_area_cm2,
        'well_spacing_mm': well_spacing_mm,
        'well_diameter_mm': well_diameter_mm
    }

def displayCalculatedUnits(stage_info, calculations, plate_geometry):
    """Display calculated unit types with smart analysis"""
    stage = stage_info['stage']
    
    print(f"\n" + "="*80)
    print(f"                 SMART UNIT ANALYSIS FOR STAGE {stage}")
    print("="*80)
    
    print(f"\nPLATE GEOMETRY (from schematic):")
    print(f"  Plate size: {plate_geometry['plate_length_cm']:.2f} x {plate_geometry['plate_width_cm']:.2f} cm")
    print(f"  Total area: {plate_geometry['total_area_cm2']:.2f} cm²")
    print(f"  Well count: {plate_geometry['well_count']}")
    print(f"  Well area: {plate_geometry['well_area_cm2']:.3f} cm² per well")
    print(f"  Well diameter: {plate_geometry['well_diameter_mm']:.1f} mm (from schematic)")
    
    print(f"\nDEVICE READINGS:")
    print(f"  Total Power: {stage_info['total_power']:.1f} {stage_info['total_units']}")
    print(f"  Per Well: {stage_info['per_power']:.1f} {stage_info['per_units']}")
    print(f"  FIRE Current: {stage_info['fire_current_ma']} mA")
    print(f"  ARM Current: {stage_info['arm_current_ma']} mA")
    print(f"  Unit Indices: Total={stage_info['total_units_index']}, Per={stage_info['per_units_index']}")
    
    # Display data quality assessment
    if 'data_quality' in calculations:
        quality = calculations['data_quality']
        print(f"\nDATA QUALITY ASSESSMENT:")
        print(f"  ✓ Direct power reading: {'Yes' if quality['has_direct_power_reading'] else 'No'}")
        print(f"  ✓ Direct irradiance reading: {'Yes' if quality['has_direct_irradiance_reading'] else 'No'}")
        print(f"  ✓ Current reading available: {'Yes' if quality['has_current_reading'] else 'No'}")
        print(f"  ✓ Per-well data available: {'Yes' if quality['has_per_well_data'] else 'No'}")
        print(f"  ✓ Power calculated from current: {'Yes' if quality['power_calculated_from_current'] else 'No'}")
        print(f"  ✓ Irradiance calculated: {'Yes' if quality['irradiance_calculated'] else 'No'}")
    
    # Display data sources
    print(f"\nDATA SOURCES & CALCULATIONS:")
    if 'total_power_source' in calculations:
        print(f"  📊 Total Power: {calculations['total_power_source']}")
    if 'per_power_source' in calculations:
        print(f"  📊 Per-Well Power: {calculations['per_power_source']}")
    if 'irradiance_source' in calculations:
        print(f"  📊 Total Irradiance: {calculations['irradiance_source']}")
    if 'per_well_irradiance_source' in calculations:
        print(f"  📊 Per-Well Irradiance: {calculations['per_well_irradiance_source']}")
    if 'fire_current_source' in calculations:
        print(f"  📊 FIRE Current: {calculations['fire_current_source']}")
    if 'arm_current_source' in calculations:
        print(f"  📊 ARM Current: {calculations['arm_current_source']}")
    
    print(f"\nCALCULATED IRRADIANCE VALUES:")
    
    if 'total_irradiance_mw_cm2' in calculations:
        print(f"  🎯 Total Irradiance: {calculations['total_irradiance_mw_cm2']:.3f} mW/cm² ⭐")
        print(f"      Total Irradiance: {calculations['total_irradiance_w_cm2']:.6f} W/cm²")
    
    if 'per_well_irradiance_mw_cm2' in calculations:
        print(f"  🎯 Per Well Irradiance: {calculations['per_well_irradiance_mw_cm2']:.3f} mW/cm²")
        print(f"      Per Well Irradiance: {calculations['per_well_irradiance_w_cm2']:.6f} W/cm²")
    
    if 'avg_well_irradiance_mw_cm2' in calculations:
        print(f"  🎯 Average Well Irradiance: {calculations['avg_well_irradiance_mw_cm2']:.3f} mW/cm²")
    
    print(f"\nCALCULATED POWER VALUES:")
    
    if 'calculated_total_power_mw' in calculations:
        print(f"  ⚡ Calculated Total (from per-well): {calculations['calculated_total_power_mw']:.1f} mW")
        print(f"      Calculated Total (from per-well): {calculations['calculated_total_power_w']:.3f} W")
    
    if 'power_density_mw_cm2' in calculations:
        print(f"  ⚡ Power Density: {calculations['power_density_mw_cm2']:.3f} mW/cm²")
        print(f"      Power Density: {calculations['power_density_w_m2']:.1f} W/m²")
    
    # Display estimated currents if available
    if 'estimated_fire_current_ma' in calculations:
        print(f"\nESTIMATED CURRENT VALUES:")
        print(f"  🔌 Estimated FIRE Current: {calculations['estimated_fire_current_ma']:.0f} mA")
    if 'estimated_arm_current_ma' in calculations:
        print(f"  🔌 Estimated ARM Current: {calculations['estimated_arm_current_ma']:.0f} mA")
    
    # Display complete unit conversion matrix
    if 'unit_conversions' in calculations:
        print(f"\nCOMPLETE UNIT CONVERSION MATRIX:")
        conversions = calculations['unit_conversions']
        if 'total_power_w' in conversions:
            print(f"  🔄 Total Power: {conversions['total_power_w']:.3f} W = {conversions['total_power_mw']:.1f} mW")
        if 'total_irradiance_mw_cm2' in conversions:
            print(f"  🔄 Total Irradiance: {conversions['total_irradiance_mw_cm2']:.3f} mW/cm² = {conversions['total_irradiance_w_cm2']:.6f} W/cm²")
        if 'per_well_power_w' in conversions:
            print(f"  🔄 Per-Well Power: {conversions['per_well_power_w']:.4f} W = {conversions['per_well_power_mw']:.2f} mW")
        if 'per_well_irradiance_mw_cm2' in conversions:
            print(f"  🔄 Per-Well Irradiance: {conversions['per_well_irradiance_mw_cm2']:.3f} mW/cm² = {conversions['per_well_irradiance_w_cm2']:.6f} W/cm²")
    
    # Key result with confidence indicator
    confidence = "HIGH" if calculations.get('data_quality', {}).get('has_direct_power_reading', False) else "MEDIUM" if calculations.get('data_quality', {}).get('has_current_reading', False) else "LOW"
    
    print(f"\n⭐ KEY RESULT (Confidence: {confidence}):")
    if 'total_irradiance_mw_cm2' in calculations:
        print(f"   If this stage were configured with mW/cm² total irradiance units,")
        print(f"   it would read approximately {calculations['total_irradiance_mw_cm2']:.3f} mW/cm²")
    else:
        print(f"   Insufficient data to calculate total irradiance")
    
    if confidence == "LOW":
        print(f"   ⚠️  Low confidence: Based on estimates from limited data")
    elif confidence == "MEDIUM":
        print(f"   ℹ️  Medium confidence: Calculated from current readings")
    else:
        print(f"   ✅ High confidence: Based on direct power measurements")
    
    print("="*80)

def displayEnhancedUnitAnalysis(stage_info, plate_geometry, led_type="generic"):
    """Display comprehensive unit analysis with enhanced confidence scoring"""
    stage = stage_info['stage']
    
    print(f"\n" + "🔬"*50)
    print(f"         COMPREHENSIVE UNIT ANALYSIS - STAGE {stage}")
    print("🔬"*50)
    
    # Get comprehensive calculations
    calculations = calculate_all_possible_units(stage_info, plate_geometry, led_type)
    unit_matrix = calculations['unit_matrix']
    detected_units = calculations['detected_units']
    
    # Display detection summary
    print(f"\n📊 UNIT DETECTION SUMMARY:")
    print(f"   Overall Confidence: {calculations['overall_confidence']}")
    print(f"   Detection Confidence: {detected_units['confidence']}")
    print(f"   LED Type Assumed: {led_type}")
    
    # Display what was detected
    print(f"\n🔍 DETECTED FROM DEVICE:")
    for unit_type, value, source in detected_units['total_power']['available']:
        print(f"   Total Power: {value:.3f} {unit_type} ({source})")
    for unit_type, value, source in detected_units['per_power']['available']:
        print(f"   Per-Well Power: {value:.3f} {unit_type} ({source})")
    for unit_type, value, source in detected_units['current']['available']:
        print(f"   Current: {value:.3f} {unit_type} ({source})")
    for unit_type, value, source in detected_units['irradiance']['available']:
        print(f"   Irradiance: {value:.3f} {unit_type} ({source})")
    
    # Display complete unit matrix
    print(f"\n📋 COMPLETE UNIT MATRIX:")
    print(f"   {'Unit Type':<25} {'Value':<12} {'Source':<25} {'Confidence':<12}")
    print(f"   {'-'*25} {'-'*12} {'-'*25} {'-'*12}")
    
    for unit_name, unit_data in unit_matrix.items():
        if unit_data['value'] > 0:
            confidence_color = {
                'VERY_HIGH': '🟢',
                'HIGH': '🟡', 
                'MEDIUM': '🟠',
                'LOW': '🔴',
                'NONE': '⚫'
            }.get(unit_data['confidence'], '⚫')
            
            print(f"   {unit_name:<25} {unit_data['value']:<12.4f} {unit_data['source']:<25} {confidence_color} {unit_data['confidence']}")
    
    # Display all possible device configurations
    print(f"\n⚙️  ALL POSSIBLE DEVICE CONFIGURATIONS:")
    
    # Total power configurations (indices 0-6)
    print(f"\n   📊 Total Power Options:")
    for i in range(7):
        if i == 4:  # Skip blank index
            print(f"      Index {i}: (BLANK)")
            continue
            
        value = 0
        unit_str = "UNKNOWN"
        confidence = "N/A"
        
        if i == 0 and unit_matrix['total_power_w']['value'] > 0:
            value = unit_matrix['total_power_w']['value']
            unit_str = "W TOTAL RADIANT POWER"
            confidence = unit_matrix['total_power_w']['confidence']
        elif i == 1 and unit_matrix['total_power_mw']['value'] > 0:
            value = unit_matrix['total_power_mw']['value']
            unit_str = "mW TOTAL RADIANT POWER"
            confidence = unit_matrix['total_power_mw']['confidence']
        elif i == 2 and unit_matrix['total_irradiance_w_cm2']['value'] > 0:
            value = unit_matrix['total_irradiance_w_cm2']['value']
            unit_str = "W/cm² TOTAL IRRADIANCE"
            confidence = unit_matrix['total_irradiance_w_cm2']['confidence']
        elif i == 3 and unit_matrix['total_irradiance_mw_cm2']['value'] > 0:
            value = unit_matrix['total_irradiance_mw_cm2']['value']
            unit_str = "mW/cm² TOTAL IRRADIANCE ⭐"
            confidence = unit_matrix['total_irradiance_mw_cm2']['confidence']
        elif i == 5 and unit_matrix['total_current_a']['value'] > 0:
            value = unit_matrix['total_current_a']['value']
            unit_str = "A TOTAL CURRENT"
            confidence = unit_matrix['total_current_a']['confidence']
        elif i == 6 and unit_matrix['total_current_ma']['value'] > 0:
            value = unit_matrix['total_current_ma']['value']
            unit_str = "mA TOTAL CURRENT"
            confidence = unit_matrix['total_current_ma']['confidence']
        
        if value > 0:
            confidence_indicator = {'VERY_HIGH': '🟢', 'HIGH': '🟡', 'MEDIUM': '🟠', 'LOW': '🔴'}.get(confidence, '⚫')
            print(f"      Index {i}: {value:.4f} {unit_str} {confidence_indicator}")
        else:
            print(f"      Index {i}: No data for {decodeTotalUnits(i)}")
    
    # Per-well configurations (indices 0-9)
    print(f"\n   📊 Per-Well Power Options:")
    for i in range(10):
        if i == 7:  # Skip blank index
            print(f"      Index {i}: (BLANK)")
            continue
            
        value = 0
        unit_str = "UNKNOWN"
        confidence = "N/A"
        
        if i == 0 and unit_matrix['per_power_w']['value'] > 0:
            value = unit_matrix['per_power_w']['value']
            unit_str = "W PER WELL"
            confidence = unit_matrix['per_power_w']['confidence']
        elif i == 1 and unit_matrix['per_power_mw']['value'] > 0:
            value = unit_matrix['per_power_mw']['value']
            unit_str = "mW PER WELL"
            confidence = unit_matrix['per_power_mw']['confidence']
        elif i == 2 and unit_matrix['total_power_w']['value'] > 0:
            value = unit_matrix['total_power_w']['value']
            unit_str = "W TOTAL RADIANT POWER"
            confidence = unit_matrix['total_power_w']['confidence']
        elif i == 3 and unit_matrix['total_power_mw']['value'] > 0:
            value = unit_matrix['total_power_mw']['value']
            unit_str = "mW TOTAL RADIANT POWER"
            confidence = unit_matrix['total_power_mw']['confidence']
        elif i == 4 and unit_matrix['per_well_irradiance_mw_cm2']['value'] > 0:
            value = unit_matrix['per_well_irradiance_mw_cm2']['value']
            unit_str = "mW/cm² PER WELL"
            confidence = unit_matrix['per_well_irradiance_mw_cm2']['confidence']
        elif i == 5 and unit_matrix['total_irradiance_mw_cm2']['value'] > 0:
            value = unit_matrix['total_irradiance_mw_cm2']['value']
            unit_str = "mW/cm² ⭐"
            confidence = unit_matrix['total_irradiance_mw_cm2']['confidence']
        elif i == 6 and unit_matrix['per_power_mw']['value'] > 0:
            value = unit_matrix['per_power_mw']['value']
            unit_str = "J/s (same as mW)"
            confidence = unit_matrix['per_power_mw']['confidence']
        elif i == 8 and unit_matrix['per_current_a']['value'] > 0:
            value = unit_matrix['per_current_a']['value']
            unit_str = "A PER WELL"
            confidence = unit_matrix['per_current_a']['confidence']
        elif i == 9 and unit_matrix['per_current_ma']['value'] > 0:
            value = unit_matrix['per_current_ma']['value']
            unit_str = "mA PER WELL"
            confidence = unit_matrix['per_current_ma']['confidence']
        
        if value > 0:
            confidence_indicator = {'VERY_HIGH': '🟢', 'HIGH': '🟡', 'MEDIUM': '🟠', 'LOW': '🔴'}.get(confidence, '⚫')
            print(f"      Index {i}: {value:.4f} {unit_str} {confidence_indicator}")
        else:
            print(f"      Index {i}: No data for {decodePerUnits(i)}")
    
    # Recommendations
    print(f"\n💡 SMART RECOMMENDATIONS:")
    
    current_total_index = stage_info['total_units_index']
    current_per_index = stage_info['per_units_index']
    
    if current_total_index == 3:
        print(f"   ✅ Stage {stage} already configured for mW/cm² total irradiance!")
        if unit_matrix['total_irradiance_mw_cm2']['value'] > 0:
            print(f"   📏 Current reading: {unit_matrix['total_irradiance_mw_cm2']['value']:.3f} mW/cm²")
    else:
        if unit_matrix['total_irradiance_mw_cm2']['value'] > 0:
            print(f"   🔧 To show mW/cm² total irradiance:")
            print(f"   📝 Set Total Units Index = 3")
            print(f"   📏 Expected reading: {unit_matrix['total_irradiance_mw_cm2']['value']:.3f} mW/cm²")
    
    if current_per_index == 5:
        print(f"   ✅ Stage {stage} per-unit field shows total mW/cm²!")
    elif current_per_index == 4:
        print(f"   ✅ Stage {stage} per-unit field shows per-well mW/cm²!")
    else:
        if unit_matrix['total_irradiance_mw_cm2']['value'] > 0:
            print(f"   🔧 To show mW/cm² in per-unit field:")
            print(f"   📝 Set Per Units Index = 5 (total) or 4 (per-well)")
    
    # Data quality assessment
    print(f"\n🎯 DATA QUALITY ASSESSMENT:")
    quality_indicators = {
        'VERY_HIGH': '🟢 VERY HIGH - Direct device measurement',
        'HIGH': '🟡 HIGH - Reliable calculation/conversion',
        'MEDIUM': '🟠 MEDIUM - Estimated from available data',
        'LOW': '🔴 LOW - Limited data, rough estimate'
    }
    
    print(f"   Overall: {quality_indicators.get(calculations['overall_confidence'], '⚫ UNKNOWN')}")
    
    # Key confidence factors
    key_units = ['total_irradiance_mw_cm2', 'total_power_mw', 'total_current_ma']
    for unit_name in key_units:
        unit_data = unit_matrix[unit_name]
        if unit_data['value'] > 0:
            confidence_desc = quality_indicators.get(unit_data['confidence'], '⚫ UNKNOWN')
            print(f"   {unit_name}: {confidence_desc}")
    
    print("🔬"*50)

def displaySmartRecommendations(all_stages_info, plate_geometry):
    """Display intelligent recommendations based on comprehensive analysis"""
    print(f"\n" + "🎯"*50)
    print("           INTELLIGENT CONFIGURATION RECOMMENDATIONS")
    print("🎯"*50)
    
    # Analyze all stages comprehensively
    stage_analyses = []
    total_device_irradiance = 0
    
    for stage_info in all_stages_info:
        if stage_info['total_power'] > 0 or stage_info['fire_current_ma'] > 0:
            analysis = calculate_all_possible_units(stage_info, plate_geometry)
            stage_analyses.append((stage_info, analysis))
            
            # Sum up total device irradiance
            if analysis['unit_matrix']['total_irradiance_mw_cm2']['value'] > 0:
                total_device_irradiance += analysis['unit_matrix']['total_irradiance_mw_cm2']['value']
    
    print(f"\n📊 DEVICE OVERVIEW:")
    print(f"   Active stages: {len(stage_analyses)}/5")
    print(f"   Total calculated irradiance: {total_device_irradiance:.3f} mW/cm²")
    
    # Intensity classification
    if total_device_irradiance > 100:
        intensity_level = "🔥 VERY HIGH"
    elif total_device_irradiance > 50:
        intensity_level = "🌟 HIGH" 
    elif total_device_irradiance > 10:
        intensity_level = "💡 MEDIUM"
    elif total_device_irradiance > 1:
        intensity_level = "🔅 LOW"
    else:
        intensity_level = "⚫ MINIMAL"
    
    print(f"   Intensity level: {intensity_level}")
    
    # Configuration optimization recommendations
    print(f"\n🔧 OPTIMIZATION RECOMMENDATIONS:")
    
    # Find best irradiance stages
    irradiance_stages = []
    power_stages = []
    current_stages = []
    
    for stage_info, analysis in stage_analyses:
        confidence = analysis['overall_confidence']
        irradiance_value = analysis['unit_matrix']['total_irradiance_mw_cm2']['value']
        
        if irradiance_value > 0:
            if confidence in ['VERY_HIGH', 'HIGH']:
                irradiance_stages.append((stage_info['stage'], irradiance_value, confidence))
            elif confidence == 'MEDIUM':
                power_stages.append((stage_info['stage'], irradiance_value, confidence))
            else:
                current_stages.append((stage_info['stage'], irradiance_value, confidence))
    
    # Sort by irradiance value
    irradiance_stages.sort(key=lambda x: x[1], reverse=True)
    power_stages.sort(key=lambda x: x[1], reverse=True)
    current_stages.sort(key=lambda x: x[1], reverse=True)
    
    if irradiance_stages:
        print(f"   🟢 HIGH CONFIDENCE stages for mW/cm² configuration:")
        for stage, irradiance, confidence in irradiance_stages:
            print(f"      Stage {stage}: {irradiance:.3f} mW/cm² ({confidence})")
            print(f"         → Set Total Units Index = 3")
    
    if power_stages:
        print(f"   🟡 MEDIUM CONFIDENCE stages:")
        for stage, irradiance, confidence in power_stages:
            print(f"      Stage {stage}: {irradiance:.3f} mW/cm² ({confidence})")
            print(f"         → Verify calibration, then set Index = 3")
    
    if current_stages:
        print(f"   🟠 LOW CONFIDENCE stages (current-based estimates):")
        for stage, irradiance, confidence in current_stages:
            print(f"      Stage {stage}: ~{irradiance:.3f} mW/cm² ({confidence})")
            print(f"         → Requires power calibration for accuracy")
    
    # Application-specific recommendations
    print(f"\n🧪 APPLICATION RECOMMENDATIONS:")
    
    if total_device_irradiance > 50:
        print(f"   🔬 High-intensity applications possible")
        print(f"   💡 Consider UV curing, phototherapy, or high-speed photoreactions")
        print(f"   ⚠️  Verify safety protocols for high-intensity UV exposure")
    elif total_device_irradiance > 10:
        print(f"   🧬 Medium-intensity biological applications")
        print(f"   💡 Suitable for cell culture, fluorescence activation")
        print(f"   📊 Good balance of power and uniformity")
    elif total_device_irradiance > 1:
        print(f"   🔬 Low-intensity precision applications")
        print(f"   💡 Ideal for sensitive biological assays")
        print(f"   📈 Consider longer exposure times for higher doses")
    else:
        print(f"   ⚠️  Very low irradiance detected")
        print(f"   🔧 Check device calibration and LED functionality")
        print(f"   📞 Contact technical support if readings seem incorrect")
    
    # Calibration recommendations
    print(f"\n🎯 CALIBRATION STATUS:")
    
    high_confidence_count = len(irradiance_stages)
    medium_confidence_count = len(power_stages)
    low_confidence_count = len(current_stages)
    
    if high_confidence_count >= 3:
        print(f"   ✅ Device well-calibrated ({high_confidence_count} high-confidence stages)")
        print(f"   💡 Ready for precision applications")
    elif high_confidence_count + medium_confidence_count >= 3:
        print(f"   🟡 Device partially calibrated")
        print(f"   🔧 Consider full calibration for optimal accuracy")
    else:
        print(f"   🔴 Device needs calibration")
        print(f"   📞 Contact technical support for calibration procedure")
        print(f"   📊 Current readings are estimates only")
    
    # Power distribution analysis
    if len(stage_analyses) > 1:
        irradiances = [analysis['unit_matrix']['total_irradiance_mw_cm2']['value'] 
                      for _, analysis in stage_analyses 
                      if analysis['unit_matrix']['total_irradiance_mw_cm2']['value'] > 0]
        
        if len(irradiances) > 1:
            max_irradiance = max(irradiances)
            min_irradiance = min(irradiances)
            uniformity = (min_irradiance / max_irradiance) * 100 if max_irradiance > 0 else 0;
            
            print(f"\n📊 POWER DISTRIBUTION:")
            print(f"   Range: {min_irradiance:.3f} - {max_irradiance:.3f} mW/cm²")
            print(f"   Uniformity: {uniformity:.1f}%")
            
            if uniformity > 90:
                print(f"   ✅ Excellent uniformity")
            elif uniformity > 75:
                print(f"   🟡 Good uniformity")
            elif uniformity > 50:
                print(f"   🟠 Moderate uniformity - consider balancing stages")
            else:
                print(f"   🔴 Poor uniformity - calibration recommended")
      print("🎯"*50)
    print(f"\nDATA SOURCES & CALCULATIONS:")
    if 'total_power_source' in calculations:
        print(f"  📊 Total Power: {calculations['total_power_source']}")
    if 'per_power_source' in calculations:
        print(f"  📊 Per-Well Power: {calculations['per_power_source']}")
    if 'irradiance_source' in calculations:
        print(f"  📊 Total Irradiance: {calculations['irradiance_source']}")
    if 'per_well_irradiance_source' in calculations:
        print(f"  📊 Per-Well Irradiance: {calculations['per_well_irradiance_source']}")
    if 'fire_current_source' in calculations:
        print(f"  📊 FIRE Current: {calculations['fire_current_source']}")
    if 'arm_current_source' in calculations:
        print(f"  📊 ARM Current: {calculations['arm_current_source']}")
    
    print(f"\nCALCULATED IRRADIANCE VALUES:")
    
    if 'total_irradiance_mw_cm2' in calculations:
        print(f"  🎯 Total Irradiance: {calculations['total_irradiance_mw_cm2']:.3f} mW/cm² ⭐")
        print(f"      Total Irradiance: {calculations['total_irradiance_w_cm2']:.6f} W/cm²")
    
    if 'per_well_irradiance_mw_cm2' in calculations:
        print(f"  🎯 Per Well Irradiance: {calculations['per_well_irradiance_mw_cm2']:.3f} mW/cm²")
        print(f"      Per Well Irradiance: {calculations['per_well_irradiance_w_cm2']:.6f} W/cm²")
    
    if 'avg_well_irradiance_mw_cm2' in calculations:
        print(f"  🎯 Average Well Irradiance: {calculations['avg_well_irradiance_mw_cm2']:.3f} mW/cm²")
    
    print(f"\nCALCULATED POWER VALUES:")
    
    if 'calculated_total_power_mw' in calculations:
        print(f"  ⚡ Calculated Total (from per-well): {calculations['calculated_total_power_mw']:.1f} mW")
        print(f"      Calculated Total (from per-well): {calculations['calculated_total_power_w']:.3f} W")
    
    if 'power_density_mw_cm2' in calculations:
        print(f"  ⚡ Power Density: {calculations['power_density_mw_cm2']:.3f} mW/cm²")
        print(f"      Power Density: {calculations['power_density_w_m2']:.1f} W/m²")
    
    # Display estimated currents if available
    if 'estimated_fire_current_ma' in calculations:
        print(f"\nESTIMATED CURRENT VALUES:")
        print(f"  🔌 Estimated FIRE Current: {calculations['estimated_fire_current_ma']:.0f} mA")
    if 'estimated_arm_current_ma' in calculations:
        print(f"  🔌 Estimated ARM Current: {calculations['estimated_arm_current_ma']:.0f} mA")
    
    # Display complete unit conversion matrix
    if 'unit_conversions' in calculations:
        print(f"\nCOMPLETE UNIT CONVERSION MATRIX:")
        conversions = calculations['unit_conversions']
        if 'total_power_w' in conversions:
            print(f"  🔄 Total Power: {conversions['total_power_w']:.3f} W = {conversions['total_power_mw']:.1f} mW")
        if 'total_irradiance_mw_cm2' in conversions:
            print(f"  🔄 Total Irradiance: {conversions['total_irradiance_mw_cm2']:.3f} mW/cm² = {conversions['total_irradiance_w_cm2']:.6f} W/cm²")
        if 'per_well_power_w' in conversions:
            print(f"  🔄 Per-Well Power: {conversions['per_well_power_w']:.4f} W = {conversions['per_well_power_mw']:.2f} mW")
        if 'per_well_irradiance_mw_cm2' in conversions:
            print(f"  🔄 Per-Well Irradiance: {conversions['per_well_irradiance_mw_cm2']:.3f} mW/cm² = {conversions['per_well_irradiance_w_cm2']:.6f} W/cm²")
    
    # Key result with confidence indicator
    confidence = "HIGH" if calculations.get('data_quality', {}).get('has_direct_power_reading', False) else "MEDIUM" if calculations.get('data_quality', {}).get('has_current_reading', False) else "LOW"
    
    print(f"\n⭐ KEY RESULT (Confidence: {confidence}):")
    if 'total_irradiance_mw_cm2' in calculations:
        print(f"   If this stage were configured with mW/cm² total irradiance units,")
        print(f"   it would read approximately {calculations['total_irradiance_mw_cm2']:.3f} mW/cm²")
    else:
        print(f"   Insufficient data to calculate total irradiance")
    
    if confidence == "LOW":
        print(f"   ⚠️  Low confidence: Based on estimates from limited data")
    elif confidence == "MEDIUM":
        print(f"   ℹ️  Medium confidence: Calculated from current readings")
    else:
        print(f"   ✅ High confidence: Based on direct power measurements")
    
    print("="*80)

def analyzeAllPossibleUnits(stage_info, plate_geometry):
    """Analyze and calculate all possible unit representations for a stage"""
    print(f"\n" + "🔬"*40)
    print(f"    COMPREHENSIVE UNIT ANALYSIS - STAGE {stage_info['stage']}")
    print("🔬"*40)
    
    # Get all available data using enhanced calculation
    enhanced_calculations = calculate_all_possible_units(stage_info, plate_geometry)
    
    # === WHAT THE DEVICE COULD SHOW FOR TOTAL POWER ===
    print(f"\n📊 TOTAL POWER UNIT OPTIONS (Index 0-6):")
    print(f"   What Stage {stage_info['stage']} could display as 'Total Power':")
    
    if 'unit_matrix' in enhanced_calculations:
        unit_matrix = enhanced_calculations['unit_matrix']
        
        print(f"   Index 0: {unit_matrix['total_power_w']['value']:.3f} W TOTAL RADIANT POWER")
        print(f"   Index 1: {unit_matrix['total_power_mw']['value']:.1f} mW TOTAL RADIANT POWER")
        print(f"   Index 2: {unit_matrix['total_irradiance_w_cm2']['value']:.6f} W/cm² TOTAL IRRADIANCE")
        print(f"   Index 3: {unit_matrix['total_irradiance_mw_cm2']['value']:.3f} mW/cm² TOTAL IRRADIANCE ⭐")
        print(f"   Index 4: (BLANK)")
        
        # Estimate current if not available
        if stage_info['fire_current_ma'] > 0:
            fire_current_a = stage_info['fire_current_ma'] / 1000
            print(f"   Index 5: {fire_current_a:.3f} A TOTAL CURRENT")
            print(f"   Index 6: {stage_info['fire_current_ma']} mA TOTAL CURRENT")
        elif 'total_current_ma' in unit_matrix:
            estimated_current_ma = unit_matrix['total_current_ma']['value']
            estimated_current_a = estimated_current_ma / 1000
            print(f"   Index 5: ~{estimated_current_a:.3f} A TOTAL CURRENT (estimated)")
            print(f"   Index 6: ~{estimated_current_ma:.0f} mA TOTAL CURRENT (estimated)")
        else:
            print(f"   Index 5: ? A TOTAL CURRENT (no current data)")
            print(f"   Index 6: ? mA TOTAL CURRENT (no current data)")
    else:
        print(f"   ❌ Cannot calculate - insufficient power data")
    
    # === WHAT THE DEVICE COULD SHOW FOR PER-WELL POWER ===
    print(f"\n📊 PER-WELL UNIT OPTIONS (Index 0-9):")
    print(f"   What Stage {stage_info['stage']} could display as 'Per-Well Power':")
    
    if 'unit_matrix' in enhanced_calculations:
        unit_matrix = enhanced_calculations['unit_matrix']
        
        print(f"   Index 0: {unit_matrix['per_power_w']['value']:.4f} W PER WELL")
        print(f"   Index 1: {unit_matrix['per_power_mw']['value']:.2f} mW PER WELL")
        print(f"   Index 2: {unit_matrix['total_power_w']['value']:.3f} W TOTAL RADIANT POWER")
        print(f"   Index 3: {unit_matrix['total_power_mw']['value']:.1f} mW TOTAL RADIANT POWER")
        print(f"   Index 4: {unit_matrix['per_well_irradiance_mw_cm2']['value']:.3f} mW/cm² PER WELL")
        print(f"   Index 5: {unit_matrix['total_irradiance_mw_cm2']['value']:.3f} mW/cm² ⭐")
        print(f"   Index 6: {unit_matrix['per_power_mw']['value']:.2f} J/s (same as mW)")
        print(f"   Index 7: (BLANK)")
        
        # Estimate per-well current
        per_well_current_ma = unit_matrix['per_power_mw']['value'] / 0.5  # Assume 0.5 mW/mA efficiency
        per_well_current_a = per_well_current_ma / 1000
        print(f"   Index 8: {per_well_current_a:.4f} A PER WELL (estimated)")
        print(f"   Index 9: {per_well_current_ma:.1f} mA PER WELL (estimated)")
    else:
        print(f"   ❌ Cannot calculate - insufficient power data")
    
    # === CONFIDENCE AND RECOMMENDATIONS ===
    print(f"\n💡 RECOMMENDATIONS:")
    
    if stage_info['total_units_index'] == 3:
        print(f"   ✅ Stage {stage_info['stage']} is already configured for mW/cm² total irradiance!")
        print(f"   📏 Current reading: {stage_info['total_power']:.3f} mW/cm²")
    else:
        print(f"   🔧 To display mW/cm² total irradiance on Stage {stage_info['stage']}:")
        print(f"   📝 Set Total Units Index to 3")
        if 'unit_matrix' in enhanced_calculations:
            expected_value = enhanced_calculations['unit_matrix']['total_irradiance_mw_cm2']['value']
            print(f"   📏 Expected reading: {expected_value:.3f} mW/cm²")
    
    if stage_info['per_units_index'] == 5:
        print(f"   ✅ Stage {stage_info['stage']} per-well units show total mW/cm²!")
        print(f"   📏 Current reading: {stage_info['per_power']:.3f} mW/cm²")
    elif stage_info['per_units_index'] == 4:
        print(f"   ✅ Stage {stage_info['stage']} is configured for mW/cm² per well!")
        print(f"   📏 Current reading: {stage_info['per_power']:.3f} mW/cm² per well")
    else:
        print(f"   🔧 To display mW/cm² irradiance in per-well units:")
        print(f"   📝 Set Per Units Index to 4 (per-well) or 5 (total)")      # === DATA QUALITY ASSESSMENT ===
    if 'data_quality' in enhanced_calculations:
        quality = enhanced_calculations['data_quality']
        print(f"\n🎯 DATA CONFIDENCE:")
        if quality['has_direct_power_reading']:
            print(f"   🟢 HIGH - Direct power measurements available")
        elif quality['has_current_reading']:
            print(f"   🟡 MEDIUM - Estimated from current readings")
        else:
            print(f"   🔴 LOW - Limited data available")
    
    print("🔬"*40)

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

def displaySmartSummary(all_stages_info, plate_geometry):
    """Display overall summary and recommendations"""
    print("\n" + "🎯"*40)
    print("              SMART DEVICE SUMMARY & RECOMMENDATIONS")
    print("🎯"*40)
    
    # Analyze all stages
    irradiance_stages = []
    power_stages = []
    current_only_stages = []
    no_data_stages = []
    
    total_irradiance_available = 0
    
    for stage_info in all_stages_info:
        stage = stage_info['stage']
        
        # Check what type of data is available
        has_power = stage_info['total_power'] > 0 or stage_info['per_power'] > 0
        has_current = stage_info['fire_current_ma'] > 0
        has_irradiance = stage_info['total_units_index'] == 3 or stage_info['per_units_index'] in [4, 5]
        
        if has_irradiance:
            irradiance_stages.append(stage)
            if stage_info['total_units_index'] == 3:
                total_irradiance_available += stage_info['total_power']
        elif has_power:
            power_stages.append(stage)
        elif has_current:
            current_only_stages.append(stage)
        else:
            no_data_stages.append(stage)
    
    # Display status
    print(f"\n📊 DEVICE CONFIGURATION STATUS:")
    print(f"   🟢 Stages with mW/cm² irradiance: {irradiance_stages if irradiance_stages else 'None'}")
    print(f"   🟡 Stages with power readings: {power_stages if power_stages else 'None'}")
    print(f"   🟠 Stages with current only: {current_only_stages if current_only_stages else 'None'}")
    print(f"   🔴 Stages with no usable data: {no_data_stages if no_data_stages else 'None'}")
      # Calculate device potential
    total_calculated_irradiance = 0
    for stage_info in all_stages_info:
        if stage_info['total_power'] > 0 or stage_info['per_power'] > 0 or stage_info['fire_current_ma'] > 0:
            stage_calculations = calculate_all_possible_units(stage_info, plate_geometry)
            if 'unit_matrix' in stage_calculations and 'total_irradiance_mw_cm2' in stage_calculations['unit_matrix']:
                total_calculated_irradiance += stage_calculations['unit_matrix']['total_irradiance_mw_cm2']['value']
    
    print(f"\n💡 IRRADIANCE POTENTIAL:")
    if irradiance_stages:
        print(f"   📏 Current total irradiance available: {total_irradiance_available:.3f} mW/cm²")
    print(f"   🧮 Calculated total device irradiance: {total_calculated_irradiance:.3f} mW/cm²")
    print(f"   📐 Based on plate area: {plate_geometry['total_area_cm2']:.2f} cm² ({plate_geometry['well_count']} wells)")
    
    # Provide recommendations
    print(f"\n🔧 CONFIGURATION RECOMMENDATIONS:")
    
    if not irradiance_stages:
        print(f"   ⚠️  No stages currently configured for mW/cm² irradiance")
        print(f"   💡 To enable mW/cm² readings:")
        for stage_info in all_stages_info[:3]:  # Show recommendations for first 3 stages
            if stage_info['total_power'] > 0 or stage_info['fire_current_ma'] > 0:
                stage_calculations = calculate_all_possible_units(stage_info, plate_geometry)
                if 'unit_matrix' in stage_calculations and 'total_irradiance_mw_cm2' in stage_calculations['unit_matrix']:
                    irradiance_value = stage_calculations['unit_matrix']['total_irradiance_mw_cm2']['value']
                    print(f"      📝 Stage {stage_info['stage']}: Set Total Units Index = 3 → {irradiance_value:.3f} mW/cm²")
    else:
        print(f"   ✅ {len(irradiance_stages)} stage(s) already configured for irradiance")
        print(f"   💡 Consider configuring additional stages for complete coverage")
    
    # Show plate geometry optimization
    print(f"\n📐 PLATE GEOMETRY (From Schematic):")
    print(f"   📏 Dimensions: {plate_geometry['plate_length_cm']:.2f} × {plate_geometry['plate_width_cm']:.2f} cm")
    print(f"   🔍 Well diameter: {plate_geometry['well_diameter_mm']:.1f} mm (schematic verified)")
    print(f"   📊 Well area: {plate_geometry['well_area_cm2']:.3f} cm² each")
    print(f"   ⚖️  Total vs. well area ratio: {plate_geometry['total_area_cm2'] / (plate_geometry['well_area_cm2'] * plate_geometry['well_count']):.2f}")
      # Show confidence levels
    print(f"\n🎯 DATA CONFIDENCE LEVELS:")
    for stage_info in all_stages_info:
        if stage_info['total_power'] > 0 or stage_info['fire_current_ma'] > 0:
            stage_calculations = calculate_all_possible_units(stage_info, plate_geometry)
            if 'data_quality' in stage_calculations:
                quality = stage_calculations['data_quality']
                if quality['has_direct_power_reading']:
                    confidence = "🟢 HIGH"
                elif quality['has_current_reading']:
                    confidence = "🟡 MEDIUM"
                else:
                    confidence = "🔴 LOW"
                print(f"   Stage {stage_info['stage']}: {confidence}")
    
    print(f"\n🌟 SUMMARY:")
    active_stages = len([s for s in all_stages_info if s['total_power'] > 0 or s['fire_current_ma'] > 0])
    print(f"   📈 {active_stages}/5 stages have usable data")
    print(f"   🎯 {len(irradiance_stages)}/5 stages configured for irradiance")
    if total_calculated_irradiance > 0:
        print(f"   ⚡ Total device irradiance potential: {total_calculated_irradiance:.3f} mW/cm²")
        intensity_level = "High" if total_calculated_irradiance > 50 else "Medium" if total_calculated_irradiance > 10 else "Low"
        print(f"   🔆 Intensity level: {intensity_level}")
    
    print("🎯"*40)

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
        
        # Detect LED type for better efficiency estimation
        detected_led_type, led_analysis = displayLedTypeAnalysis(all_stages_info, plate_geometry)
        
        # Display available unit types first
        displayAvailableUnitTypes()
        
        # Then display current configurations
        displayAllStagesPowerInfo(all_stages_info)# Calculate and display derived units for each stage
        print("\n" + "🧮"*40)
        print("           SMART UNIT CALCULATIONS & ANALYSIS")
        print("🧮"*40)
        
        # Enhanced analysis for each stage
        enhanced_analyses = []
        for stage_info in all_stages_info:
            if stage_info['total_power'] > 0 or stage_info['per_power'] > 0 or stage_info['fire_current_ma'] > 0:
                # Show enhanced comprehensive analysis
                print(f"\n{'='*80}")
                print(f"                      STAGE {stage_info['stage']} ANALYSIS")
                print(f"{'='*80}")
                  # Use enhanced analysis with detected LED type                displayEnhancedUnitAnalysis(stage_info, plate_geometry, detected_led_type)
                enhanced_analyses.append(stage_info)
                
                # Show comprehensive unit analysis
                analyzeAllPossibleUnits(stage_info, plate_geometry)
            else:
                print(f"\nStage {stage_info['stage']}: No usable data available for calculations")
                print(f"  💡 Tip: Check device connection and ensure stage is configured")
        
        # Provide overall smart recommendations using enhanced analysis
        displaySmartRecommendations(all_stages_info, plate_geometry)
        
        # Provide overall summary and recommendations (legacy)
        displaySmartSummary(all_stages_info, plate_geometry)
        
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
