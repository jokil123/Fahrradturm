from __future__ import annotations
from abc import ABC, abstractmethod

from context import Context
from queue_message import ConsoleInput, FirebaseUpdate, RetrieveBicycle, StoreBicycle


class State(ABC):
    context: Context

    async def next(self) -> State:
        return self

    def __init__(self, context: Context) -> None:
        self.context = context


class Idle(State):
    async def next(self) -> State:
        print("Idle")

        message = self.context.queue.get()

        match message:
            case ConsoleInput():
                match message.input:
                    case "help" | "h":
                        print("helptext.txt lol")
                    case "exit" | "quit" | "q" | "EOF":
                        return Exiting(self.context)
                    case _:
                        print("Unknown command: {}".format(message.input))
            case StoreBicycle():
                print(message)
                return StoringBicycle(self.context)
            case RetrieveBicycle():
                print(message)
                return RetrievingBicycle(self.context)
            case _:
                raise Exception("Unknown message type")

        # catch all
        return self


class Exiting(State):
    pass


class UserInsertingBicycle(State):
    async def next(self) -> State:
        return self

    def __init__(self, context, user) -> None:
        super().__init__(context)
        self.user = user


class StoringBicycle(State):
    pass


class RetrievingBicycle(State):
    pass


class UserRemovingBicycle(State):
    pass


class IdleAnimation(State):
    pass
