import React, { useEffect, useState } from "react";
import init, { Entity, Point, Tile, World, } from '../rustlib/pkg';
import TileMap from "./components/gameboard/TileMap";
import SideBar from "./components/sidebar/SideBar";

const width = 50;
const height = 35;

export default function App(): React.JSX.Element {

  const [world, setWorld] = useState<World>();
  const [tiles, setTiles] = useState<[Point, Tile][]>();
  const [player, setPlayer] = useState<Entity>()
  const [entities, setEntities] = useState<Entity[]>();

  useEffect(() => {
    init().then(() => {
      const w = new World(width, height);
      setWorld(w);
      setTiles(w.get_tiles());
      const c: { entities: Entity[], player: Entity } = w.get_all_creatures()
      setEntities(c.entities)
      setPlayer(c.player)
    })
  }, [])

  function handleKeyPresses(e: { key: string; }) {
    const direction = ["z", "s", "q", "d", " "].indexOf(e.key);
    if (world && direction > -1) {
      init().then(() => {
        const c: { entities: Entity[], player: Entity } = world.take_turn_and_return(direction);
        setEntities(c.entities)
        setPlayer(c.player)
      })
    }
  }

  return tiles && entities && player ? (
    <>
      <div id="game-screen">
        <div key="tilemap" id="tilemap" onKeyDown={handleKeyPresses} tabIndex={1}>
          <TileMap tiles={tiles} entities={entities} player={player} width={width} />
        </div>
        <SideBar playerInfo={player} />
      </div>
    </>
  ) : (
    <div style={{ backgroundColor: "blue" }} onKeyDown={handleKeyPresses}>
      LOADING
    </div>
  )
}
