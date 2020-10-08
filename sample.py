import shanten
import datetime

from memory_profiler import profile

def test_shanten():
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




# @profile
def test_hora():
    show = True
    # tehais = ["4m","5m","6m","7m","8m","9m","1p"]
    tehais = ["6p","7p","8p","9p","9p","6s","7s", "E","N","N"]
    furos = [
        {"type":"chi", "taken":"5s", "consumed":["4s", "6s"]},
        # {"type":"pon","taken":"S","consumed":["S","S"]},
    ]
    taken = "5s"

    oya = True
    hora_type = "tsumo"
    first_turn = False
    doras = ["1m"]
    uradoras = []
    reach = False
    double_reach = False
    ippatsu = False
    rinshan = False
    chankan = False
    haitei = False
    bakaze = "E"
    jikaze = "S"
    try:
        result = shanten.get_hora(
            tehais,
            furos,
            taken,
            oya,
            hora_type,
            first_turn,
            doras,
            uradoras,
            reach,
            double_reach,
            ippatsu,
            rinshan,
            chankan,
            haitei,
            bakaze,
            jikaze,
            show,
        )
    except:
        print("Exception occered!")
    print(result)
    yakus = result[1]
    for yaku in yakus:
        print(yaku)

def check_memleak():
    import time
    import collections
    import gc
    for _ in range(10):
        print("append")

        r = collections.deque()
        for i in range(10000000):
            a = shanten.check_memleak()
            # time.sleep(0.1)
            r.append(a)
        print("clear")
        r.clear()
        print("sleep")
        gc.collect()
        time.sleep(2)
        
        

test_hora()
# check_memleak()
