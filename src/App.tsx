import { useEffect, useState } from "react";
import init, { World, Tile, } from "rustlib";

export default function App() {

  const [map, setMap] = useState<Tile[]>([]);
  const [loading, setLoading] = useState<boolean>(false);

  let dim = 100;


  useEffect(() => {
    setLoading(true)
    init().then(() => {
      let w = (World.build_map(dim * 1.5, dim).render()) as Tile[];
      setMap(w);
    })
    setLoading(false)
    console.log(map)
  }, []);
  
  return loading ? <span>uhh</span> : (
    <>
      <p>
        {
          map.map((i, index) => {
            if ((index + 1) % (dim * 1.5) === 0) {
              return (
              <>
                <span key={"f"+index+i.val} style={{"color": `#${((index % dim) * 9) + 100}`}}>
                  {i.char}
                </span>
                <br />
              </>
              )
            } else {
              return (
                <>
                  <span key={"f"+index+i.val} style={{"color": `#${((index % dim) * 9) + 100}`}}>
                    {i.char}
                  </span>
                </>
                )
            }
          })
        }
      </p>
    </>
  )
}
