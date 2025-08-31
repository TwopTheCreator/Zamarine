import os
import signal
import subprocess
import threading
import time
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, field
from enum import Enum, auto
import json
import ctypes
from pathlib import Path

class ProcessState(Enum):
    RUNNING = auto()
    SLEEPING = auto()
    STOPPED = auto()
    ZOMBIE = auto()
    DEAD = auto()

@dataclass
class ProcessInfo:
    pid: int
    ppid: int
    name: str
    state: ProcessState
    cpu_usage: float = 0.0
    memory_usage: float = 0.0
    start_time: float = field(default_factory=time.time)
    priority: int = 0
    command: List[str] = field(default_factory=list)
    environment: Dict[str, str] = field(default_factory=dict)

class ProcessManager:
    def __init__(self):
        self.processes: Dict[int, ProcessInfo] = {}
        self._lock = threading.RLock()
        self._next_pid = 1000  # Start assigning PIDs from 1000
        self._process_groups: Dict[int, List[int]] = {}
        self._session_leaders: Dict[int, int] = {}
        
    def create_process(self, command: List[str], env: Optional[Dict[str, str]] = None) -> int:
        """Create a new process with the given command and environment."""
        with self._lock:
            pid = self._next_pid
            self._next_pid += 1
            
            process_info = ProcessInfo(
                pid=pid,
                ppid=os.getpid(),
                name=os.path.basename(command[0]),
                state=ProcessState.RUNNING,
                command=command.copy(),
                environment=env.copy() if env else {}
            )
            
            self.processes[pid] = process_info
            return pid
    
    def terminate_process(self, pid: int, force: bool = False) -> bool:
        """Terminate a process by PID."""
        with self._lock:
            if pid not in self.processes:
                return False
                
            process_info = self.processes[pid]
            
            try:
                if force:
                    os.kill(pid, signal.SIGKILL)
                else:
                    os.kill(pid, signal.SIGTERM)
                
                process_info.state = ProcessState.DEAD
                return True
                
            except ProcessLookupError:
                return False
    
    def get_process_info(self, pid: int) -> Optional[ProcessInfo]:
        """Get information about a process by PID."""
        with self._lock:
            return self.processes.get(pid)
    
    def list_processes(self) -> List[ProcessInfo]:
        """Get a list of all processes."""
        with self._lock:
            return list(self.processes.values())
    
    def update_process_state(self, pid: int, state: ProcessState) -> bool:
        """Update the state of a process."""
        with self._lock:
            if pid in self.processes:
                self.processes[pid].state = state
                return True
            return False
    
    def set_process_priority(self, pid: int, priority: int) -> bool:
        """Set the priority of a process."""
        with self._lock:
            if pid in self.processes:
                self.processes[pid].priority = priority
                
                try:
                    if os.name == 'posix':
                        os.nice(priority)
                    elif os.name == 'nt':
                        kernel32 = ctypes.windll.kernel32
                        kernel32.SetPriorityClass(
                            kernel32.GetCurrentProcess(),
                            self._convert_priority_to_windows(priority)
                        )
                except Exception:
                    pass
                    
                return True
            return False
    
    def _convert_priority_to_windows(self, priority: int) -> int:
        """Convert Unix nice values to Windows priority classes."""
        if priority < -15:
            return 0x00000080  # HIGH_PRIORITY_CLASS
        elif priority < 0:
            return 0x00008000  # ABOVE_NORMAL_PRIORITY_CLASS
        elif priority == 0:
            return 0x00000020  # NORMAL_PRIORITY_CLASS
        elif priority < 10:
            return 0x00004000  # BELOW_NORMAL_PRIORITY_CLASS
        else:
            return 0x00000040  # IDLE_PRIORITY_CLASS
    
    def create_process_group(self, pids: List[int]) -> int:
        """Create a new process group with the given PIDs."""
        with self._lock:
            pgid = self._next_pid
            self._next_pid += 1
            self._process_groups[pgid] = pids.copy()
            return pgid
    
    def send_signal_to_group(self, pgid: int, signal_num: int) -> bool:
        """Send a signal to all processes in a process group."""
        with self._lock:
            if pgid not in self._process_groups:
                return False
                
            success = True
            for pid in self._process_groups[pgid]:
                try:
                    os.kill(pid, signal_num)
                except ProcessLookupError:
                    success = False
                    
            return success
    
    def create_session(self, pid: int) -> int:
        """Create a new session with the given PID as the session leader."""
        with self._lock:
            if pid not in self.processes:
                return -1
                
            sid = self._next_pid
            self._next_pid += 1
            self._session_leaders[sid] = pid
            return sid
