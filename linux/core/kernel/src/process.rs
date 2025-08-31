use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::Mutex;
use crate::sync::SpinLock;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(u64);

#[derive(Debug)]
pub struct Process {
    pub pid: ProcessId,
    pub name: String,
    pub state: ProcessState,
    pub context: ProcessContext,
    pub memory_regions: Vec<MemoryRegion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Zombie,
}

#[derive(Debug)]
pub struct ProcessContext {
    pub registers: Registers,
    pub stack_pointer: usize,
    pub instruction_pointer: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
}

#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: usize,
    pub size: usize,
    pub permissions: MemoryPermissions,
    pub flags: MemoryFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPermissions {
    Read,
    Write,
    Execute,
    ReadWrite,
    ReadExecute,
    ReadWriteExecute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryFlags {
    pub shared: bool,
    pub locked: bool,
    pub no_reserve: bool,
}

pub struct ProcessManager {
    processes: SpinLock<Vec<Arc<Mutex<Process>>>>,
    next_pid: SpinLock<u64>,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: SpinLock::new(Vec::new()),
            next_pid: SpinLock::new(1),
        }
    }

    pub fn create_process(&self, name: &str) -> ProcessId {
        let mut processes = self.processes.lock();
        let pid = ProcessId(self.allocate_pid());
        
        let process = Process {
            pid: pid.clone(),
            name: String::from(name),
            state: ProcessState::Ready,
            context: ProcessContext {
                registers: Registers::default(),
                stack_pointer: 0,
                instruction_pointer: 0,
            },
            memory_regions: Vec::new(),
        };
        
        processes.push(Arc::new(Mutex::new(process)));
        pid
    }
    
    fn allocate_pid(&self) -> u64 {
        let mut next_pid = self.next_pid.lock();
        let pid = *next_pid;
        *next_pid += 1;
        pid
    }
}
