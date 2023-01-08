from abc import ABC, abstractmethod

from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from tower import Tower


class TowerDisplay(ABC):
    @abstractmethod
    def __init__(self) -> None:
        pass

    @abstractmethod
    def update(self, tower: Tower):
        pass

    @abstractmethod
    def initialize(self, tower: Tower):
        pass


class LEDDisplay(TowerDisplay):
    pass
