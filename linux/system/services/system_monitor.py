import os
import time
import psutil
import platform
import threading
from typing import Dict, List, Optional, Tuple, Any
from dataclasses import dataclass, field
from datetime import datetime
import json
from pathlib import Path

@dataclass
class SystemMetrics:
    timestamp: float
    cpu_percent: float
    memory_percent: float
    disk_usage: Dict[str, float]
    network_io: Dict[str, Dict[str, float]]
    processes: int
    load_avg: Tuple[float, float, float]
    uptime: float
    cpu_times: Dict[str, float]
    memory_info: Dict[str, float]
    swap_memory: Dict[str, float]
    disk_io: Dict[str, float]
    temperatures: Dict[str, float]
    fans: Dict[str, float]
    battery: Optional[Dict[str, float]]

class SystemMonitor:
    def __init__(self, update_interval: float = 5.0):
        self.update_interval = update_interval
        self._running = False
        self._thread: Optional[threading.Thread] = None
        self._lock = threading.RLock()
        self._metrics: Optional[SystemMetrics] = None
        self._callbacks = []
        
    def start(self) -> None:
        """Start the system monitoring thread."""
        if self._running:
            return
            
        self._running = True
        self._thread = threading.Thread(target=self._monitor_loop, daemon=True)
        self._thread.start()
    
    def stop(self) -> None:
        """Stop the system monitoring thread."""
        self._running = False
        if self._thread:
            self._thread.join(timeout=self.update_interval * 2)
    
    def add_callback(self, callback) -> None:
        """Add a callback function to be called with new metrics."""
        with self._lock:
            self._callbacks.append(callback)
    
    def remove_callback(self, callback) -> None:
        """Remove a callback function."""
        with self._lock:
            if callback in self._callbacks:
                self._callbacks.remove(callback)
    
    def get_metrics(self) -> Optional[SystemMetrics]:
        """Get the most recent system metrics."""
        with self._lock:
            return self._metrics
    
    def _monitor_loop(self) -> None:
        """Main monitoring loop that runs in a separate thread."""
        while self._running:
            metrics = self._collect_metrics()
            
            with self._lock:
                self._metrics = metrics
                callbacks = self._callbacks.copy()
            
            for callback in callbacks:
                try:
                    callback(metrics)
                except Exception as e:
                    print(f"Error in monitor callback: {e}")
            
            time.sleep(self.update_interval)
    
    def _collect_metrics(self) -> SystemMetrics:
        """Collect all system metrics."""
        # CPU metrics
        cpu_percent = psutil.cpu_percent(interval=None)
        cpu_times = {k: v for k, v in psutil.cpu_times_percent(interval=None)._asdict().items()}
        
        # Memory metrics
        memory = psutil.virtual_memory()
        swap = psutil.swap_memory()
        
        # Disk metrics
        disk_usage = {}
        for part in psutil.disk_partitions(all=False):
            try:
                usage = psutil.disk_usage(part.mountpoint)
                disk_usage[part.mountpoint] = usage.percent
            except Exception:
                continue
        
        # Network metrics
        net_io = {}
        for name, stats in psutil.net_io_counters(pernic=True).items():
            net_io[name] = {
                'bytes_sent': stats.bytes_sent,
                'bytes_recv': stats.bytes_recv,
                'packets_sent': stats.packets_sent,
                'packets_recv': stats.packets_recv,
                'err_in': stats.errin,
                'err_out': stats.errout,
                'drop_in': stats.dropin,
                'drop_out': stats.dropout,
            }
        
        # Disk I/O
        disk_io = {}
        try:
            disk_io_counters = psutil.disk_io_counters()
            if disk_io_counters:
                disk_io = {
                    'read_count': disk_io_counters.read_count,
                    'write_count': disk_io_counters.write_count,
                    'read_bytes': disk_io_counters.read_bytes,
                    'write_bytes': disk_io_counters.write_bytes,
                    'read_time': disk_io_counters.read_time,
                    'write_time': disk_io_counters.write_time,
                }
        except Exception:
            pass
        
        # Sensors
        temperatures = {}
        fans = {}
        try:
            for name, entries in psutil.sensors_temperatures().items():
                for i, entry in enumerate(entries):
                    key = f"{name}_{i}" if len(entries) > 1 else name
                    temperatures[key] = entry.current
            
            for name, entries in psutil.sensors_fans().items():
                for i, entry in enumerate(entries):
                    key = f"{name}_{i}" if len(entries) > 1 else name
                    fans[key] = entry.current
        except Exception:
            pass
        
        # Battery
        battery = None
        try:
            if hasattr(psutil, 'sensors_battery'):
                batt = psutil.sensors_battery()
                if batt:
                    battery = {
                        'percent': batt.percent,
                        'secsleft': batt.secsleft,
                        'power_plugged': batt.power_plugged,
                    }
        except Exception:
            pass
        
        # System info
        load_avg = psutil.getloadavg() if hasattr(psutil, 'getloadavg') else (0.0, 0.0, 0.0)
        uptime = time.time() - psutil.boot_time()
        
        return SystemMetrics(
            timestamp=time.time(),
            cpu_percent=cpu_percent,
            memory_percent=memory.percent,
            disk_usage=disk_usage,
            network_io=net_io,
            processes=len(psutil.pids()),
            load_avg=load_avg,
            uptime=uptime,
            cpu_times=cpu_times,
            memory_info={
                'total': memory.total,
                'available': memory.available,
                'used': memory.used,
                'free': memory.free,
                'active': getattr(memory, 'active', 0),
                'inactive': getattr(memory, 'inactive', 0),
                'buffers': getattr(memory, 'buffers', 0),
                'cached': getattr(memory, 'cached', 0),
                'shared': getattr(memory, 'shared', 0),
                'slab': getattr(memory, 'slab', 0),
            },
            swap_memory={
                'total': swap.total,
                'used': swap.used,
                'free': swap.free,
                'percent': swap.percent,
                'sin': getattr(swap, 'sin', 0),
                'sout': getattr(swap, 'sout', 0),
            },
            disk_io=disk_io,
            temperatures=temperatures,
            fans=fans,
            battery=battery,
        )
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert the current metrics to a dictionary."""
        metrics = self.get_metrics()
        if not metrics:
            return {}
            
        return self._metrics_to_dict(metrics)
    
    def _metrics_to_dict(self, metrics: SystemMetrics) -> Dict[str, Any]:
        """Convert metrics to a serializable dictionary."""
        return {
            'timestamp': metrics.timestamp,
            'cpu': {
                'percent': metrics.cpu_percent,
                'times': metrics.cpu_times,
                'load_avg': {
                    '1min': metrics.load_avg[0],
                    '5min': metrics.load_avg[1],
                    '15min': metrics.load_avg[2],
                },
            },
            'memory': {
                'percent': metrics.memory_percent,
                'info': metrics.memory_info,
            },
            'swap': metrics.swap_memory,
            'disk': {
                'usage': metrics.disk_usage,
                'io': metrics.disk_io,
            },
            'network': metrics.network_io,
            'system': {
                'processes': metrics.processes,
                'uptime': metrics.uptime,
                'hostname': platform.node(),
                'os': platform.system(),
                'os_version': platform.version(),
                'platform': platform.platform(),
                'processor': platform.processor(),
                'architecture': platform.architecture(),
            },
            'sensors': {
                'temperatures': metrics.temperatures,
                'fans': metrics.fans,
            },
            'battery': metrics.battery,
        }
    
    def save_to_file(self, path: str) -> None:
        """Save the current metrics to a JSON file."""
        metrics = self.get_metrics()
        if not metrics:
            return
            
        data = self._metrics_to_dict(metrics)
        
        path_obj = Path(path)
        path_obj.parent.mkdir(parents=True, exist_ok=True)
        
        with open(path, 'w') as f:
            json.dump(data, f, indent=2)
