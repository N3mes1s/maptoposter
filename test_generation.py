"""Test poster generation."""

import requests
import time

BASE_URL = "http://localhost:8002"

def test_poster_generation():
    """Test complete poster generation."""
    print("Testing poster generation...")
    print("=" * 50)

    # Create poster request - use a small city with small distance for speed
    payload = {
        "city": "Venice",
        "country": "Italy",
        "theme": "blueprint",
        "distance": 3000  # Small area for faster test
    }

    print(f"Request: {payload}")
    r = requests.post(f"{BASE_URL}/api/posters", json=payload)
    print(f"Response status: {r.status_code}")

    if r.status_code != 200:
        print(f"Error: {r.text}")
        return

    data = r.json()
    job_id = data["job_id"]
    print(f"Job ID: {job_id}")
    print(f"Estimated time: {data.get('estimated_time')} seconds")
    print()

    # Poll for status
    print("Polling for status...")
    max_wait = 180  # 3 minutes max
    start_time = time.time()
    last_progress = -1

    while time.time() - start_time < max_wait:
        status_r = requests.get(f"{BASE_URL}/api/posters/{job_id}")
        if status_r.status_code == 200:
            status = status_r.json()
            progress = int(status.get("progress", 0) * 100)
            current_step = status.get("current_step", "unknown")
            message = status.get("message", "")

            if progress != last_progress:
                elapsed = int(time.time() - start_time)
                print(f"[{elapsed:3d}s] {progress:3d}% - {current_step}: {message}")
                last_progress = progress

            if status.get("status") == "completed":
                print()
                print("SUCCESS!")
                print(f"Download URL: {status.get('download_url')}")

                # Test download
                download_url = f"{BASE_URL}{status.get('download_url')}"
                dl = requests.get(download_url)
                if dl.status_code == 200:
                    print(f"Download size: {len(dl.content)} bytes")
                    # Save the file
                    with open("/home/user/maptoposter/test_poster.png", "wb") as f:
                        f.write(dl.content)
                    print("Saved to test_poster.png")
                return True

            elif status.get("status") == "failed":
                print()
                print(f"FAILED: {status.get('error')}")
                return False

        time.sleep(2)

    print()
    print("TIMEOUT: Job did not complete in time")
    return False


if __name__ == "__main__":
    test_poster_generation()
