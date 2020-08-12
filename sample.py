import shanten
import datetime
# warm up
result = shanten.get_shanten([
        4,4,4,2,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0
    ])


print(datetime.datetime.now())

for i in range(10000):
    result = shanten.get_shanten([
            4,4,4,2,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0
        ])
    a,b,c = shanten.get_shanten_all([
            4,4,4,2,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0
        ])

    
# for i in range(1000000):
#     result = shanten.get_shanten([
#             1,1,1,2,0,0,0,0,0,
#             0,0,0,3,3,3,0,0,0,
#             0,0,0,0,0,0,0,0,0,
#             0,0,0,0,0,0,0
#         ])


print(datetime.datetime.now())
print(result)
print(a,b,c)