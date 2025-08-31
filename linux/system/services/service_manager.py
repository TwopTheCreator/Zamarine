import os
import sys
import time
import signal
import logging
import subprocess
import threading
import json
from pathlib import Path
from typing import Dict, List, Optional, Callable, Any, Union
from dataclasses import dataclass, field
from enum import Enum, auto
import shlex
import atexit

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout),
        logging.FileHandler('service_manager.log')
    ]
)
logger = logging.getLogger('ServiceManager')

class ServiceState(Enum):
    STOPPED = auto()
    STARTING = auto()
    RUNNING = auto()
    STOPPING = auto()
    RESTARTING = auto()
    FAILED = auto()
    UNKNOWN = auto()

class ServiceType(Enum):
    SIMPLE = auto()
    FORKING = auto()
    ONESHOT = auto()
    DBUS = auto()
    NOTIFY = auto()
    IDLE = auto()

@dataclass
class ServiceConfig:
    name: str
    description: str = ""
    exec_start: str = ""
    exec_stop: str = ""
    exec_reload: str = ""
    working_directory: str = os.getcwd()
    environment: Dict[str, str] = field(default_factory=dict)
    user: str = ""
    group: str = ""
    type: ServiceType = ServiceType.SIMPLE
    restart: str = "on-failure"
    restart_sec: int = 1
    timeout_start_sec: int = 90
    timeout_stop_sec: int = 10
    pid_file: str = ""
    auto_start: bool = True
    dependencies: List[str] = field(default_factory=list)
    wanted_by: List[str] = field(default_factory=list)
    required_by: List[str] = field(default_factory=list)
    before: List[str] = field(default_factory=list)
    after: List[str] = field(default_factory=list)
    wants: List[str] = field(default_factory=list)
    requires: List[str] = field(default_factory=list)
    conflicts: List[str] = field(default_factory=list)

@dataclass
class ServiceStatus:
    name: str
    state: ServiceState
    pid: int = 0
    exit_code: Optional[int] = None
    start_time: float = 0.0
    uptime: float = 0.0
    restart_count: int = 0
    last_error: str = ""

