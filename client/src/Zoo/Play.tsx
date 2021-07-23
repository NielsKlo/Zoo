import React, { useState, useEffect } from "react";
import {Animal} from "./Animal";
import type { GameState } from "../gameState";

type PlayProps = {
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Play({ gameState, setGameState}: PlayProps) {

    useEffect(() => {
        var handle = setInterval(tickForward, 5000);
        return () => {
            clearInterval(handle);
        }
    });

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

    async function saveAnimal(){
        try {
            const response = await fetch('/zoo/save_animal');

            if(response.ok){
                console.log('Saved the animal!');
            } else {
                console.error(response.statusText);
            }
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