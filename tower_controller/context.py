from dataclasses import dataclass
from google.cloud.firestore_v1.client import Client


@dataclass
class Context:
    db: Client
