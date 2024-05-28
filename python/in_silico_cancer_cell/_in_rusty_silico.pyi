import enum

class A549CancerCell:
    @staticmethod
    def new() -> A549CancerCell:
        pass

    def evaluate(self, protocol: PatchClampProtocol, phase: CellPhase) -> float:
        pass

class PatchClampProtocol(enum.Enum):
    Activation = enum.auto()
    Deactivation = enum.auto()
    Ramp = enum.auto()

class CellPhase(enum.Enum):
    G0 = enum.auto()
    G1 = enum.auto()

class ChannelCountsProblem:
    @staticmethod
    def new(data: PatchClampData) -> ChannelCountsProblem:
        pass

    def precompute_single_channel_currents(self):
        pass

    def get_current_basis(self) -> list[list[float]]:
        pass

class InSilicoMethod(enum.Enum):
    Projection = enum.auto()
    ParticleSwarm = enum.auto()
    SteepestDescent = enum.auto()
    LBFGS = enum.auto()

class PatchClampData:
    @staticmethod
    def pyload(protocol: PatchClampProtocol, phase: CellPhase) -> PatchClampData:
        pass

def find_best_fit_for(data: PatchClampData, using: InSilicoMethod) -> list[float]:
    pass

def setup_logging():
    pass
