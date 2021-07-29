import React, { useState } from "react";
import type { Animal } from "../gameState";

type InfoBoxProps = {
    infoState: Animal;
}

export function InfoBox({ infoState }: InfoBoxProps) {
    if (infoState === undefined) {
        return (<div/>)
    }

    let animalImage = "images/" + infoState.species + ".png";

    return (
        <div className="info">
            <img src={animalImage} />
            <p> Species: {infoState.species} </p>
            <p> Zoo ID: {infoState.id} </p>
            <p> Name: {infoState.name} </p>
        </div>
    )
}