from __future__ import annotations
from abc import ABC, abstractmethod
import asyncio

from context import Context
from event import ConsoleInputEvent, DatabaseStoreEvent, TimeoutEvent, listen_to


class State(ABC):
    context: Context

    async def next(self) -> State:
        return self

    def __init__(self, context: Context) -> None:
        self.context = context


class Idle(State):
    async def next(self) -> State:
        print("Idle")

        event = await listen_to(events=[
            ConsoleInputEvent(),
            DatabaseStoreEvent(
                self.context.db, self.context.tower_id)])

        match event:
            case ConsoleInputEvent():
                print(event.input_value)
                match event.input_value:
                    case "q" | "quit" | "exit":
                        return Exiting(self.context)
            case DatabaseStoreEvent():
                print(event.snap)
                return UserInsertingBicycle(self.context)
            case _:
                raise Exception("Unknown message type")

        # catch all
        return self


class Exiting(State):
    pass


class UserInsertingBicycle(State):
    async def next(self) -> State:
        print("UserInsertingBicycle")

        event = await listen_to(events=[TimeoutEvent(1)])

        return self

    def __init__(self, context) -> None:
        super().__init__(context)
        # self.user = user


class StoringBicycle(State):
    pass


class RetrievingBicycle(State):
    pass


class UserRemovingBicycle(State):
    pass


class IdleAnimation(State):
    pass
