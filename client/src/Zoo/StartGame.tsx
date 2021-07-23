import React, { useState } from "react";
import type { GameState } from "../gameState";

type StartGameProps = {
    setGameState(newGameState: GameState): void;
}

export function StartGame({ setGameState}: StartGameProps) {
    const [playerName, setPlayerName] = useState("");

    async function tryStartGame(e: React.FormEvent) {
        e.preventDefault();
        try {
            const response = await fetch('/zoo/get_animals', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'text/plain'
                },
                body: playerName
            });
            if(response.ok){
                const message = await response.json();
                await setGameState(message);
            } else {
                console.error(response.statusText);
            }
        } catch(e) {
            console.log(e.toString());
        }
    }
    return (
        <form onSubmit={(e) => tryStartGame(e)}>
            <input value={playerName}
                placeholder="Niels"
                onChange={(e) => setPlayerName(e.target.value)}
            />
            <button className="startGameButton" type="submit">
                Play!
            </button>
        </form>
    )
}