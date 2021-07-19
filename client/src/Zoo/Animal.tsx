import React from "react";
import { Animal } from "../gameState";
import { GameState } from "../gameState";

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