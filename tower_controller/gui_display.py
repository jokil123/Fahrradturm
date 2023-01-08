
from threading import Thread
from tower_display import TowerDisplay
import tkinter
from tkinter import Tk
from tower import LogisticState


class GUIDisplay(TowerDisplay):
    window: Tk

    def __init__(self) -> None:
        Thread(target=self.window_loop).start()

    def window_loop(self):
        self.window = Tk()
        self.window.title("Bicycle Tower")
        self.window.geometry("500x500")

        self.window.mainloop()

    def clear_window(self):
        for widget in self.window.winfo_children():
            widget.destroy()

    # TODO: kinda scuffed but should work
    def generate_content(self, tower):
        for (x, level) in enumerate(tower.boxes):
            for (y, box) in enumerate(level):
                label = tkinter.Label(self.window, text="Box")
                label.config(font=("Courier", 20))

                match box.logistic_state:
                    case LogisticState.STORED:
                        label.config(bg="green")
                    case LogisticState.RETRIEVED:
                        label.config(bg="red")
                    case LogisticState.IN_TANSIT:
                        label.config(bg="yellow")

                label.grid(column=x, row=y)

    def update(self, tower):
        self.clear_window()
        self.generate_content(tower)
        self.window.title("Amogus")

    def initialize(self, tower):
        self.update(tower)
