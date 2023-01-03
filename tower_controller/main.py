import asyncio
from enum import Enum
import os
from queue import Queue
from threading import Thread
import threading
import aioconsole
from dotenv import load_dotenv

import firebase_admin
from firebase_admin import credentials
from firebase_admin import firestore
from google.cloud.firestore_v1.client import Client

from job import Job
from joboperator import JobOperator


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
    asyncio.run(main())
