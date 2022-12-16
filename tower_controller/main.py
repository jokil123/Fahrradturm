import asyncio
import os
import aioconsole
from dotenv import load_dotenv

import firebase_admin
from firebase_admin import credentials
from firebase_admin import firestore
from google.cloud.firestore_v1.client import Client

import classes


def main():
    # print("Starting up...")
    # load_dotenv()

    # cred = credentials.Certificate("adminsdk.json")
    # app = firebase_admin.initialize_app(cred)

    # db: Client = firestore.client(app)

    # tower_id = os.getenv("TOWER_ID")
    # if not tower_id:
    #     raise Exception("Tower ID not found")

    # tower_watch = db.collection("towers").document(
    #     tower_id).on_snapshot(on_snapshot)

    # while True:
    #     match input():
    #         case "q":
    #             break
    #         case "h":
    #             print_help()

    # tower_watch.close()
    # print("Shutting down...")

    loop = asyncio.new_event_loop()
    task_set = set()

    task_set.add(loop.create_task(asyncio.sleep(10)))
    task_set.add(loop.create_task(aioconsole.ainput()))

    first, pending = loop.run_until_complete(
        asyncio.wait(task_set, return_when=asyncio.FIRST_COMPLETED)
    )

    for task in pending:
        task.cancel()

    for task in first:
        print(task.result())

    loop.close()

    print("Done")


def on_snapshot(doc_snapshot, changes, read_time):
    for doc in doc_snapshot:
        print(f"Received document snapshot: {doc.id}")


def print_help():
    print("q - quit")
    print("h - help")


if __name__ == "__main__":
    # asyncio.run(main())
    main()
