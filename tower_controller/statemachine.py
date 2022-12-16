from __future__ import annotations
from abc import ABC, abstractmethod
import asyncio

import aioconsole

from context import Context


class State(ABC):
    context: Context

    async def next(self) -> State:
        return self

    def __init__(self, context: Context) -> None:
        self.context = context


class Idle(State):
    async def next(self) -> State:
        print("Idle")

        loop = asyncio.new_event_loop()
        task_set = set()

        task_set.add(loop.create_task(asyncio.sleep(10)))
        task_set.add(loop.create_task(aioconsole.ainput()))

        done, pending = await asyncio.wait(task_set, return_when=asyncio.FIRST_COMPLETED)

        match input():
            case "insert":
                return UserInsertingBicycle(self.context)
            case "retrieve":
                return UserRemovingBicycle(self.context)
        # catch all
        return self


class UserInsertingBicycle(State):
    def next(self) -> State:
        pass

    def __init__(self, user) -> None:
        super().__init__()
        self.user = user


class StoringBicycle(State):
    def next(self) -> State:
        pass


class RetrievingBicycle(State):
    def next(self) -> State:
        pass


class UserRemovingBicycle(State):
    def next(self) -> State:
        pass


class IdleAnimation(State):
    def next(self) -> State:
        pass
