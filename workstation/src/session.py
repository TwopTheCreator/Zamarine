import json
from datetime import datetime
from uuid import uuid4

class Session:
    def __init__(self, user):
        self.user = user
        self.session_id = str(uuid4())
        self.start_time = datetime.now()
        self.data = {}

    def set_data(self, key, value):
        self.data[key] = value

    def get_data(self, key, default=None):
        return self.data.get(key, default)

    def to_dict(self):
        return {
            "user": self.user.username,
            "role": self.user.role,
            "session_id": self.session_id,
            "start_time": self.start_time.isoformat(),
            "data": self.data
        }

class SessionManager:
    def __init__(self, file_path="sessions.json"):
        self.active_sessions = {}
        self.file_path = file_path

    def start_session(self, user):
        session = Session(user)
        self.active_sessions[session.session_id] = session
        print(f"Session started: {session.session_id} for user {user.username}")
        return session.session_id

    def end_session(self, session_id):
        if session_id in self.active_sessions:
            session = self.active_sessions.pop(session_id)
            print(f"Session ended: {session.session_id} for user {session.user.username}")
            return session
        return None

    def save_sessions(self):
        """Save all active sessions to JSON file"""
        with open(self.file_path, "w") as f:
            data = {sid: session.to_dict() for sid, session in self.active_sessions.items()}
            json.dump(data, f, indent=4)
        print(f"Saved {len(self.active_sessions)} sessions to {self.file_path}")

    def load_sessions(self):
        """Load sessions from JSON file"""
        try:
            with open(self.file_path, "r") as f:
                data = json.load(f)
            for sid, session_data in data.items():
                session = Session(User(session_data["user"], session_data["role"]))
                session.session_id = session_data["session_id"]
                session.start_time = datetime.fromisoformat(session_data["start_time"])
                session.data = session_data["data"]
                self.active_sessions[sid] = session
            print(f"Loaded {len(self.active_sessions)} sessions from {self.file_path}")
        except FileNotFoundError:
            print("No previous session file found.")
