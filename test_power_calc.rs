use lumidox_ii_controller::core::calculations::irradiance::IrradianceCalculator;

fn main() {
    println!("=== Testing Power Calculation Logic ===");
    
    // Test 1: Direct calculation without device data (fallback)
    println!("\n1. Testing fallback calibration for 405mA:");
    let power_info = IrradianceCalculator::estimate_power_from_current(405, None);
    println!("  Total power: {} {}", power_info.total_power, power_info.total_units);
    println!("  Per power: {} {}", power_info.per_power, power_info.per_units);
    
    // Test 2: Test with empty device data (should also use fallback)
    println!("\n2. Testing with empty stage info (should fallback):");
    let empty_stage_info = std::collections::HashMap::new();
    let power_info2 = IrradianceCalculator::estimate_power_with_device_data(405, Some(&empty_stage_info));
    println!("  Total power: {} {}", power_info2.total_power, power_info2.total_units);
    println!("  Per power: {} {}", power_info2.per_power, power_info2.per_units);
    
    // Test 3: Irradiance calculation
    println!("\n3. Testing irradiance calculation for 405mA:");
    match IrradianceCalculator::calculate_irradiance_from_current(405) {
        Ok(irradiance_data) => {
            println!("  Surface irradiance: {:.1} mW/cm²", irradiance_data.surface_irradiance_mw_cm2);
            println!("  Well-bottom irradiance: {:.1} mW/cm²", irradiance_data.well_bottom_irradiance_mw_cm2);
            println!("  Total power used: {:.1} mW", irradiance_data.total_power_mw);
        }
        Err(e) => println!("  Error: {}", e),
    }
    
    // Test 4: Validate against user measurement
    println!("\n4. Model validation:");
    match IrradianceCalculator::validate_against_measurement() {
        Ok(report) => println!("{}", report),
        Err(e) => println!("  Error: {}", e),
    }
}
