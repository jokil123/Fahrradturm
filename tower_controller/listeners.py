from queue import Queue
from threading import Event
from queue_message import ConsoleInput, FirebaseUpdate, QueueMessage
from google.cloud.firestore_v1.client import Client

# improve this entire function


def listen_for_console_input(queue: Queue[QueueMessage], quit_event: Event):
    while not quit_event.is_set():
        try:
            # TODO: this blocks when exiting. Figure out how to fix that
            i = input()
            queue.put(ConsoleInput(i))
        except (EOFError, KeyboardInterrupt):
            queue.put(ConsoleInput("EOF"))
            quit_event.set()
            break

    print("Console input thread shutting down...")


def listen_for_firebase_updates(queue: Queue[QueueMessage], quit_event: Event, db: Client, tower_id: str):
    tower_watch = db.collection("towers").document(
        tower_id).on_snapshot(lambda snap, diff, time: queue.put(FirebaseUpdate(snap, diff, time)))

    quit_event.wait()
    tower_watch.close()
    print("Firebase thread shutting down...")
