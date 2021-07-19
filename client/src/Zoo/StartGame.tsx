import React, { useState } from "react";
import type { GameState } from "../gameState";

type StartGameProps = {
    setGameState(newGameState: GameState): void;
}

export function StartGame({ setGameState}: StartGameProps) {
    const [message, setMessage] = useState("");

    async function tryStartGame(e: React.MouseEvent) {
        try {
            const response = await fetch('/zoo/animal');
            if(response.ok){
                const message = await response.json();
                await console.log(message);
                await setGameState(message);
            } else {
                console.error(response.statusText);
            }
        } catch(e) {
            console.log(e.toString());
        }
    }
    return (
        <button className="startGameButton" onClick={(e) => tryStartGame(e)}>
            Start
        </button>
    )
}