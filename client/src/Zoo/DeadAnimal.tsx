import React, {useState} from "react";
import type { Animal } from "../gameState";
import type { GameState } from "../gameState";
import "./DeadAnimal.css";

type DeadAnimalProps = {
    id: number;
    gameState: GameState;
}

export function DeadAnimal({id, gameState}: DeadAnimalProps) {
    let animal: Animal = gameState.dead_animals[id];

    let animalImage = "images/" + animal.species + ".png";

    let background = "gray";

    return (
        <div className="deadAnimal" style={{backgroundColor: background}}>
            <img src={animalImage} />
            <div> {animal.name} </div>
        </div>
    )
}