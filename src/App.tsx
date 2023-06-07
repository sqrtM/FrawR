import { useEffect, useState } from "react";
import init, { Entity, Tile, World, } from "rustlib";
import TileMap from "./components/TileMap";

let height = 20;
let width = 20;

import { createContext } from 'react';
export const PlayerContext = createContext<Entity>(new Entity);

export default function App(): React.JSX.Element {

  const [world, setWorld] = useState<World>();
  const [tiles, setTiles] = useState<Tile[]>();
  const [player, setPlayer] = useState<Entity>()
  const [entities, setEntities] = useState<Entity[]>();

  useEffect(() => {
    init().then(() => {
      const w = new World(width, height);
      w.build_map(539483);
      w.set_entities();
      setWorld(w);
      setTiles(w.get_tiles() as Tile[]);
      setEntities(w.get_entities() as Entity[])
      setPlayer(w.get_player() as Entity)
    })
  }, [])

  function handleKeyPresses(e: any) {
    const direction = ["z", "s", "q", "d"].indexOf(e.key);
    if (world && direction > -1) {
      init().then(() => {
        world.take_turn(direction);
        setEntities(world.get_entities())
        setPlayer(world.get_player())
      })
    }
  }

  return world && tiles && entities && player ? (
    <>
      <div style={{ backgroundColor: "red" }}>
        henlo testing testing
      </div>
      <div key="tilemap" id="tilemap" onKeyDown={handleKeyPresses} tabIndex={1}>
        <PlayerContext.Provider value={player}>
          <TileMap tiles={tiles} entities={entities} width={width} />
        </PlayerContext.Provider>
      </div>
    </>
  ) : (
    <div style={{ backgroundColor: "blue" }}>
      LOADING
    </div>
  )
}
