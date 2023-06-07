import { Entity, Tile, } from "rustlib";
import TileRow from "./TileRow";

type TileMapProps = {
  tiles: Tile[],
  entities: Entity[],
  width: number
}

export default function TileMap(props: TileMapProps): React.JSX.Element {
  // this can probably be done faster... Rust?
  const splitIntoRows = (arr: Tile[]): Tile[][] => {
    let row: Tile[][] = [];
    for (let i = 0; i < arr.length && arr.length > 0; i += props.width) {
      row.push(arr.slice(i, i + props.width))
    }
    return row;
  }

  return (
    <>
      {
        splitIntoRows(props.tiles).map((i, index) => {
          let entitiesForThisRow = props.entities.filter(i => i.location.y === index);
          return (
            <div key={"row-" + index} id={"row-" + index}>
              <TileRow row={i} tileIndex={index} entities={entitiesForThisRow} />
            </div>
          )
        })
      }
    </>
  )
}
