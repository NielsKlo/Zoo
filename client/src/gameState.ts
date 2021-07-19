export interface GameState {
    animal: Animal;
}

export interface Animal {
    species: string;
    age: number;
    hunger: number;
}