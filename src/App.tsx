import { useEffect, useState } from "react";
import init, { World, } from "rustlib";
import TileMap from "./components/TileMap";

let height = 100;
let width = 100;

export default function App(): React.JSX.Element {

  const [world, setWorld] = useState<World>();

  useEffect(() => {
    init().then(() => {
      const w = new World;
      w.build_map(height, width, 539483);
      w.set_entities();
      w.sort_entities();
      setWorld(w);
    })
  }, [])


  return world ? (
    <>
      <div style={{ backgroundColor: "red" }}>
        henlo testing testing
      </div>
      <div key="tilemap" id="tilemap">
        <TileMap world={world} width={width} />
      </div>
    </>
  ) : (
    <div style={{ backgroundColor: "blue" }}>
      LOADING
    </div>
  )
}
