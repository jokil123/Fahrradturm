from __future__ import annotations
from dataclasses import dataclass
from queue import Queue
from google.cloud.firestore_v1.client import Client
from firebase_admin import App

# TODO: Why is python so ugly? 🤮🤮🤮
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from job import Job

from tower import Tower


@dataclass()
class Context:
    app: App
    db: Client
    job_queue: Queue[Job]
    tower_id: str
    tower: Tower
