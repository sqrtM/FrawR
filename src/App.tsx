import { useEffect, useState } from "react";
import init, { World, } from "rustlib";
import TileMap from "./components/TileMap";

export default function App(): React.JSX.Element {

  const [world, setWorld] = useState<World>();

  let height = 100;
  let width = 100;

  useEffect(() => {
    init().then(() => {
      const w = new World;
      w.build_map(height, width, 53983);
      w.set_entities();
      setWorld(w);
      w.sort_entities();
      console.log(w.get_tiles())
      console.log(w.get_entities());
    })
  }, [])


  return world ? (
    <>
      <div style={{ backgroundColor: "red" }}>
        henlo testing testing
      </div>
      <div key="tilemap" id="tilemap">
        <TileMap tileArray={world.get_tiles()} width={width} />
      </div>
    </>
  ) : (
    <div style={{ backgroundColor: "blue" }}>
      LOADING
    </div>
  )
}
