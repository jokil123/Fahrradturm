from __future__ import annotations
from context import Context
from pytimedinput import timedInput


class Job():
    async def execute(self, context: Context):
        pass


class StoreBicycleJob(Job):
    def __init__(self, user_id: str):
        self.user_id = user_id

    async def execute(self, context: Context):
        print("Storing bicycle with id: ", self.user_id)

        # TODO: No empty boxes might be available
        box = context.tower.get_empty_box()
        if not box:
            raise Exception("No empty box available")

        text, timeOut = timedInput(
            "Please confirm you have stored the bicycle in box " + str(box) + " (y/n): ", 10)

        if text == "y":
            context.tower.rent_box(box, self.user_id)
            print("Bicycle stored")

        context.tower.store_box(box)
