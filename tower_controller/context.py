from dataclasses import dataclass
from queue import Queue
from google.cloud.firestore_v1.client import Client


@dataclass
class Context:
    db: Client
    tower_id: str
