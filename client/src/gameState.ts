export interface GameState {
    player: string;
    score: number;
    animals: Animal[];
}

export interface Animal {
    id: number;
    species: string;
    hunger: number;
}