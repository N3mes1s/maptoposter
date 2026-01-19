"""API tests using requests."""

import requests
import time
import json

BASE_URL = "http://localhost:8002"


def test_health():
    """Test health endpoint."""
    print("1. Testing health endpoint...")
    r = requests.get(f"{BASE_URL}/health")
    assert r.status_code == 200
    data = r.json()
    assert data["status"] == "healthy"
    print(f"   Health: {data}")
    return True


def test_themes():
    """Test themes endpoint."""
    print("2. Testing themes endpoint...")
    r = requests.get(f"{BASE_URL}/api/themes")
    assert r.status_code == 200
    data = r.json()
    assert "themes" in data
    assert len(data["themes"]) > 0
    print(f"   Themes count: {len(data['themes'])}")
    print(f"   First theme: {data['themes'][0]['id']}")
    return True


def test_frontend():
    """Test frontend is served."""
    print("3. Testing frontend...")
    r = requests.get(f"{BASE_URL}/")
    assert r.status_code == 200
    assert "MapToPoster" in r.text
    assert "city" in r.text
    assert "country" in r.text
    assert "theme-selector" in r.text
    print("   Frontend HTML: OK")

    # Test CSS
    r = requests.get(f"{BASE_URL}/css/styles.css")
    assert r.status_code == 200
    assert "theme-card" in r.text
    print("   CSS: OK")

    # Test JS
    r = requests.get(f"{BASE_URL}/js/app.js")
    assert r.status_code == 200
    assert "fetchThemes" in r.text
    print("   JavaScript: OK")

    r = requests.get(f"{BASE_URL}/js/api.js")
    assert r.status_code == 200
    assert "createPoster" in r.text
    print("   API client: OK")

    return True


def test_api_docs():
    """Test API documentation."""
    print("4. Testing API documentation...")
    r = requests.get(f"{BASE_URL}/api/docs")
    assert r.status_code == 200
    assert "swagger" in r.text.lower() or "openapi" in r.text.lower()
    print("   OpenAPI docs: OK")
    return True


def test_poster_generation():
    """Test poster generation endpoint."""
    print("5. Testing poster generation...")

    # Create poster request
    payload = {
        "city": "Venice",
        "country": "Italy",
        "theme": "blueprint",
        "distance": 4000
    }

    print(f"   Requesting poster for {payload['city']}, {payload['country']}...")
    r = requests.post(f"{BASE_URL}/api/posters", json=payload)

    if r.status_code == 200:
        data = r.json()
        print(f"   Job created: {data.get('job_id', 'N/A')}")

        job_id = data.get("job_id")
        if job_id:
            # Poll for status
            for i in range(60):  # 60 * 2 = 120 seconds max
                time.sleep(2)
                status_r = requests.get(f"{BASE_URL}/api/posters/{job_id}")
                if status_r.status_code == 200:
                    status = status_r.json()
                    progress = status.get("progress", 0)
                    current_step = status.get("current_step", "unknown")
                    print(f"   Progress: {progress}% - {current_step}")

                    if status.get("status") == "completed":
                        print(f"   Download URL: {status.get('download_url')}")
                        return True
                    elif status.get("status") == "failed":
                        print(f"   Failed: {status.get('error')}")
                        return False

            print("   Timeout waiting for completion")
            return False
    else:
        print(f"   Error: {r.status_code} - {r.text}")
        return False


def main():
    """Run all tests."""
    print("=" * 50)
    print("MapToPoster API Tests")
    print("=" * 50)
    print()

    tests = [
        ("Health", test_health),
        ("Themes", test_themes),
        ("Frontend", test_frontend),
        ("API Docs", test_api_docs),
        ("Poster Generation", test_poster_generation),
    ]

    results = []
    for name, test_fn in tests:
        try:
            result = test_fn()
            results.append((name, result))
            print(f"   Result: {'PASS' if result else 'FAIL'}")
        except Exception as e:
            print(f"   Exception: {e}")
            results.append((name, False))
        print()

    print("=" * 50)
    print("Summary:")
    for name, result in results:
        status = "PASS" if result else "FAIL"
        print(f"  {name}: {status}")
    print("=" * 50)

    passed = sum(1 for _, r in results if r)
    total = len(results)
    print(f"\n{passed}/{total} tests passed")


if __name__ == "__main__":
    main()
