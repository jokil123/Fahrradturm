import asyncio
from enum import Enum
import os
from queue import Queue
from threading import Thread
import threading
from time import sleep
from typing import Any, Callable, Iterable, Mapping
import aioconsole
from dotenv import load_dotenv

import firebase_admin
from firebase_admin import credentials
from firebase_admin import firestore
from google.cloud.firestore_v1.client import Client
from gui_display import GUIDisplay

from job import Job
from joboperator import JobOperator
from tower import Tower


async def main():
    load_dotenv()
    print("Starting up...")

    cred = credentials.Certificate("adminsdk.json")
    app = firebase_admin.initialize_app(cred)
    db: Client = firestore.client(app)

    tower_id = os.getenv("TOWER_ID")
    if not tower_id:
        raise Exception("TOWER_ID not found")

    job_queue: Queue[Job] = Queue()

    job_operator = JobOperator()

    while True:
        print("Waiting for job...")
        job = job_queue.get()
        print("Got job: ", job)
        print("Executing job...")
        await job_operator.execute(job)
        print("Job executed")

    print("Shutting down...")

if __name__ == "__main__":
    # asyncio.run(main())
    tower = Tower(5, 5, GUIDisplay())
    box = tower.get_empty_box()

    tower.retrieve_box(box)

    sleep(3)

    print("storing box")
    tower.store_box(box)


class JobProvider(Thread):
    job_queue: Queue[Job]

    def __init__(self, job_queue: Queue[Job], group: None, target: Callable[..., object] | None, name: str | None, args: Iterable[Any], kwargs: Mapping[str, Any] | None, *, daemon: bool | None) -> None:
        super().__init__(group, target, name, args, kwargs, daemon=daemon)
        self.job_queue = job_queue

    def run(self) -> None:
        pass
