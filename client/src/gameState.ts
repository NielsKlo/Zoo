export interface GameState {
    player: string;
    level: number;
    progress: number;
    difficulty: number;
    animals: Animal[];
    dead_animals: Animal[];
}

export interface Animal {
    id: number;
    species: string;
    name: string;
    hunger: number;
}