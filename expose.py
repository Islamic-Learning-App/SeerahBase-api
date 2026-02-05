import os
import subprocess
import time
import requests
import shutil

CLOUDFLARED_URL = "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64"
BINARY_NAME = "cloudflared"

def download_cloudflared():
    if shutil.which(BINARY_NAME):
        print(f"✅ {BINARY_NAME} is already installed.")
        return shutil.which(BINARY_NAME)
    
    if os.path.exists(BINARY_NAME):
        print(f"✅ {BINARY_NAME} binary found in current directory.")
        return f"./{BINARY_NAME}"

    print(f"⬇️ Downloading {BINARY_NAME}...")
    try:
        response = requests.get(CLOUDFLARED_URL, stream=True)
        response.raise_for_status()
        with open(BINARY_NAME, "wb") as f:
            for chunk in response.iter_content(chunk_size=8192):
                f.write(chunk)
        os.chmod(BINARY_NAME, 0o755)
        print("✅ Download complete.")
        return f"./{BINARY_NAME}"
    except Exception as e:
        print(f"❌ Failed to download cloudflared: {e}")
        return None

def start_tunnel(binary_path, port):
    print(f"🚀 Starting tunnel for port {port}...")
    # --url flag automatically creates a quick tunnel
    cmd = [binary_path, "tunnel", "--url", f"http://localhost:{port}"]
    
    process = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Read stderr to find the URL (cloudflared prints URL to stderr)
    url = None
    print("⏳ Waiting for URL generation...")
    
    # Simple loop to read lines
    start_time = time.time()
    while time.time() - start_time < 30: # Wait up to 30 seconds
        line = process.stderr.readline()
        if not line:
            if process.poll() is not None:
                break
            continue
            
        if "trycloudflare.com" in line and "https://" in line:
            print(f"DEBUG: Found URL line: {line.strip()}")
            # Extract URL
            parts = line.split()
            for part in parts:
                if "https://" in part and "trycloudflare.com" in part:
                    url = part
                    break
            if url:
                break
                
    if url:
        print(f"\n🎉 Public URL: {url}")
        print("(Press Ctrl+C to stop the tunnel)\n")
    else:
        print("❌ Could not extract URL from cloudflared output.")
        process.terminate()
        return

    try:
        process.wait()
    except KeyboardInterrupt:
        print("\n🛑 Stopping tunnel...")
        process.terminate()

if __name__ == "__main__":
    binary = download_cloudflared()
    if binary:
        # We only need to expose Frontend (5173) because Vite proxies /api -> 3000
        start_tunnel(binary, 5173)
