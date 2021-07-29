import React, { useState } from "react";
import {SurvivingAnimal} from "./SurvivingAnimal";
import {DeadAnimal} from "./DeadAnimal";
import type { GameState } from "../gameState";
import "./Ending.css";

type EndingProps = {
    gameState: GameState;
}

export function Ending({ gameState }: EndingProps) {
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

    return (
        <div>
            <p className="endingMessage"> Game Finished: reached level {gameState.level} </p>
            <p className="survivorHeader"> The Survivors </p>
            <div className="survivors">
                {
                gameState.animals.map((animal) => (
                    <SurvivingAnimal id={getAnimalIndex()} gameState={gameState} key={animal.id} />
                ))}
            </div>
            <p className="fallenHeader"> The Fallen </p>
            <div className="fallen">
                {gameState.dead_animals.map((deadAnimal) => (
                    <DeadAnimal id={getDeadAnimalIndex()} gameState={gameState} key={deadAnimal.id}/>
                ))}
            </div>
        </div>
    )
}