from __future__ import annotations
import aioconsole
import asyncio
import threading
from typing import Any
from xmlrpc.client import Boolean
from google.cloud.firestore_v1.client import Client
from asyncio.futures import Future

from google.cloud.firestore_v1.watch import DocumentChange
from google.cloud.firestore_v1.base_document import DocumentSnapshot

from proto.datetime_helpers import DatetimeWithNanoseconds


from console.utils import wait_key


class Event():
    async def subscribe(self) -> Event:
        return self

    def get_event_name(self) -> str:
        return self.__class__.__name__


class ButtonPressedEvent(Event):
    async def subscribe(self) -> ButtonPressedEvent:
        await asyncio.sleep(2)
        return self


class ConsoleInputEvent(Event):
    input_value: str

    async def subscribe(self) -> ConsoleInputEvent:
        self.input_value = await aioconsole.ainput()
        return self


class DatabaseStoreEvent(Event):
    db: Client
    tower_id: str

    # TODO: fix these types
    snap: DocumentSnapshot
    diff: DocumentChange
    time: DatetimeWithNanoseconds

    first: DocumentSnapshot | None = None

    def __init__(self, db: Client, tower_id: str) -> None:
        self.db = db
        self.tower_id = tower_id
        self.event = asyncio.Event()

    async def subscribe(self) -> DatabaseStoreEvent:
        event = threading.Event()

        def event_callback(snap: DocumentSnapshot, diff: DocumentChange, time: DatetimeWithNanoseconds):
            if not self.first:
                self.first = snap
                return

            self.snap = snap
            self.diff = diff
            self.time = time

            event.set()

        watch = self.db.collection("towers").document(
            self.tower_id).on_snapshot(event_callback)

        event.wait()
        watch.close()
        return self

    def get_event_name(self) -> str:
        return super().get_event_name() + " " + self.tower_id


class DatabaseRetrieveEvent(Event):
    pass


class TimeoutEvent(Event):
    seconds: int

    def __init__(self, seconds: int) -> None:
        self.seconds = seconds

    async def subscribe(self) -> TimeoutEvent:
        await asyncio.sleep(self.seconds)
        return self

    def get_event_name(self) -> str:
        return super().get_event_name() + " " + str(self.seconds)


class KeyboardButtonPressedEvent(Event):
    button: str
    result: str | None

    def __init__(self, button: str) -> None:
        self.button = button

    async def subscribe(self) -> KeyboardButtonPressedEvent:
        self.result = wait_key(self.button)

        return self

    def get_event_name(self) -> str:
        return super().get_event_name() + " " + self.button


async def listen_to(events: list[Event]) -> Event:
    tasks = map(lambda e: asyncio.create_task(e.subscribe()), events)
    task_names = map(lambda e: e.get_event_name(), events)

    # for task, task_name in zip(tasks, task_names):
    #     task.set_name(task_name)

    done, pending = await asyncio.wait(tasks, return_when=asyncio.FIRST_COMPLETED)

    for task in pending:
        task.cancel()

    return done.pop().result()
