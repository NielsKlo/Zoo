import React, { useState } from "react";
import { StartGame } from "./StartGame";
import { Play } from "./Play";
import type { GameState } from "../gameState";

export function Zoo() {
    const [ gameState, setGameState ] = useState<GameState | undefined>(undefined);

    if (!gameState) {
        return <StartGame setGameState={setGameState} />
    }

    return <div>
            <Play gameState={gameState} setGameState={setGameState} />
            <Timer setGameState={setGameState} />
            </div>
}