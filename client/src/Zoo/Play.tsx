import React from "react";
import {Animal} from "./Animal";
import type { GameState } from "../gameState";

type PlayProps = {
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Play({ gameState, setGameState}: PlayProps) {
    async function saveAnimal(){
        try {
            const response = await fetch('/zoo/save_animal', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(gameState)
            });
        } catch (error) {
            console.error(error.toString());
        }
    }

    return (
        <div>
            <Animal gameState={gameState} setGameState={setGameState}/>
            <button className="saveButton" onClick={() => saveAnimal()}> Save </button>
        </div>
    )
}