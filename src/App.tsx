import { useEffect, useState } from "react";
import init, { Entity, Tile, World, } from "rustlib";
import TileMap from "./components/TileMap";

let height = 200;
let width = 200;

export default function App(): React.JSX.Element {

  const [world, setWorld] = useState<World>();
  const [tiles, setTiles] = useState<Tile[]>();
  const [entities, setEntities] = useState<Entity[]>();

  const [clicks, setClicks] = useState<number>(0);

  useEffect(() => {
    init().then(() => {
      const w = new World;
      w.build_map(height, width, 539483);
      w.set_entities();
      setWorld(w);
      setTiles(w.get_tiles());
      setEntities(w.get_entities())
    })
  }, [])

  useEffect(() => {
    if (world) {
      init().then(() => {
        world.take_turn(1);
        setEntities(world.get_entities())
      })
    }
  }, [clicks])

  function handleKeyPresses() {
    console.log(clicks)
    setClicks(clicks + 1)
  }

  return world && tiles && entities ? (
    <>
      <div style={{ backgroundColor: "red" }}>
        henlo testing testing
      </div>
      <div key="tilemap" id="tilemap" onKeyDown={handleKeyPresses} tabIndex={-1}>
        <TileMap tiles={tiles} entities={entities} width={width} />
      </div>
    </>
  ) : (
    <div style={{ backgroundColor: "blue" }}>
      LOADING
    </div>
  )
}
