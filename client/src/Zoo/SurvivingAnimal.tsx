import React, {useState} from "react";
import type { Animal } from "../gameState";
import type { GameState } from "../gameState";
import './SurvivingAnimal.css';

type SurvivingAnimalProps = {
    id: number;
    gameState: GameState;
}

export function SurvivingAnimal({id, gameState}: SurvivingAnimalProps) {
    let animal: Animal = gameState.animals[id];

    let animalImage = "images/" + animal.species + ".png";

    let background = "lime";

    return (
        <div className="survivor" style={{backgroundColor: background}}>
            <img src={animalImage} />
            <div> {animal.name} </div>
        </div>
    )
}