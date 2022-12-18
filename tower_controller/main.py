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

import classes
from queue_message import ConsoleInput, FirebaseUpdate, QueueMessage


def main():
    load_dotenv()
    print("Starting up...")

    queue: Queue[QueueMessage] = Queue()

    quit_event: threading.Event = threading.Event()
    Thread(target=listen_for_console_input,
           args=[queue, quit_event], name="console").start()
    Thread(target=listen_for_firebase_updates,
           args=[queue, quit_event], name="firebase").start()

    while True:
        message = queue.get()

        match message:
            case ConsoleInput():
                match message.input:
                    case "help" | "h":
                        print("helptext.txt lol")
                    case "exit" | "quit" | "q" | "EOF":
                        break
                    case _:
                        print(format("Unknown command: {message.input}"))
            case FirebaseUpdate():
                print(message)
            case _:
                raise Exception("Unknown message type")

    print("Shutting down...")
    quit_event.set()


def listen_for_console_input(queue: Queue[QueueMessage], quit_event: threading.Event):
    while True and not quit_event.is_set():
        try:
            i = input()
        except EOFError:
            i = "EOF"
        queue.put(ConsoleInput(i))

    print("Console input thread shutting down...")


def listen_for_firebase_updates(queue: Queue[QueueMessage], quit_event: threading.Event):
    pass


if __name__ == "__main__":
    main()
