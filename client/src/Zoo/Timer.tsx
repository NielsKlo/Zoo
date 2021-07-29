import React, { useState,  useEffect } from "react";
import type { GameState } from "../gameState";

type TimerProps = {
    difficulty: number;
    setGameState(newGameState: GameState): void;
}

export function Timer({difficulty, setGameState}: TimerProps) {
    let tickRate = 2000 / difficulty;

    useEffect(() => {
        var timer = setInterval(tickForward, tickRate);
        return () => {
            clearInterval(timer);
        }
    }, []);

    async function tickForward() {
        try{
            const response = await fetch('/zoo/tick_forward');

            if(response.ok) {
                const updatedGameState: GameState = await response.json();
                setGameState(updatedGameState);
            } else {
                console.error(response.statusText);
            }

        } catch (error){
            console.error(error.toString());
        }
    }

    return <div />
}