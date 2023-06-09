import { Entity, Tile, } from "rustlib";
import TileRow from "./TileRow";

type TileMapProps = {
  tiles: [[x: number, y: number], Tile][],
  entities: Entity[],
  width: number
  player: Entity
}

export default function TileMap(props: TileMapProps): React.JSX.Element {
  // this can probably be done faster... Rust?
  const splitIntoRows = (arr: [[x: number, y: number], Tile][]): [[x: number, y: number], Tile][][] => {
    let row: [[x: number, y: number], Tile][][] = [];
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
          let player = props.player.location.y === index ? props.player : false;
          return (
            <div key={"row-" + index} id={"row-" + index}>
              <TileRow row={i} tileIndex={index} player={player} entity={entitiesForThisRow} />
            </div>
          )
        })
      }
    </>
  )
}