class ServiceManager:
    def __init__(self, config_dir: str = "/etc/services"):
        self.config_dir = Path(config_dir)
        self.services: Dict[str, ServiceConfig] = {}
        self.processes: Dict[str, subprocess.Popen] = {}
        self.status: Dict[str, ServiceStatus] = {}
        self.lock = threading.RLock()
        self.running = False
        self.threads: Dict[str, threading.Thread] = {}
        
        # Ensure config directory exists
        self.config_dir.mkdir(parents=True, exist_ok=True)
        
        # Register cleanup on exit
        atexit.register(self.stop_all_services)
    
    def load_service(self, name: str) -> bool:
        """Load a service configuration from a JSON file."""
        config_file = self.config_dir / f"{name}.json"
        
        if not config_file.exists():
            logger.error(f"Service config not found: {config_file}")
            return False
        
        try:
            with open(config_file, 'r') as f:
                data = json.load(f)
            
            config = ServiceConfig(
                name=data.get('name', name),
                description=data.get('description', ''),
                exec_start=data['exec_start'],
                exec_stop=data.get('exec_stop', ''),
                exec_reload=data.get('exec_reload', ''),
                working_directory=data.get('working_directory', os.getcwd()),
                environment=data.get('environment', {}),
                user=data.get('user', ''),
                group=data.get('group', ''),
                type=ServiceType[data.get('type', 'SIMPLE')],
                restart=data.get('restart', 'on-failure'),
                restart_sec=data.get('restart_sec', 1),
                timeout_start_sec=data.get('timeout_start_sec', 90),
                timeout_stop_sec=data.get('timeout_stop_sec', 10),
                pid_file=data.get('pid_file', ''),
                auto_start=data.get('auto_start', True),
                dependencies=data.get('dependencies', []),
                wanted_by=data.get('wanted_by', []),
                required_by=data.get('required_by', []),
                before=data.get('before', []),
                after=data.get('after', []),
                wants=data.get('wants', []),
                requires=data.get('requires', []),
                conflicts=data.get('conflicts', []),
            )
            
            with self.lock:
                self.services[name] = config
                if name not in self.status:
                    self.status[name] = ServiceStatus(name=name, state=ServiceState.STOPPED)
            
            logger.info(f"Loaded service: {name}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to load service {name}: {e}")
            return False
    
    def load_all_services(self) -> None:
        """Load all service configurations from the config directory."""
        for config_file in self.config_dir.glob("*.json"):
            name = config_file.stem
            self.load_service(name)
    
    def start_service(self, name: str) -> bool:
        """Start a service by name."""
        with self.lock:
            if name not in self.services:
                logger.error(f"Service not found: {name}")
                return False
                
            if name in self.processes:
                logger.warning(f"Service {name} is already running")
                return True
            
            config = self.services[name]
            self.status[name] = ServiceStatus(
                name=name,
                state=ServiceState.STARTING,
                start_time=time.time()
            )
            
            try:
                # Prepare environment
                env = os.environ.copy()
                env.update(config.environment)
                
                # Split command and arguments
                cmd = shlex.split(config.exec_start)
                
                # Start the process
                process = subprocess.Popen(
                    cmd,
                    cwd=config.working_directory,
                    env=env,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    shell=False,
                    start_new_session=True
                )
                
                self.processes[name] = process
                self.status[name].pid = process.pid
                self.status[name].state = ServiceState.RUNNING
                
                # Start monitoring thread
                thread = threading.Thread(
                    target=self._monitor_service,
                    args=(name, process),
                    daemon=True
                )
                self.threads[name] = thread
                thread.start()
                
                logger.info(f"Started service {name} (PID: {process.pid})")
                return True
                
            except Exception as e:
                logger.error(f"Failed to start service {name}: {e}")
                self.status[name].state = ServiceState.FAILED
                self.status[name].last_error = str(e)
                return False
    
    def stop_service(self, name: str, force: bool = False) -> bool:
        """Stop a running service."""
        with self.lock:
            if name not in self.processes:
                logger.warning(f"Service {name} is not running")
                return True
                
            process = self.processes[name]
            self.status[name].state = ServiceState.STOPPING
            
            try:
                if force:
                    process.terminate()
                    try:
                        process.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        process.kill()
                else:
                    # Try graceful shutdown first
                    process.terminate()
                    try:
                        process.wait(timeout=10)
                    except subprocess.TimeoutExpired:
                        process.kill()
                
                # Clean up
                if name in self.processes:
                    del self.processes[name]
                if name in self.threads:
                    del self.threads[name]
                
                self.status[name].state = ServiceState.STOPPED
                self.status[name].uptime = time.time() - self.status[name].start_time
                
                logger.info(f"Stopped service {name}")
                return True
                
            except Exception as e:
                logger.error(f"Error stopping service {name}: {e}")
                self.status[name].state = ServiceState.FAILED
                self.status[name].last_error = str(e)
                return False
    
    def restart_service(self, name: str) -> bool:
        """Restart a service."""
        with self.lock:
            if name in self.processes:
                if not self.stop_service(name):
                    return False
            return self.start_service(name)
    
    def reload_service(self, name: str) -> bool:
        """Reload a service configuration."""
        with self.lock:
            if name not in self.services:
                logger.error(f"Service not found: {name}")
                return False
                
            config = self.services[name]
            
            if not config.exec_reload:
                logger.warning(f"No reload command defined for service {name}")
                return self.restart_service(name)
            
            try:
                # Execute reload command
                subprocess.run(
                    shlex.split(config.exec_reload),
                    cwd=config.working_directory,
                    env=os.environ.copy(),
                    check=True
                )
                
                logger.info(f"Reloaded service {name}")
                return True
                
            except subprocess.CalledProcessError as e:
                logger.error(f"Failed to reload service {name}: {e}")
                return False
    
    def get_service_status(self, name: str) -> Optional[ServiceStatus]:
        """Get the status of a service."""
        with self.lock:
            if name not in self.status:
                return None
                
            status = self.status[name]
            
            # Update uptime if running
            if status.state == ServiceState.RUNNING and status.start_time > 0:
                status.uptime = time.time() - status.start_time
            
            return status
    
    def list_services(self) -> List[ServiceStatus]:
        """Get the status of all services."""
        with self.lock:
            return list(self.status.values())
    
    def start_all_services(self) -> None:
        """Start all auto-start services."""
        with self.lock:
            for name, config in self.services.items():
                if config.auto_start:
                    self.start_service(name)
    
    def stop_all_services(self) -> None:
        """Stop all running services."""
        with self.lock:
            for name in list(self.processes.keys()):
                self.stop_service(name, force=True)
    
    def _monitor_service(self, name: str, process: subprocess.Popen) -> None:
        """Monitor a service process and handle restarts."""
        while True:
            try:
                # Wait for process to complete
                return_code = process.wait()
                
                with self.lock:
                    self.status[name].exit_code = return_code
                    self.status[name].uptime = time.time() - self.status[name].start_time
                    self.status[name].restart_count += 1
                    
                    # Check if we should restart
                    config = self.services[name]
                    should_restart = (
                        (config.restart == 'always') or
                        (config.restart == 'on-failure' and return_code != 0) or
                        (config.restart == 'unless-stopped' and return_code != 0)
                    )
                    
                    if should_restart:
                        logger.info(f"Service {name} exited with code {return_code}, restarting...")
                        time.sleep(config.restart_sec)
                        self.start_service(name)
                    else:
                        logger.info(f"Service {name} exited with code {return_code}, not restarting")
                        self.status[name].state = ServiceState.STOPPED
                        if name in self.processes:
                            del self.processes[name]
                        if name in self.threads:
                            del self.threads[name]
                    
                    break
                    
            except Exception as e:
                logger.error(f"Error monitoring service {name}: {e}")
                with self.lock:
                    self.status[name].state = ServiceState.FAILED
                    self.status[name].last_error = str(e)
                    if name in self.processes:
                        del self.processes[name]
                    if name in self.threads:
                        del self.threads[name]
                break
    
    def create_service_config(self, config: ServiceConfig) -> bool:
        """Create a new service configuration file."""
        try:
            config_file = self.config_dir / f"{config.name}.json"
            
            # Convert enums to strings
            data = {
                key: value.name if isinstance(value, (ServiceType, ServiceState)) else value
                for key, value in config.__dict__.items()
                if not key.startswith('_')
            }
            
            with open(config_file, 'w') as f:
                json.dump(data, f, indent=4)
            
            # Reload the service
            return self.load_service(config.name)
            
        except Exception as e:
            logger.error(f"Failed to create service config {config.name}: {e}")
            return False
    
    def remove_service(self, name: str) -> bool:
        """Remove a service configuration."""
        with self.lock:
            if name in self.processes:
                self.stop_service(name, force=True)
            
            config_file = self.config_dir / f"{name}.json"
            
            try:
                if config_file.exists():
                    config_file.unlink()
                
                if name in self.services:
                    del self.services[name]
                if name in self.status:
                    del self.status[name]
                
                logger.info(f"Removed service: {name}")
                return True
                
            except Exception as e:
                logger.error(f"Failed to remove service {name}: {e}")
                return False
