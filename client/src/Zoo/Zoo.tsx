import React, { useState } from "react";
import { StartGame } from "./StartGame";
import { Play } from "./Play";
import { Timer } from "./Timer";
import { Ending } from "./Ending";
import type { GameState } from "../gameState";

export function Zoo() {
    const [ gameState, setGameState ] = useState<GameState | undefined>(undefined);

    console.log("redrawing Zoo.txs");

    if (!gameState) {
        return <StartGame setGameState={setGameState} />
    }

    if ((gameState.animals.length + gameState.dead_animals.length) === 26){
        return <div>
            <Ending gameState={gameState} setGameState={setGameState} />
            </div>
    }

    return <div>
            <Play gameState={gameState} setGameState={setGameState} />
            <Timer setGameState={setGameState} />
            </div>
}