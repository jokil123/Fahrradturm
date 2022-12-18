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

import db_model
from listeners import listen_for_console_input, listen_for_firebase_updates
from queue_message import ConsoleInput, FirebaseUpdate, QueueMessage
from context import Context
import statemachine


async def main():
    load_dotenv()
    print("Starting up...")

    cred = credentials.Certificate("adminsdk.json")
    app = firebase_admin.initialize_app(cred)
    db: Client = firestore.client(app)

    tower_id = os.getenv("TOWER_ID")
    if not tower_id:
        raise Exception("TOWER_ID not found")

    # queue for async input from different sources
    queue: Queue[QueueMessage] = Queue()
    quit_event = threading.Event()

    Thread(target=listen_for_console_input,
           args=[queue, quit_event], name="console").start()
    Thread(target=listen_for_firebase_updates,
           args=[queue, quit_event, db, tower_id], name="firebase").start()

    state: statemachine.State = statemachine.Idle(Context(queue, db, tower_id))

    # main thread handles input from queue
    while True:
        state = await state.next()

        if isinstance(state, statemachine.Exiting):
            break

    print("Shutting down...")
    quit_event.set()


if __name__ == "__main__":
    asyncio.get_event_loop().run_until_complete(main())
