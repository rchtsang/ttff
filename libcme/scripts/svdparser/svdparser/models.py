import re
from collections import namedtuple
from enum import Enum

from utils import (
    camel_to_snake,
    snake_to_camel,
)

BITRANGE_PTRN = re.compile(r"\[(?P<msb>\d+):(?P<lsb>\d+)\]")

Range = namedtuple("Range", ("min", "max"))

class AccessType(Enum):
    READ_ONLY   = "read-only"
    WRITE_ONLY  = "write-only"
    READ_WRITE  = "read-write"
    WRITEONCE   = "writeonce"
    READ_WRITEONCE = "read-writeonce"

    def as_bits(string):
        match string:
            case AccessType.READ_ONLY.value:
                return 0b100
            case AccessType.WRITE_ONLY.value:
                return 0b010
            case AccessType.READ_WRITE.value:
                return 0b110
            case AccessType.WRITEONCE.value:
                return 0b010
            case AccessType.READ_WRITEONCE.value:
                return 0b110
            case _:
                raise ValueError(f"invalid access type: {string}")

class ProtectionType(Enum):
    S = "s" # secure permission required for access
    N = "n" # non-secure or secure permission required for access
    P = "p" # privileged permission required for access

class DataType(Enum):
    UINT8_T      = "uint8_t"
    UINT16_T     = "uint16_t"
    UINT32_T     = "uint32_t"
    UINT64_T     = "uint64_t"
    INT8_T       = "int8_t"
    INT16_T      = "int16_t"
    INT32_T      = "int32_t"
    INT64_T      = "int64_t"
    P_UINT8_T    = "uint8_t *"
    P_UINT16_T   = "uint16_t *"
    P_UINT32_T   = "uint32_t *"
    P_UINT64_T   = "uint64_t *"
    P_INT8_T     = "int8_t *"
    P_INT16_T    = "int16_t *"
    P_INT32_T    = "int32_t *"
    P_INT64_T    = "int64_t *"

class ModifiedWriteValuesType(Enum):
    # write data bits of one shall clear (set to zero) the corresponding bit 
    # in the register.
    ONE_TO_CLEAR   = "oneToClear"
    # write data bits of one shall set (set to one) the corresponding bit in 
    # the register.
    ONE_TO_SET     = "oneToSet"
    # write data bits of one shall toggle (invert) the corresponding bit in 
    # the register.
    ONE_TO_TOGGLE  = "oneToToggle"
    # write data bits of zero shall clear (set to zero) the corresponding bit 
    # in the register.
    ZERO_TO_CLEAR  = "zeroToClear"
    # write data bits of zero shall set (set to one) the corresponding bit 
    # in the register.
    ZERO_TO_SET    = "zeroToSet"
    # write data bits of zero shall toggle (invert) the corresponding bit 
    # in the register.
    ZERO_TO_TOGGLE = "zeroToToggle"
    # after a write operation all bits in the field are cleared (set to zero).
    CLEAR          = "clear"
    # after a write operation all bits in the field are set (set to one).
    SET            = "set"
    # after a write operation all bit in the field may be modified (default).
    MODIFY         = "modify"

class ReadActionType(Enum):
    # The register is cleared (set to zero) following a read operation.
    CLEAR           = "clear"
    # The register is set (set to ones) following a read operation.
    SET             = "set"
    # The register is modified in some way after a read operation.
    MODIFY          = "modify"
    # One or more dependent resources other than the current register are 
    # immediately affected by a read operation (it is recommended that the
    # register description specifies these dependencies).
    MODIFY_EXTERNAL = "modifyExternal"


class WriteConstraint:
    __slots__ = ("write_as_read", "use_enumerated_values", "range")
    
    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            self[k] = v

    def __setitem__(self, fieldname, value):
        # dynamic type checking
        match fieldname:
            case "write_as_read":
                assert isinstance(value, bool)
            case "use_enumerated_values":
                assert isinstance(value, bool)
            case "range":
                assert isinstance(value, Range)
            case _ :
                raise ValueError(f"invalid field name: {fieldname}")
        setattr(self, fieldname, value)

    def __getitem__(self, fieldnmae):
        getattr(self, fieldname)


class AddressBlock:
    __slots__ = ("offset", "size", "usage", "protection")
    
    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            self[k] = v

    def __setitem__(self, fieldname, value):
        # dynamic type checking
        match fieldname:
            case "offset":
                assert isinstance(value, int)
            case "size":
                assert isinstance(value, int)
            case "usage":
                assert value in ["registers", "buffer", "reserved"]
            case "protection":
                assert value in ["s", "n", "p", None]
            case _ :
                raise KeyError(f"{fieldname} invalid. expected {self.__slots__}")
        setattr(self, fieldname, value)

    def __getitem__(self, fieldname):
        getattr(self, fieldname)


class Interrupt:
    __slots__ = ("name", "description", "value")

    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            self[k] = v

    def __setitem__(self, fieldname, value):
        # dynamic type checking
        match fieldname:
            case "name" | "description":
                assert isinstance(value, str)
            case "value":
                assert isinstance(value, int)
            case _ :
                raise KeyError(f"{fieldname} invalid. expected {self.__slots__}")
        setattr(self, fieldname, value)

    def __getitem__(self, fieldname):
        getattr(self, fieldname)
