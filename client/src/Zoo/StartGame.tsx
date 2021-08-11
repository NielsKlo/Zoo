import React, { useState } from "react";
import type { GameState } from "../gameState";

type StartGameProps = {
    setGameState(newGameState: GameState): void;
}

export function StartGame({ setGameState}: StartGameProps) {
    const [playerName, setPlayerName] = useState("");
    const [difficulty, setDifficulty] = useState("easy");

    function convertDifficulty() {
        if (difficulty === "easy"){
            return 1;
        } else if(difficulty === "medium") {
            return 5;
        } else {
            return 10;
        }
    }

    async function tryStartGame(e: React.FormEvent) {
        e.preventDefault();
        let convertedDifficulty = convertDifficulty();
        try {
            const response = await fetch('/zoo/get_animals', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    player: playerName,
                    difficulty: convertedDifficulty
                })
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
                placeholder="Player"
                onChange={(e) => setPlayerName(e.target.value)}
            />
            <select value={difficulty}
                onChange={(e) => setDifficulty(e.target.value)}>
                <option value="easy"> Easy </option>
                <option value="medium"> Medium </option>
                <option value="hard"> Hard </option>
            </select>
            <button className="startGameButton" type="submit">
                Play!
            </button>
        </form>
    )
}