from __future__ import annotations
from abc import ABC, abstractmethod
import asyncio

from context import Context
from event import ConsoleInputEvent, DatabaseStoreEvent, KeyboardButtonPressedEvent, TimeoutEvent, listen_to


class State(ABC):
    context: Context

    async def next(self) -> State:
        print(self.__class__.__name__)

        event = await listen_to(events=[TimeoutEvent(1)])

        return self

    def __init__(self, context: Context) -> None:
        self.context = context


class Idle(State):
    async def next(self) -> State:
        print("Idle")

        event = await listen_to(events=[
            # ConsoleInputEvent(),
            DatabaseStoreEvent(
                self.context.db, self.context.tower_id),
            # TimeoutEvent(10)
        ])

        match event:
            case ConsoleInputEvent():
                print(event.input_value)
                match event.input_value:
                    case "q" | "quit" | "exit":
                        return Exiting(self.context)
            case DatabaseStoreEvent():
                print(event.snap)
                return UserInsertingBicycle(self.context)
            case TimeoutEvent():
                return IdleAnimation(self.context)
            case _:
                raise Exception("Unknown message type")

        # catch all
        return self


class Exiting(State):
    pass


class UserInsertingBicycle(State):
    async def next(self) -> State:
        print("UserInsertingBicycle")

        event = await listen_to(events=[TimeoutEvent(2),
                                        KeyboardButtonPressedEvent("a"),
                                        ])
        print(event)
        match event:
            case TimeoutEvent():
                return Idle(self.context)
            case KeyboardButtonPressedEvent():
                return StoringBicycle(self.context)

        return self

    def __init__(self, context) -> None:
        super().__init__(context)
        # self.user = user


class StoringBicycle(State):
    async def next(self) -> State:
        print("StoringBicycle")

        event = await listen_to(events=[TimeoutEvent(5)])

        print("StoringBicycle done")

        return Idle(self.context)


class RetrievingBicycle(State):
    pass


class UserRemovingBicycle(State):
    pass


class IdleAnimation(State):
    pass
