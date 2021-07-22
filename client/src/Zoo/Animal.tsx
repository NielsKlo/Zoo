import React, {useState} from "react";
import type { Animal } from "../gameState";
import type { GameState } from "../gameState";

type AnimalProps = {
    gameState: GameState;
    setGameState(newGameState: GameState): void;
}

export function Animal({gameState, setGameState}: AnimalProps) {
    const [animalState, setAnimalState] = useState<GameState>(gameState);

    async function feedAnimal(){
        try{
            const response = await fetch('/zoo/feed_animal', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-type': 'application/json'
                },
                body: JSON.stringify(gameState)
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
        <div>
            Species: {gameState.animal.species}
            <br></br>
            Age: {gameState.animal.age}
            <br></br>
            Hunger: {gameState.animal.hunger}
            <button className="feedButton" onClick={() => feedAnimal()}> Feed </button>
        </div>
    )
}