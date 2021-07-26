import React, {useState} from "react";
import type { Animal } from "../gameState";
import type { GameState } from "../gameState";
import './Animal.css';

type AnimalProps = {
    id: number;
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Animal({id, gameState, setGameState}: AnimalProps) {

    let animalImage = "images/" + gameState.animals[id].species + ".png";

    async function feedAnimal(){
        let stringId = "" + id;
        try{
            const response = await fetch('/zoo/feed_animal', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'text/plain'
                },
                body: stringId
            });

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

    return (
        <div className="animal">
            <img src={animalImage} />
            <div className="species">
            Species: {gameState.animals[id].species}
            </div>
            <div className="hunger">
            Hunger: {gameState.animals[id].hunger}
            </div>
            <button className="feedButton" onClick={() => feedAnimal()}> Feed </button>
        </div>
    )
}