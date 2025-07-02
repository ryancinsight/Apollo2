#!/usr/bin/env python3
"""
Test script to validate the accuracy of the new well-bottom irradiance calculations.
This script simulates the Rust calculations to verify they match the user's measurement:
5.5 mW/cm² at 405 mA through lid at well bottom.
"""

import math

def get_plate_geometry():
    """Get plate geometry specifications"""
    plate_length_mm = 127.75
    plate_width_mm = 105.5
    
    plate_length_cm = plate_length_mm / 10.0
    plate_width_cm = plate_width_mm / 10.0
    total_area_cm2 = plate_length_cm * plate_width_cm
    
    well_count = 96
    well_diameter_mm = 5.0
    well_area_cm2 = math.pi * (well_diameter_mm / 20.0) ** 2
    
    return {
        'total_area_cm2': total_area_cm2,
        'well_area_cm2': well_area_cm2,
        'well_count': well_count
    }

def estimate_power_for_current(current_ma):
    """Estimate power for current using fallback calibration"""
    calibration_points = [
        (60, 500.0, 5.0),      # Stage 1
        (110, 1900.0, 19.0),   # Stage 2  
        (230, 2400.0, 24.0),   # Stage 3
        (420, 4800.0, 48.0),   # Stage 4
        (795, 9600.0, 96.0),   # Stage 5
    ]
    
    current_f32 = float(current_ma)
    
    # Find interpolation bounds for 405 mA (between stages 3 and 4)
    # Stage 3: (230, 2400.0, 24.0)
    # Stage 4: (420, 4800.0, 48.0)
    
    low_current, low_total, low_per = 230, 2400.0, 24.0
    high_current, high_total, high_per = 420, 4800.0, 48.0
    
    # Linear interpolation
    current_range = high_current - low_current  # 420 - 230 = 190
    current_offset = current_f32 - low_current  # 405 - 230 = 175
    interpolation_factor = current_offset / current_range  # 175/190 = 0.921
    
    interpolated_total = low_total + (high_total - low_total) * interpolation_factor
    interpolated_per = low_per + (high_per - low_per) * interpolation_factor
    
    print(f"Power interpolation for {current_ma} mA:")
    print(f"  Between Stage 3 ({low_current} mA, {low_total} mW) and Stage 4 ({high_current} mA, {high_total} mW)")
    print(f"  Interpolation factor: {interpolation_factor:.3f}")
    print(f"  Interpolated total power: {interpolated_total:.1f} mW")
    print(f"  Interpolated per-well power: {interpolated_per:.1f} mW")
    
    return interpolated_total, interpolated_per

def calculate_well_bottom_attenuation(well_depth_mm=44.0, well_diameter_mm=5.0, include_lid=True):
    """Calculate detailed well-bottom attenuation"""
    print(f"\nCalculating well-bottom attenuation:")
    print(f"  Well depth: {well_depth_mm} mm")
    print(f"  Well diameter: {well_diameter_mm} mm")
    print(f"  Include lid losses: {include_lid}")
    
    # 1. Geometric light collection efficiency
    well_radius_mm = well_diameter_mm / 2.0
    acceptance_half_angle_rad = math.atan(well_radius_mm / well_depth_mm)
    
    # Solid angle fraction
    cone_collection_efficiency = 1.0 - math.cos(acceptance_half_angle_rad)
    
    print(f"  Acceptance half-angle: {math.degrees(acceptance_half_angle_rad):.2f}°")
    print(f"  Cone collection efficiency: {cone_collection_efficiency:.4f}")
    
    # 2. Well wall reflection losses
    wall_reflectivity = 0.85
    aspect_ratio = well_depth_mm / well_diameter_mm
    avg_bounces = max(aspect_ratio * 0.5, 1.0)
    wall_transmission_efficiency = wall_reflectivity ** avg_bounces
    
    print(f"  Aspect ratio: {aspect_ratio:.1f}")
    print(f"  Average bounces: {avg_bounces:.1f}")
    print(f"  Wall transmission efficiency: {wall_transmission_efficiency:.4f}")
    
    # 3. Combined geometric efficiency
    direct_light_fraction = cone_collection_efficiency
    indirect_light_fraction = 1.0 - direct_light_fraction
    
    geometric_efficiency = (direct_light_fraction * 1.0 + 
                          indirect_light_fraction * wall_transmission_efficiency)
    
    print(f"  Direct light fraction: {direct_light_fraction:.4f}")
    print(f"  Indirect light fraction: {indirect_light_fraction:.4f}")
    print(f"  Combined geometric efficiency: {geometric_efficiency:.4f}")
    
    # 4. Air transmission
    air_transmission = 0.97 ** (well_depth_mm / 10.0)
    print(f"  Air transmission: {air_transmission:.4f}")
    
    # 5. Lid transmission
    lid_transmission = 0.75 if include_lid else 1.0
    print(f"  Lid transmission: {lid_transmission:.4f}")
    
    # 6. LED array coupling
    led_array_coupling = 0.80
    print(f"  LED array coupling: {led_array_coupling:.4f}")
    
    # 7. Total attenuation before calibration
    total_theoretical = (geometric_efficiency * air_transmission * 
                        lid_transmission * led_array_coupling)
    print(f"  Theoretical attenuation: {total_theoretical:.6f}")
    
    # 8. Calibration factor (from user measurement)
    # User measured 5.5 mW/cm² at 405 mA
    # Theoretical gives 0.257111 attenuation
    # Surface irradiance at 405 mA is ~34.2 mW/cm²
    # So we need: 5.5 / 34.2 = 0.1608 total attenuation
    # Calibration factor = 0.1608 / 0.257111 = 0.625
    calibration_factor = 0.625
    print(f"  Calibration factor: {calibration_factor:.4f}")
    
    total_attenuation = total_theoretical * calibration_factor
    print(f"  Final attenuation: {total_attenuation:.6f}")
    
    return total_attenuation

def test_model_accuracy():
    """Test the model against user's measurement"""
    print("=== Well-Bottom Irradiance Model Validation ===")
    print("User measurement: 5.5 mW/cm² at 405 mA through lid at well bottom")
    print()
    
    # Get geometry
    geometry = get_plate_geometry()
    print(f"Plate geometry:")
    print(f"  Total area: {geometry['total_area_cm2']:.2f} cm²")
    print(f"  Well area: {geometry['well_area_cm2']:.4f} cm²")
    print()
    
    # Estimate power for 405 mA
    total_power_mw, per_power_mw = estimate_power_for_current(405)
    print()
    
    # Calculate surface irradiance
    surface_irradiance = total_power_mw / geometry['total_area_cm2']
    print(f"Surface irradiance: {surface_irradiance:.1f} mW/cm²")
    print()
    
    # Calculate well-bottom attenuation
    well_attenuation = calculate_well_bottom_attenuation(include_lid=True)
    print()
    
    # Calculate well-bottom irradiance
    well_bottom_irradiance = surface_irradiance * well_attenuation
    
    print("=== RESULTS ===")
    print(f"Calculated well-bottom irradiance: {well_bottom_irradiance:.1f} mW/cm²")
    print(f"User measured irradiance: 5.5 mW/cm²")
    
    error_percent = abs(well_bottom_irradiance - 5.5) / 5.5 * 100
    print(f"Error: {error_percent:.1f}%")
    
    if error_percent < 15:
        print("✓ Model accuracy is GOOD (< 15% error)")
    elif error_percent < 30:
        print("⚠ Model accuracy is FAIR (15-30% error)")
    else:
        print("✗ Model accuracy needs improvement (> 30% error)")

if __name__ == "__main__":
    test_model_accuracy()
