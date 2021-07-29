import React, { useState } from "react";
import {Animal} from "./Animal";
import {DeadAnimal} from "./DeadAnimal";
import type { GameState } from "../gameState";
import { InfoBox } from "./InfoBox";
import "./Play.css";

type PlayProps = {
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Play({ gameState, setGameState}: PlayProps) {
    const [ infoState, setInfoState ] = useState<Animal | undefined>(undefined);

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

    async function resetGame() {
        try{
            const response = await fetch('/zoo/reset_game');

            if(response.ok) {
                const updatedGameState: GameState = await response.json();
                setGameState(updatedGameState);
            } else {
                console.error(response.statusText);
            }
        } catch (error) {
            console.error(error.toString());
        }
    }

    return (
        <div className="container">
            <div className="playContainer">
                <div>
                <p> Current player: {gameState.player} </p>
                <p> Level: {gameState.level} </p>
                <p>  Progress: {progress} % </p>
                </div>
                <div className="animalClass">
                    {
                    gameState.animals.map((animal) => (
                        <Animal id={getAnimalIndex()} gameState={gameState} setGameState={setGameState} setInfoState={setInfoState} key={animal.id}/>
                    ))}
                </div>
                <div className="graveyard">
                    {
                    gameState.dead_animals.map((deadAnimal) => (
                        <DeadAnimal id={getDeadAnimalIndex()} gameState={gameState} key={deadAnimal.id}/>
                    ))}
                </div>
                <button className="saveButton" onClick={() => saveAnimal()}> Save </button>
                <button className="resetButton" onClick={() => resetGame()}> Reset </button>
            </div>
            <InfoBox className="infoBox" infoState={infoState} />
        </div>
    )
}