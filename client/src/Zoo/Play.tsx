import React from "react";
import {Animal} from "./Animal";
//import type { GameState } from "../gameState";

type PlayProps = {
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Play({ gameState, setGameState}: PlayProps) {
    return (
        <div>
            <Animal gameState={gameState} setGameState={setGameState}/>
        </div>
    )
}