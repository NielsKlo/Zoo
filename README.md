# nklootwijk_Zoo

The core idea of this project was for a player to be able to feed and build up a collection of zoo animals.
Players can feed animals and will get new animals faster if they keep their animals properly fed.
There are three difficulty options to choose from. The game can be saved and continued from that save.

#Software needed:

- Node.js

- Rust language

- Cargo

- MongoDB

**Installation:**

- Run "cargo build" in the nklootwijk_zoo directory.

- Run "npm install" in the client directory.

This will likely take a few minutes.

**Setting up the database:**

- Download the on-premise, community MongoDB server and install it. Use the default port (27017). 
  
- run "cargo run" in the database directory. (WIP)


#Commands to run the program:

- "cargo run" in the nklootwijk_zoo directory.

- "npm run start" in the client directory.

Your browser should automatically open to the correct page. If it doesn't, go to localhost:3000 manually.

#Commands to run the tests:

- "cargo test" in the domain or database directory.