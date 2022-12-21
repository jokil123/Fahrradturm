import asyncio
from threading import Thread
import threading
from time import sleep

from event import Event


def callback_function_content(cb):
    sleep(5)
    print("calling callback")
    cb()


def callback_function(cb):
    Thread(target=callback_function_content, args=([cb])).start()


async def main():
    event = threading.Event()

    callback_function(lambda: event.set())

    print("a")
    event.wait()
    print("b")


asyncio.run(main())
