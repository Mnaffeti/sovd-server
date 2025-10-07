#!/usr/bin/env python3
"""
Test script for SOVD2UDS Adapter API
Tests various endpoints and operations
"""

import requests
import json
import sys

BASE_URL = "http://localhost:8081/api/v1"

def test_health():
    """Test health check endpoint"""
    print("Testing health check...")
    response = requests.get("http://localhost:8081/health")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}\n")
    return response.status_code == 200

def test_get_components():
    """Test getting all components"""
    print("Testing get components...")
    response = requests.get(f"{BASE_URL}/components")
    print(f"Status: {response.status_code}")
    data = response.json()
    print(f"Components: {json.dumps(data, indent=2)}\n")
    return response.status_code == 200

def test_get_component_data(component_id="engine"):
    """Test getting component data items"""
    print(f"Testing get data items for {component_id}...")
    response = requests.get(f"{BASE_URL}/components/{component_id}/data")
    print(f"Status: {response.status_code}")
    data = response.json()
    print(f"Data Items: {json.dumps(data, indent=2)}\n")
    return response.status_code == 200

def test_get_data_item_value(component_id="engine", data_id="vin"):
    """Test reading a specific data item"""
    print(f"Testing read {data_id} from {component_id}...")
    response = requests.get(f"{BASE_URL}/components/{component_id}/data/{data_id}")
    print(f"Status: {response.status_code}")
    
    if response.status_code == 200:
        data = response.json()
        print(f"Data: {json.dumps(data, indent=2)}\n")
    else:
        print(f"Error: {response.text}\n")
    
    return response.status_code == 200

def test_read_dtcs(component_id="engine"):
    """Test reading DTCs"""
    print(f"Testing read DTCs from {component_id}...")
    payload = {"action": "read"}
    response = requests.post(
        f"{BASE_URL}/components/{component_id}/dtcs",
        json=payload
    )
    print(f"Status: {response.status_code}")
    
    if response.status_code == 200:
        data = response.json()
        print(f"DTCs: {json.dumps(data, indent=2)}\n")
    else:
        print(f"Error: {response.text}\n")
    
    return response.status_code == 200

def test_clear_dtcs(component_id="engine"):
    """Test clearing DTCs"""
    print(f"Testing clear DTCs from {component_id}...")
    payload = {"action": "clear"}
    response = requests.post(
        f"{BASE_URL}/components/{component_id}/dtcs",
        json=payload
    )
    print(f"Status: {response.status_code}")
    
    if response.status_code == 200:
        data = response.json()
        print(f"Result: {json.dumps(data, indent=2)}\n")
    else:
        print(f"Error: {response.text}\n")
    
    return response.status_code == 200

def test_control_actuator(component_id="engine"):
    """Test actuator control"""
    print(f"Testing actuator control on {component_id}...")
    payload = {
        "actuator_id": "fuel_pump",
        "action": "start",
        "duration": 30
    }
    response = requests.post(
        f"{BASE_URL}/components/{component_id}/actuators/control",
        json=payload
    )
    print(f"Status: {response.status_code}")
    
    if response.status_code == 200:
        data = response.json()
        print(f"Result: {json.dumps(data, indent=2)}\n")
    else:
        print(f"Error: {response.text}\n")
    
    return response.status_code == 200

def test_session_control(component_id="engine"):
    """Test diagnostic session control"""
    print(f"Testing session control on {component_id}...")
    payload = {
        "service_type": "session_control",
        "parameters": {
            "session_type": 3  # Extended diagnostic session
        }
    }
    response = requests.post(
        f"{BASE_URL}/components/{component_id}/services",
        json=payload
    )
    print(f"Status: {response.status_code}")
    
    if response.status_code == 200:
        data = response.json()
        print(f"Result: {json.dumps(data, indent=2)}\n")
    else:
        print(f"Error: {response.text}\n")
    
    return response.status_code == 200

def run_all_tests():
    """Run all tests"""
    print("=" * 60)
    print("SOVD2UDS Adapter API Test Suite")
    print("=" * 60 + "\n")
    
    tests = [
        ("Health Check", test_health),
        ("Get Components", test_get_components),
        ("Get Component Data Items", test_get_component_data),
        ("Read Data Item (VIN)", test_get_data_item_value),
        ("Read Data Item (Software Version)", 
         lambda: test_get_data_item_value("engine", "ecu_software_version")),
        ("Read DTCs", test_read_dtcs),
        ("Session Control", test_session_control),
        ("Control Actuator", test_control_actuator),
        # ("Clear DTCs", test_clear_dtcs),  # Commented out to avoid clearing actual DTCs
    ]
    
    results = []
    for name, test_func in tests:
        try:
            success = test_func()
            results.append((name, success))
        except Exception as e:
            print(f"Test '{name}' failed with exception: {e}\n")
            results.append((name, False))
    
    # Print summary
    print("=" * 60)
    print("Test Summary")
    print("=" * 60)
    for name, success in results:
        status = "✓ PASS" if success else "✗ FAIL"
        print(f"{status}: {name}")
    
    passed = sum(1 for _, success in results if success)
    total = len(results)
    print(f"\nPassed: {passed}/{total}")
    
    return passed == total

if __name__ == "__main__":
    try:
        success = run_all_tests()
        sys.exit(0 if success else 1)
    except KeyboardInterrupt:
        print("\nTests interrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\nFatal error: {e}")
        sys.exit(1)
