import mjaigym
from mjaigym.mjson import Mjson
from mjaigym.board import ArchiveBoard
from mjaigym.board.function.pai import Pai

import shanten
import datetime
from pathlib import Path

mjson_paths = Path("/data/").glob("**/*.mjson")
show = False

for i,mjson_path in enumerate(mjson_paths):
    
    print(mjson_path)
    print(datetime.datetime.now(), f"{i}")
    mjson = Mjson.load(mjson_path)
    
    for kyoku in mjson.game.kyokus:
        all_furos = [[],[],[],[]]

        oya_player = kyoku.kyoku_mjsons[0]["oya"]
        first_turns = [True,True,True,True]
        reachs = [False,False,False,False]
        ippatsus = [False,False,False,False]
        double_reachs = [False,False,False,False]
        rinshans = [False,False,False,False]
        doramarkers = []
        tsumo_count = 0
        bakaze = kyoku.kyoku_mjsons[0]["bakaze"]
        winds = ["E","S","W","N"]
        jikazes = [
            winds[(0-oya_player+4)%4],
            winds[(1-oya_player+4)%4],
            winds[(2-oya_player+4)%4],
            winds[(3-oya_player+4)%4],
        ]
        chankan = False
        previous_actions = [None,None,None,None]
        previous_action = None

        for line in kyoku.kyoku_mjsons:
            # state update
            if line["type"] == "kakan":
                chankan = True

            if line["type"] in ["ankan", "kakan", "daiminkan"]:
                rinshans[line["actor"]] = True

            if line["type"] in ["dora", "start_kyoku"]:
                # print("doraline", line)
                doramarkers.append(line["dora_marker"])

            if line["type"] == "tsumo":
                tsumo_count += 1
                chankan = False

            if line["type"] == "dahai":
                first_turns[line["actor"]] = False
                ippatsus[line["actor"]] = False
                rinshans[line["actor"]] = False

            if line["type"] in ["pon", "chi", "ankan", "daiminkan"]:
                first_turns = [False,False,False,False]
                ippatsus = [False,False,False,False]

            if line["type"] == "reach_accepted":
                ippatsus[line["actor"]] = True

            if line["type"] == "reach":
                reachs[line["actor"]] = True
                if first_turns[line["actor"]]:
                    double_reachs[line["actor"]] = True

            # furo update
            if line["type"] in ["chi", "pon", "daiminkan"]:
                all_furos[line["actor"]].append({
                        "type":line["type"],
                        "taken":line["pai"],
                        "consumed":line["consumed"],
                    })
            elif line["type"] == "kakan":
                for f in all_furos[line["actor"]]:
                    if f["type"] == "pon":
                        pon_taken = Pai.from_str(f["taken"])
                        kakan_pai = Pai.from_str(line["pai"])
                        if pon_taken.is_same_symbol(kakan_pai):
                            f["type"] = "kakan"
                            f["consumed"].append(f["taken"])
                            f["taken"] = line["pai"]
            elif line["type"] == "ankan":
                all_furos[line["actor"]].append({
                        "type":line["type"],
                        "consumed":line["consumed"],
                    })

            # check hora
            if line["type"] == "hora":
                tehai = line["hora_tehais"]
                taken = line["pai"]
                furos = all_furos[line["actor"]]
                # print("--------------------------")
                # print(line)
                # print(tehai)
                # print(furos)
                # print(taken)
                # print(doramarkers)

                hora_type = "tsumo" if line["actor"] == line["target"] else "ron"
                actor = line["actor"]
                
                haitei = tsumo_count == 70
                uradoramarkers = line["uradora_markers"]
                first_turn = first_turns[actor]
                reache = reachs[actor]
                double_reach = double_reachs[actor]
                
                ippatsu = ippatsus[actor]
                rinshan = rinshans[actor]
                jikaze = jikazes[actor]
                oya = oya_player == actor

                doras = [Pai.from_str(p).succ.str for p in doramarkers]
                uradoras = [Pai.from_str(p).succ.str for p in uradoramarkers]

                result = shanten.get_hora(
                    tehai, 
                    furos, 
                    taken,
                    oya,
                    hora_type,
                    first_turn,
                    doras,
                    uradoras,
                    reache,
                    double_reach,
                    ippatsu,
                    rinshan,
                    chankan,
                    haitei,
                    bakaze,
                    jikaze,
                    show,
                )
                
                try:
                    assert result[1] > 0
                    if line["fan"] >= 100:
                        assert result[1] >= 100
                    else:
                        assert result[1] == line["fan"]
                    
                    assert result[0] == line["fu"]
                except:
                    print("line",line)
                    print(result)
                    print(tehai)
                    print(taken)
                    print(furos)
                    print(result[1], "actual", line["fan"])
                    print(result[0], "actual", line["fu"])
                    print(line["fu"])
                    print("chankan", chankan)
                    all_pais = tehai
                    all_pais.append(taken)
                    for furo in furos:
                        all_pais += furo["consumed"]
                        if "taken" in furo:
                            all_pais.append(furo["taken"])

                    akadora_count = len([p for p in all_pais if p.endswith("r")])
                    print("akadora_count", akadora_count)
                    assert result[1] == line["fan"] + akadora_count


            if previous_action and previous_action["type"] == "kakan":
                ippatsus = [False,False,False,False]        

            if "actor" in line:
                previous_actions[line["actor"]] = line
                previous_action = line
