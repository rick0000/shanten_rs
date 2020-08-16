import shanten
import datetime

result = shanten.get_shanten([
        4,4,4,2,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0
    ],0)

print(result)


result = shanten.get_shanten([
        0,0,0,0,0,2,0,0,2,
        0,0,0,2,0,0,0,2,0,
        2,0,2,0,0,0,0,0,0,
        0,0,2,0,0,0,0
    ], 0)

print(result)