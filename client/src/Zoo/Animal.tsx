import React, {useState} from "react";
import type { Animal } from "../gameState";
import type { GameState } from "../gameState";
import './Animal.css';

type AnimalProps = {
    id: number;
    gameState: GameState;
    setGameState(newGameState: GameState): void;
    setInfoState(newInfoState: Animal): void;
}

export function Animal({id, gameState, setGameState, setInfoState}: AnimalProps) {
    let animal: Animal = gameState.animals[id];

    let animalImage = "images/" + animal.species + ".png";

    let background = getCurrentBackgroundColor();

    function getCurrentBackgroundColor(){
        if (animal.hunger >= 96){
            return "green";
        } else if (animal.hunger >= 31) {
            return "lime";
        } else if (animal.hunger >= 11) {
            return "orange";
        } else if (animal.hunger >= 1) {
            return "red";
        } else {
            return "gray"
        }
    }

    function replaceInfoState(e) {
        e.preventDefault();
        setInfoState(animal);
    }

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

    async function bulkFeedAnimal(){
        let stringId = "" + id;
        try{
            const response = await fetch('/zoo/bulk_feed_animal', {
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
        <div className="animal"
                style={{backgroundColor: background}}
                onClick={replaceInfoState}>
            <img src={animalImage}/>
            <div> Name: {animal.name} </div>
            <div> Hunger: {animal.hunger} </div>
            <button className="feedButton" onClick={() => feedAnimal()}> Feed </button>
            <button className="bulkFeedButton" onClick={() => bulkFeedAnimal()}> Bulk Feed </button>
        </div>
    )
}