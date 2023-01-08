from __future__ import annotations
from enum import Enum
from typing import List, Tuple

from tower_display import TowerDisplay
from box import Box
from logistic_state import LogisticState


class Tower:
    boxes: List[List[Box]] = []
    display: TowerDisplay

    def __init__(self, levels: int, boxes_per_level: int, display: TowerDisplay) -> None:
        self.__initiate_boxes(levels, boxes_per_level)
        self.display = display
        self.display.initialize(self)

    def __initiate_boxes(self, levels: int, boxes_per_level: int):
        boxes: List[List[Box]] = []
        for i in range(levels):
            level = []
            for j in range(boxes_per_level):
                level.append(Box())
            boxes.append(level)

        self.boxes = boxes
        self.display.update(self)

    def get_box(self, box_id: Tuple[int, int]) -> Box:
        return self.boxes[box_id[0]][box_id[1]]

    def get_empty_box(self) -> Box:
        for level in self.boxes:
            for box in level:
                if box.is_empty and box.logistic_state == LogisticState.STORED:
                    return box

        raise Exception("No empty box available")

    def retrieve_box(self, box: Box):
        # TODO: Make program not crash if box is already retrieved or in transit
        if box.logistic_state != LogisticState.STORED:
            raise Exception("Box is not stored")

        # TODO: Simulate retrieving of box
        # box.logistic_state = LogisticState.IN_TANSIT
        # wait(5)
        box.logistic_state = LogisticState.RETRIEVED
        self.display.update(self)

    def store_box(self, box: Box) -> None:
        # TODO: Make program not crash if box is already stored
        if box.logistic_state != LogisticState.RETRIEVED:
            raise Exception("Box is not retrieved")

        box.logistic_state = LogisticState.STORED
        self.display.update(self)

    def rent_box(self, box: Box, user_id: str) -> None:
        if not box.is_empty:
            raise Exception("Box is not empty")

        print("Its rentin' time")

        box.rented_by = user_id
        box.is_empty = False
        self.display.update(self)

    def unrent_box(self, box: Box) -> None:
        print("Its unrentin' time")

        box.rented_by = None
        box.is_empty = True
        self.display.update(self)
