import os
import json
import threading
import time
from datetime import datetime
from queue import Queue
from uuid import uuid4

class Logger:
    def __init__(self, log_file="system.log"):
        self.log_file = log_file
        self.lock = threading.Lock()

    def log(self, message, level="INFO"):
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        entry = f"[{timestamp}] [{level}] {message}\n"
        with self.lock:
            with open(self.log_file, "a") as f:
                f.write(entry)
        print(entry, end="")

logger = Logger()

class User:
    def __init__(self, username, role="user"):
        self.username = username
        self.role = role
        self.session_id = None

    def start_session(self):
        self.session_id = str(uuid4())
        logger.log(f"User {self.username} started session {self.session_id}")
        return self.session_id

    def end_session(self):
        logger.log(f"User {self.username} ended session {self.session_id}")
        self.session_id = None

class UserManager:
    def __init__(self):
        self.users = {}
    
    def add_user(self, username, role="user"):
        if username in self.users:
            logger.log(f"Attempted to add existing user: {username}", "WARNING")
            return False
        self.users[username] = User(username, role)
        logger.log(f"Added user: {username} with role {role}")
        return True

    def remove_user(self, username):
        if username not in self.users:
            logger.log(f"Attempted to remove non-existing user: {username}", "WARNING")
            return False
        del self.users[username]
        logger.log(f"Removed user: {username}")
        return True

user_manager = UserManager()

class VirtualFile:
    def __init__(self, name, content=""):
        self.name = name
        self.content = content
        self.created_at = datetime.now()
        self.modified_at = self.created_at

    def write(self, content):
        self.content = content
        self.modified_at = datetime.now()
        logger.log(f"File {self.name} updated")

    def read(self):
        logger.log(f"File {self.name} read")
        return self.content

class FileSystem:
    def __init__(self):
        self.files = {}

    def create_file(self, name, content=""):
        if name in self.files:
            logger.log(f"Attempted to create existing file: {name}", "WARNING")
            return False
        self.files[name] = VirtualFile(name, content)
        logger.log(f"File {name} created")
        return True

    def delete_file(self, name):
        if name not in self.files:
            logger.log(f"Attempted to delete non-existing file: {name}", "WARNING")
            return False
        del self.files[name]
        logger.log(f"File {name} deleted")
        return True

fs = FileSystem()

class Task(threading.Thread):
    def __init__(self, name, func, *args, **kwargs):
        super().__init__()
        self.name = name
        self.func = func
        self.args = args
        self.kwargs = kwargs
        self.daemon = True
        logger.log(f"Task {self.name} initialized")

    def run(self):
        logger.log(f"Task {self.name} started")
        try:
            self.func(*self.args, **self.kwargs)
            logger.log(f"Task {self.name} completed")
        except Exception as e:
            logger.log(f"Task {self.name} failed: {e}", "ERROR")

class Scheduler:
    def __init__(self):
        self.tasks = []
        self.queue = Queue()

    def schedule(self, task):
        self.tasks.append(task)
        self.queue.put(task)
        logger.log(f"Task {task.name} scheduled")

    def run(self):
        logger.log("Scheduler started")
        while not self.queue.empty():
            task = self.queue.get()
            task.start()
            task.join()

scheduler = Scheduler()

class Event:
    def __init__(self, name, data=None):
        self.name = name
        self.data = data or {}
        logger.log(f"Event created: {self.name}")

class EventBus:
    def __init__(self):
        self.listeners = {}

    def register(self, event_name, callback):
        if event_name not in self.listeners:
            self.listeners[event_name] = []
        self.listeners[event_name].append(callback)
        logger.log(f"Listener registered for event: {event_name}")

    def emit(self, event):
        logger.log(f"Event emitted: {event.name}")
        for callback in self.listeners.get(event.name, []):
            try:
                callback(event)
            except Exception as e:
                logger.log(f"Event listener failed: {e}", "ERROR")

event_bus = EventBus()
