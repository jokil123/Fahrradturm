from dataclasses import dataclass
from queue import Queue
from google.cloud.firestore_v1.client import Client

from queue_message import QueueMessage


@dataclass
class Context:
    queue: Queue[QueueMessage]
    db: Client
    tower_id: str
