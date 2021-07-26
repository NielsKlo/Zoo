import React, { useState,  useEffect } from "react";
import type { GameState } from "../gameState";

type TimerProps = {
    setGameState(newGameState: GameState): void;
}

export function Timer({setGameState}: TimerProps) {

    useEffect(() => {
        var timer = setInterval(tickForward, 2000);
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