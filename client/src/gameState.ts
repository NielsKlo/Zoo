export interface GameState {
    player: string;
    animals: Animal[];
}

export interface Animal {
    id: number;
    species: string;
    hunger: number;
}