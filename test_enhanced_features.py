#!/usr/bin/env python3
"""
Test script to demonstrate all enhanced features of display_stage1_units.py
"""

import display_stage1_units

def test_comprehensive_functionality():
    """Test all enhanced functions"""
    print("🔬 COMPREHENSIVE ENHANCED UNIT ANALYSIS DEMO")
    print("="*80)
    
    # Get plate geometry
    plate_geometry = display_stage1_units.get_plate_geometry()
    print(f"✅ Plate geometry loaded: {plate_geometry['total_area_cm2']:.2f} cm²")
      # Create mock multi-stage data
    mock_stages = [
        {
            'stage': 1,
            'total_power': 15.5,
            'per_power': 2.3,
            'fire_current_ma': 45.0,
            'total_units_index': 2,  # mW
            'per_units_index': 2,    # mW
            'total_units': 'mW',
            'per_units': 'mW'
        },
        {
            'stage': 2,
            'total_power': 125.8,
            'per_power': 0.0,
            'fire_current_ma': 0.0,
            'total_units_index': 3,  # mW/cm²
            'per_units_index': 0,
            'total_units': 'mW/cm²',
            'per_units': ''
        },
        {
            'stage': 3,
            'total_power': 0.0,
            'per_power': 1.85,
            'fire_current_ma': 32.5,
            'total_units_index': 0,
            'per_units_index': 5,    # mW/cm²
            'total_units': '',
            'per_units': 'mW/cm²'
        }
    ]
    
    print("\n📊 TESTING ENHANCED UNIT ANALYSIS:")
    for stage_info in mock_stages:
        print(f"\n--- Testing Stage {stage_info['stage']} ---")
        
        # Test unit detection
        detected_units = display_stage1_units.detect_all_unit_types(stage_info)
        print(f"✅ Unit detection: {len(detected_units)} types detected")
        
        # Test comprehensive calculations
        calculations = display_stage1_units.calculate_all_possible_units(stage_info, plate_geometry)
        print(f"✅ Calculations: {len(calculations)} values computed")
        
        # Test LED type detection for single stage
        led_type, efficiency, analysis = display_stage1_units.detect_led_type([stage_info], plate_geometry)
        print(f"✅ LED analysis: {led_type} type, {efficiency:.1f}% efficiency")
    
    print("\n🎯 TESTING DISPLAY FUNCTIONS:")
    
    # Test enhanced unit analysis display
    print("\n--- Enhanced Unit Analysis ---")
    display_stage1_units.displayEnhancedUnitAnalysis(mock_stages[0], plate_geometry)
    
    # Test smart recommendations
    print("\n--- Smart Recommendations ---")
    display_stage1_units.displaySmartRecommendations(mock_stages, plate_geometry)
    
    # Test LED type analysis
    print("\n--- LED Type Analysis ---")
    display_stage1_units.displayLedTypeAnalysis(mock_stages, plate_geometry)
    
    # Test smart summary
    print("\n--- Smart Summary ---")
    display_stage1_units.displaySmartSummary(mock_stages, plate_geometry)
    
    print("\n✅ ALL ENHANCED FEATURES TESTED SUCCESSFULLY!")
    print("="*80)

if __name__ == "__main__":
    test_comprehensive_functionality()
