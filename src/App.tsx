import { useEffect, useState } from "react";
import init, { Entity, Point, Tile, World, } from "rustlib";
import TileMap from "./components/TileMap";

let height = 100;
let width = 100;

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
      let c: {entities: Entity[], player: Entity} = w.get_all_creatures()
      setEntities(c.entities)
      setPlayer(c.player)
    })
  }, [])

  function handleKeyPresses(e: { key: string; }) {
    const direction = ["z", "s", "q", "d", " "].indexOf(e.key);
    if (world && direction > -1) {
      init().then(() => {
        let c: {entities: Entity[], player: Entity} = world.take_turn_and_return(direction);
        setEntities(c.entities)
        setPlayer(c.player)
      })
    }
  }

  return tiles && entities && player ? (
    <>
      <div style={{ backgroundColor: "red" }}>
        henlo testing testing
      </div>
      <div key="tilemap" id="tilemap" onKeyDown={handleKeyPresses} tabIndex={1}>
          <TileMap tiles={tiles} entities={entities} player={player} width={width} />
      </div>
    </>
  ) : (
    <div style={{ backgroundColor: "blue" }} onKeyDown={handleKeyPresses}>
      LOADING
    </div>
  )
}
