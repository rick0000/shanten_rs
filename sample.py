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



tehais = ["4m","5m","6m","7m","8m","9m","1p"]
furos = [
    {"type":"chi","taken":"2m","consumed":["1m","3m"]},
    # {"type":"pon","pais":["1p","1p","1p"]},
    {"type":"ankan","consumed":["1s","1s","1s","1s"]},
]
taken = "1p"
result = shanten.get_hora(tehais,furos,taken)
print(result)

