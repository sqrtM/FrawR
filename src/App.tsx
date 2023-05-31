import init, { World } from "rustlib";

export default function App() {

  let map;
  init().then(() => {
   let w = (World.build_map());
   console.log(w.render());
  })
  return (
    <>
      <h1>Vite + React</h1>
      <div className="card">
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}
