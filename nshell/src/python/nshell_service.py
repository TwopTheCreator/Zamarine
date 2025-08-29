"""
nshell_service.py - Python backend service for nshell
Provides enhanced functionality and system integration
"""

import os
import sys
import json
import subprocess
import platform
import socket
import psutil
import datetime
from pathlib import Path

class NShellService:
    def __init__(self):
        self.version = "0.1"
        self.platform = platform.system()
        self.hostname = socket.gethostname()
        self.user = os.getlogin()
        
    def get_system_info(self):
        """Get detailed system information"""
        return {
            "platform": self.platform,
            "hostname": self.hostname,
            "user": self.user,
            "python_version": platform.python_version(),
            "os_version": platform.version(),
            "machine": platform.machine(),
            "processor": platform.processor(),
            "boot_time": datetime.datetime.fromtimestamp(psutil.boot_time()).strftime("%Y-%m-%d %H:%M:%S"),
        }
    
    def execute_command(self, command):
        """Execute a shell command and return the output"""
        try:
            result = subprocess.run(
                command,
                shell=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )
            return {
                "success": result.returncode == 0,
                "returncode": result.returncode,
                "stdout": result.stdout,
                "stderr": result.stderr
            }
        except Exception as e:
            return {
                "success": False,
                "error": str(e)
            }
    
    def get_disk_usage(self, path="."):
        """Get disk usage information for the specified path"""
        try:
            usage = psutil.disk_usage(path)
            return {
                "path": os.path.abspath(path),
                "total_gb": round(usage.total / (1024 ** 3), 2),
                "used_gb": round(usage.used / (1024 ** 3), 2),
                "free_gb": round(usage.free / (1024 ** 3), 2),
                "percent_used": usage.percent
            }
        except Exception as e:
            return {"error": str(e)}
    
    def get_process_list(self):
        """Get list of running processes"""
        processes = []
        for proc in psutil.process_iter(['pid', 'name', 'username', 'status']):
            try:
                pinfo = proc.info
                processes.append({
                    "pid": pinfo['pid'],
                    "name": pinfo['name'],
                    "user": pinfo['username'],
                    "status": pinfo['status']
                })
            except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
                pass
        return processes
    
    def get_network_info(self):
        """Get network interface information"""
        interfaces = []
        for name, addrs in psutil.net_if_addrs().items():
            if name == 'lo':
                continue
            addr_info = {"interface": name, "addresses": []}
            for addr in addrs:
                if addr.family == psutil.AF_INET:
                    addr_info["addresses"].append({
                        "type": "IPv4",
                        "address": addr.address,
                        "netmask": addr.netmask
                    })
                elif addr.family == psutil.AF_INET6:
                    addr_info["addresses"].append({
                        "type": "IPv6",
                        "address": addr.address.split('%')[0],
                        "netmask": addr.netmask
                    })
            if addr_info["addresses"]:
                interfaces.append(addr_info)
        return interfaces

    def format_output(self, data, format_type="text"):
        """Format output based on requested format"""
        if format_type == "json":
            return json.dumps(data, indent=2)
        elif format_type == "text":
            if isinstance(data, list):
                return "\n".join(str(item) for item in data)
            elif isinstance(data, dict):
                return "\n".join(f"{k}: {v}" for k, v in data.items())
            return str(data)
        return str(data)

def main():
    """Main entry point for the service"""
    service = NShellService()
    
    if len(sys.argv) > 1:
        command = sys.argv[1].lower()
        format_type = sys.argv[2] if len(sys.argv) > 2 else "text"
        
        if command == "system":
            result = service.get_system_info()
        elif command == "disk":
            path = sys.argv[3] if len(sys.argv) > 3 else "."
            result = service.get_disk_usage(path)
        elif command == "processes":
            result = service.get_process_list()
        elif command == "network":
            result = service.get_network_info()
        elif command == "exec" and len(sys.argv) > 2:
            cmd = " ".join(sys.argv[2:])
            result = service.execute_command(cmd)
        else:
            result = {"error": f"Unknown command: {command}"}
        
        print(service.format_output(result, format_type))
    else:
        print("NShell Python Service")
        print("Available commands:")
        print("  system - Show system information")
        print("  disk [path] - Show disk usage (default: current directory)")
        print("  processes - List running processes")
        print("  network - Show network interfaces")
        print("  exec <command> - Execute a shell command")

if __name__ == "__main__":
    main()
