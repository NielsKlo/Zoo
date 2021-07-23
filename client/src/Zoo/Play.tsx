import React, { useState, useEffect } from "react";
import {Animal} from "./Animal";
import type { GameState } from "../gameState";
import "./Play.css";

type PlayProps = {
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Play({ gameState, setGameState}: PlayProps) {

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
            <div className="animalClass">
                {
                gameState.animals.map((animal) => (
                    <Animal id={animal.id} gameState={gameState} setGameState={setGameState} key={animal.id}/>
                ))}
            </div>
            <button className="saveButton" onClick={() => saveAnimal()}> Save </button>
        </div>
    )
}