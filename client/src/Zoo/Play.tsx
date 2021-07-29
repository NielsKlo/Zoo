import React, { useState } from "react";
import {Animal} from "./Animal";
import {DeadAnimal} from "./DeadAnimal";
import type { GameState } from "../gameState";
import "./Play.css";

type PlayProps = {
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Play({ gameState, setGameState}: PlayProps) {
    const progress: number = Math.trunc(gameState.progress / 100);
    let animalIndex = 0;
    let deadAnimalIndex = 0;

    function getAnimalIndex() {

        let oldIndex = animalIndex;
        animalIndex += 1;
        return oldIndex;
    }

    function getDeadAnimalIndex() {
        let oldIndex = deadAnimalIndex;
        deadAnimalIndex += 1;
        return oldIndex;
    }

    async function saveAnimal(){
        try {
            const response = await fetch('/zoo/save_animals');

            if(response.ok){
                console.log('Saved the animals!');
            } else {
                console.error(response.statusText);
            }
        } catch (error) {
            console.error(error.toString());
        }
    }

    return (
        <div>
            <p> Current player: {gameState.player} </p>
            <p> Level: {gameState.level} </p>
            <p>  Progress: {progress} % </p>
            <div className="animalClass">
                {
                gameState.animals.map((animal) => (
                    <Animal id={getAnimalIndex()} gameState={gameState} setGameState={setGameState} key={animal.id}/>
                ))}
            </div>
            <button className="saveButton" onClick={() => saveAnimal()}> Save </button>
            <div className="graveyard">
                {
                gameState.dead_animals.map((deadAnimal) => (
                    <DeadAnimal id={getDeadAnimalIndex()} gameState={gameState} key={deadAnimal.id}/>
                ))}
            </div>
        </div>
    )
}