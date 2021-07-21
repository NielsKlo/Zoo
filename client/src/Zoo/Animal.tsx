import React from "react";
import type { Animal } from "../gameState";
import type { GameState } from "../gameState";

type AnimalProps = {
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Animal({gameState, setGameState}: AnimalProps) {

    return (
        <div>
            Species: {gameState.animal.species}
            <br></br>
            Age: {gameState.animal.age}
            <br></br>
            Hunger: {gameState.animal.hunger}
        </div>
    )
}